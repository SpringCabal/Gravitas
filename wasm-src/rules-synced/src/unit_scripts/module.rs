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
    /// Attachments and calls made while the CUS state was in use further up
    /// the stack (see [`attach`]), applied on the next Core-CUS tick. This is
    /// the CUS module's own state, not a gadget's.
    static DEFERRED_UNITS: RefCell<Vec<(i32, i32)>> = const { RefCell::new(Vec::new()) };
    static DEFERRED_CALLS: RefCell<Vec<(i32, &'static str)>> = const { RefCell::new(Vec::new()) };
}

/// Attach the unit's script, if its def has one.
///
/// The rules run in this module, so they reach the CUS state directly. That
/// fails only while the state is in use further up the stack (a unit created
/// from inside a unit script) or before the engine has initialized it (units
/// that exist when the rules load); those units are attached on the next tick.
pub fn attach(unit: i32, def: i32) {
    if unit >= 0 && crate::with_cus_module(|module| module.attach(unit, def)).is_none() {
        DEFERRED_UNITS.with(|deferred| deferred.borrow_mut().push((unit, def)));
    }
}

/// Run a named unit-script function, like Lua's `env.Function()`. Deferred to
/// the next tick in the same cases as [`attach`].
pub fn call(unit: i32, function_name: &'static str) {
    if unit >= 0 && crate::with_cus_module(|module| module.call(unit, function_name)).is_none() {
        DEFERRED_CALLS.with(|deferred| deferred.borrow_mut().push((unit, function_name)));
    }
}

#[derive(Default)]
pub struct GravitasCusModule {
    registry: CusRegistry<GravitasUnitScript>,
    handles: BTreeMap<u32, CusHandle>,
}

impl GravitasCusModule {
    fn attach(&mut self, unit_id: i32, def: i32) {
        let Some(kind) = crate::defs::get().unit(def).script else {
            return;
        };
        let unit = UnitId(unit_id);
        let instance = instance_id(unit);
        if self.handles.contains_key(&instance) {
            return;
        }
        let map = match spring::get_unit_piece_map(unit_id) {
            Ok(map) => map,
            Err(error) => {
                crate::diagnostics::warning(&alloc::format!(
                    "no unit script for unit {unit_id}: piece map failed: {error:?}"
                ));
                return;
            }
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
        match WasmCus::attach(unit, instance, capabilities(kind)) {
            Ok(_) => {
                self.handles.insert(instance, handle);
            }
            Err(error) => {
                crate::diagnostics::warning(&alloc::format!(
                    "no unit script for unit {unit_id}: attach failed: {error:?}"
                ));
                let _ = self.registry.detach(handle);
            }
        }
    }

    fn call(&mut self, unit_id: i32, function_name: &str) {
        let Some(handle) = self.handles.get(&instance_id(UnitId(unit_id))).copied() else {
            return;
        };
        self.sync_instance_frame(handle);
        let _ = self.registry.with(handle, |instance| {
            let ctx = instance.context();
            instance.with_state(|script| script.named_call(&ctx, function_name))
        });
    }

    fn apply_deferred(&mut self) {
        let units = DEFERRED_UNITS.with(|deferred| core::mem::take(&mut *deferred.borrow_mut()));
        for (unit_id, def) in units {
            // A unit that died meanwhile may have handed its id to a new one.
            if crate::units::def_of(unit_id) == Some(def) {
                self.attach(unit_id, def);
            }
        }
        let calls = DEFERRED_CALLS.with(|deferred| core::mem::take(&mut *deferred.borrow_mut()));
        for (unit_id, function_name) in calls {
            self.call(unit_id, function_name);
        }
    }

    fn sync_instance_frame(&mut self, handle: CusHandle) {
        let _ = self.registry.with(handle, |instance| {
            instance.tick(spring::game::current_frame() as u64)
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
        self.apply_deferred();
        self.registry.tick(frame as u64);
    }

    fn cus_detach(&mut self, instance_id: u32) {
        if let Some(handle) = self.handles.remove(&instance_id) {
            let _ = self.registry.detach(handle);
        }
    }
}
