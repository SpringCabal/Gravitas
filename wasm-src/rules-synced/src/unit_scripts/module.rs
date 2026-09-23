//! The Core-CUS module: attaches scripts to units and routes engine calls to
//! them.

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use spring::UnitId;
use spring::cus::core_module::{CoreCusCallResult, CoreCusModule};
use spring::cus::wasm::WasmCus;
use spring::cus::{CusHandle, CusInstance, CusRegistry, Piece, UnitScriptCall};

use super::common::EnginePieces;
use super::{GravitasUnitScript, capabilities};

/// Core-CUS instance ids are the unit id offset by a per-module tag ("GR").
const INSTANCE_ID_BASE: u32 = 0x4752_0000;

fn instance_id(unit: UnitId) -> u32 {
    INSTANCE_ID_BASE.wrapping_add(unit.0 as u32)
}

thread_local! {
    static PENDING_UNITS: RefCell<Vec<(i32, i32)>> = const { RefCell::new(Vec::new()) };
    static PENDING_CALLS: RefCell<Vec<(i32, &'static str)>> = const { RefCell::new(Vec::new()) };
}

/// Queue a named unit-script call for the next Core-CUS tick.
///
/// The rules run inside this same module, so `call_unit_script` goes through
/// the engine and back into a module that is still inside a gadget callin; the
/// engine rejects that nested call (ApiError 1), so the animation never ran.
/// Calls are applied in order on the tick, like unit attach.
pub fn queue_call(unit: i32, function_name: &'static str) {
    if unit >= 0 {
        PENDING_CALLS.with(|pending| pending.borrow_mut().push((unit, function_name)));
    }
}

/// Queue a unit for script attachment on the next Core-CUS tick, so a gadget
/// callin never attaches a script from inside the module.
pub fn queue_unit_created(unit: i32, def: i32) {
    if unit >= 0 {
        PENDING_UNITS.with(|pending| pending.borrow_mut().push((unit, def)));
    }
}

#[derive(Default)]
pub struct GravitasCusModule {
    registry: CusRegistry<GravitasUnitScript>,
    handles: BTreeMap<u32, CusHandle>,
}

impl GravitasCusModule {
    fn attach_pending(&mut self) {
        let pending = PENDING_UNITS.with(|units| core::mem::take(&mut *units.borrow_mut()));
        for (unit_id, def) in pending {
            let Some(kind) = crate::defs::get().unit(def).script else {
                continue;
            };
            let unit = UnitId(unit_id);
            let instance = instance_id(unit);
            if self.handles.contains_key(&instance) {
                continue;
            }
            let Ok(map) = spring::get_unit_piece_map(unit_id) else {
                continue;
            };
            let root = spring::get_unit_root_piece(unit_id)
                .ok()
                .and_then(|piece| piece.checked_sub(1))
                .map(Piece)
                .unwrap_or(Piece::INVALID);
            let resolver = EnginePieces { map, root };
            let host = WasmCus::new(unit, instance);
            let engine = Rc::new(RefCell::new(host.engine()));
            let instance_state = CusInstance::construct(unit, &resolver, engine);
            let handle = self.registry.attach(instance_state);
            if WasmCus::attach(unit, instance, capabilities(kind)).is_ok() {
                self.handles.insert(instance, handle);
            } else {
                let _ = self.registry.detach(handle);
            }
        }
    }

    fn apply_pending_calls(&mut self) {
        let pending = PENDING_CALLS.with(|calls| core::mem::take(&mut *calls.borrow_mut()));
        for (unit_id, function_name) in pending {
            let Some(handle) = self.handles.get(&instance_id(UnitId(unit_id))).copied() else {
                continue;
            };
            self.sync_instance_frame(handle);
            let _ = self.registry.with(handle, |instance| {
                let ctx = instance.context();
                instance.with_state(|script| script.named_call(&ctx, function_name))
            });
        }
    }

    fn sync_instance_frame(&mut self, handle: CusHandle) {
        let _ = self.registry.with(handle, |instance| {
            instance.tick(crate::units::game_frame() as u64)
        });
    }
}

impl CoreCusModule for GravitasCusModule {
    fn cus_invoke(
        &mut self,
        instance_id: u32,
        call: u32,
        float_arguments: &[f32],
        integer_arguments: &[i32],
        result: &mut CoreCusCallResult<'_>,
    ) -> bool {
        let Some(handle) = self.handles.get(&instance_id).copied() else {
            return false;
        };
        let Some(call) = UnitScriptCall::from_u32(call) else {
            return false;
        };
        if matches!(
            call,
            UnitScriptCall::RawCall | UnitScriptCall::StartMoving | UnitScriptCall::StopMoving
        ) {
            self.sync_instance_frame(handle);
        }
        let mut call_result = spring::cus::UnitScriptCallResult::default();
        let handled = self
            .registry
            .with(handle, |instance| {
                instance.invoke(call, float_arguments, integer_arguments, &mut call_result)
            })
            .unwrap_or(false);
        if !handled {
            return false;
        }
        result.int_value = call_result.int_value;
        result.float_value = call_result.float_value;
        result.bool_value = call_result.bool_value;
        result.complete = call_result.complete;
        result.int_count = call_result.int_values.len().min(result.int_values.len());
        result.int_values[..result.int_count]
            .copy_from_slice(&call_result.int_values[..result.int_count]);
        true
    }

    fn cus_call_named(
        &mut self,
        instance_id: u32,
        function_name: &str,
        _arguments: &[f32],
        _return_values: &mut [f32],
        found: &mut bool,
    ) -> Option<usize> {
        let Some(handle) = self.handles.get(&instance_id).copied() else {
            return Some(0);
        };
        self.sync_instance_frame(handle);
        let handled = self.registry.with(handle, |instance| {
            let ctx = instance.context();
            instance.with_state(|script| script.named_call(&ctx, function_name))
        })?;
        if handled {
            *found = true;
            Some(0)
        } else {
            None
        }
    }

    fn cus_tick(&mut self, frame: u32) {
        self.attach_pending();
        self.apply_pending_calls();
        self.registry.tick(frame as u64);
    }

    fn cus_detach(&mut self, instance_id: u32) {
        if let Some(handle) = self.handles.remove(&instance_id) {
            let _ = self.registry.detach(handle);
        }
    }
}
