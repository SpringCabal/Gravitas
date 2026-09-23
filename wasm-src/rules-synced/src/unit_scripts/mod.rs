//! Gravitas unit scripts implemented through the engine's portable Core-CUS API.
//!
//! The Lua game used three families of scripts: small scenery scripts,
//! Cassos robots, and Gravit.  The registry below keeps those families in one
//! guest module while preserving the engine-facing CUnitScript callbacks.

mod animations;
mod common;
mod immobile;
mod robots;

use alloc::collections::BTreeMap;
use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use spring::UnitId;
use spring::cus::core_module::{CoreCusCallResult, CoreCusModule};
use spring::cus::wasm::WasmCus;
use spring::cus::{
    CusHandle, CusInstance, CusRegistry, InitCtx, Piece, ScriptCapabilities, SfxFlags, UnitCtx,
    UnitScript, UnitScriptCall, WeaponId, WreckLevel,
};

use self::animations::{GravitAnimationState, GravitPieces};
use self::common::{EnginePieces, SIG_AIM, SIG_ANIMATION, SIG_STOP, explode, play_unit_sound};
use self::robots::{RobotAnimationState, RobotKind, RobotPieces};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptKind {
    Barrel,
    Canister,
    Crate,
    Gate,
    Electrafi,
    Fire,
    Plate,
    Wall1,
    Wall2,
    Wall3,
    Wall4,
    Gravit,
    Bob,
    Wallier,
    Projector,
    Boss,
}

impl ScriptKind {
    /// The script a unit def uses, by def name.
    pub fn for_def_name(name: &str) -> Option<Self> {
        Some(match name {
            "barrel" => Self::Barrel,
            "canister" => Self::Canister,
            "crate" => Self::Crate,
            "gate" => Self::Gate,
            "electrafi" => Self::Electrafi,
            "fire" => Self::Fire,
            "plate" => Self::Plate,
            "wall1" => Self::Wall1,
            "wall2" => Self::Wall2,
            "wall3" => Self::Wall3,
            "wall4" => Self::Wall4,
            "gravit" => Self::Gravit,
            "bob" | "b.o.b" => Self::Bob,
            "wallier" => Self::Wallier,
            "projector" => Self::Projector,
            "cometopapa" | "come_to_papa" => Self::Boss,
            _ => return None,
        })
    }
}

fn capabilities(kind: ScriptKind) -> ScriptCapabilities {
    let mut bits = ScriptCapabilities::KILLED;
    match kind {
        ScriptKind::Gravit => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::START_MOVING
                | ScriptCapabilities::STOP_MOVING
                | ScriptCapabilities::QUERY_NANO_PIECE
                | ScriptCapabilities::QUERY_BUILD_INFO
                | ScriptCapabilities::FIRE_WEAPON
                | ScriptCapabilities::QUERY_WEAPON
                | ScriptCapabilities::AIM_WEAPON
                | ScriptCapabilities::AIM_FROM_WEAPON;
        }
        ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::START_MOVING
                | ScriptCapabilities::STOP_MOVING
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE
                | ScriptCapabilities::QUERY_NANO_PIECE
                | ScriptCapabilities::QUERY_BUILD_INFO
                | ScriptCapabilities::FIRE_WEAPON
                | ScriptCapabilities::QUERY_WEAPON
                | ScriptCapabilities::AIM_WEAPON
                | ScriptCapabilities::AIM_FROM_WEAPON;
        }
        ScriptKind::Gate | ScriptKind::Plate => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE;
        }
        _ => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE;
        }
    }
    ScriptCapabilities::new(bits)
}

/// Core-CUS instance ids are the unit id offset by a per-module tag ("GR").
const INSTANCE_ID_BASE: u32 = 0x4752_0000;

fn instance_id(unit: UnitId) -> u32 {
    INSTANCE_ID_BASE.wrapping_add(unit.0 as u32)
}

pub struct GravitasUnitScript {
    kind: Option<ScriptKind>,
    gravit_pieces: GravitPieces,
    gravit_state: Rc<RefCell<GravitAnimationState>>,
    robot_pieces: RobotPieces,
    robot_state: Rc<RefCell<RobotAnimationState>>,
    next_gun: usize,
}

impl UnitScript for GravitasUnitScript {
    fn new(ctx: &mut InitCtx<'_>) -> Self {
        let gravit_pieces = GravitPieces::from_init(ctx);
        let robot_pieces = RobotPieces::from_init(ctx);
        Self {
            kind: None,
            gravit_state: Rc::new(RefCell::new(GravitAnimationState {
                active: false,
                jumping: false,
                pieces: gravit_pieces,
            })),
            robot_state: Rc::new(RefCell::new(RobotAnimationState {
                pieces: robot_pieces,
                kind: RobotKind::Projector,
                started: false,
                moving: false,
                aiming: false,
                explicit_target: false,
                last_heading: 0.0,
                last_pitch: 0.0,
            })),
            gravit_pieces,
            robot_pieces,
            next_gun: 0,
        }
    }

    fn create(&mut self, ctx: &UnitCtx) {
        match self.kind {
            Some(ScriptKind::Gravit) => {
                animations::create(ctx, self.gravit_pieces, Rc::clone(&self.gravit_state));
            }
            Some(
                kind @ (ScriptKind::Bob
                | ScriptKind::Wallier
                | ScriptKind::Projector
                | ScriptKind::Boss),
            ) => {
                let robot_kind = robot_kind(kind).expect("robot script kind");
                self.robot_state.borrow_mut().kind = robot_kind;
                robots::create(
                    ctx,
                    self.robot_pieces,
                    robot_kind,
                    Rc::clone(&self.robot_state),
                );
            }
            _ => {}
        }
    }

    fn start_moving(&mut self, ctx: &UnitCtx, _reversing: bool) {
        match self.kind {
            Some(ScriptKind::Gravit) => {
                animations::start_moving(ctx, self.gravit_pieces, Rc::clone(&self.gravit_state));
            }
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => {
                robots::start_moving(Rc::clone(&self.robot_state));
            }
            _ => {}
        }
    }

    fn stop_moving(&mut self, ctx: &UnitCtx) {
        match self.kind {
            Some(ScriptKind::Gravit) => {
                animations::stop_moving(ctx, self.gravit_pieces, Rc::clone(&self.gravit_state));
            }
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => {
                robots::stop_moving(Rc::clone(&self.robot_state));
            }
            _ => {}
        }
    }

    fn activate(&mut self, ctx: &UnitCtx) {
        match self.kind {
            Some(ScriptKind::Gravit) => animations::activate(&self.gravit_state),
            Some(kind) => immobile::activate(ctx, kind),
            None => {}
        }
    }

    fn deactivate(&mut self, ctx: &UnitCtx) {
        match self.kind {
            Some(ScriptKind::Gravit) => animations::deactivate(&self.gravit_state),
            Some(kind) => immobile::deactivate(ctx, kind),
            None => {}
        }
    }

    fn killed(&mut self, ctx: &UnitCtx, _recent_damage: f32, _max_health: f32) -> WreckLevel {
        match self.kind {
            Some(ScriptKind::Canister) => {
                immobile_explode(ctx, "canister", SfxFlags::SHATTER);
            }
            Some(ScriptKind::Crate) => {
                immobile_explode(ctx, "Cube", SfxFlags::EXPLODE);
            }
            Some(ScriptKind::Wall2) => {
                for name in WALL2_FRAGMENTS {
                    immobile_explode(ctx, name, SfxFlags(SfxFlags::FALL.0 | SfxFlags::EXPLODE.0));
                }
            }
            Some(ScriptKind::Gravit) => {
                ctx.signal(SIG_ANIMATION);
                ctx.signal(SIG_AIM);
                ctx.signal(SIG_STOP);
                play_unit_sound("sounds/death.ogg", 1.0, ctx.unit());
                for piece in [
                    self.gravit_pieces.thigh_left,
                    self.gravit_pieces.thigh_right,
                    self.gravit_pieces.head,
                    self.gravit_pieces.pelvis,
                    self.gravit_pieces.arm_left,
                    self.gravit_pieces.arm_right,
                ] {
                    explode(ctx, piece, SfxFlags::EXPLODE);
                }
            }
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => {
                return robots::killed(ctx, &self.robot_state.borrow());
            }
            _ => {}
        }
        WreckLevel::One
    }

    fn query_weapon(&mut self, _ctx: &UnitCtx, weapon: WeaponId) -> Piece {
        match self.kind {
            Some(ScriptKind::Gravit) => {
                if self.gravit_state.borrow().active {
                    self.gravit_pieces.muzzle_right
                } else {
                    self.gravit_pieces.muzzle_left
                }
            }
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => {
                let piece =
                    robots::query_weapon(&self.robot_state.borrow(), weapon.0, self.next_gun);
                self.next_gun = self.next_gun.wrapping_add(1);
                piece
            }
            _ => Piece::INVALID,
        }
    }

    fn aim_from_weapon(&mut self, _ctx: &UnitCtx, weapon: WeaponId) -> Piece {
        match self.kind {
            Some(ScriptKind::Gravit) => match weapon.0 {
                0 => self.gravit_pieces.arm_right,
                _ => self.gravit_pieces.head,
            },
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => robots::aim_from(&self.robot_state.borrow(), weapon.0, self.next_gun),
            _ => Piece::INVALID,
        }
    }

    fn aim_weapon(
        &mut self,
        ctx: &UnitCtx,
        weapon: WeaponId,
        heading: f32,
        pitch: f32,
    ) -> Option<bool> {
        match self.kind {
            Some(ScriptKind::Gravit) => {
                if !self.gravit_state.borrow().jumping {
                    animations::aim(ctx, self.gravit_pieces, weapon.0 != 0, heading, pitch);
                }
                Some(true)
            }
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => robots::aim(ctx, Rc::clone(&self.robot_state), weapon.0, heading, pitch),
            _ => None,
        }
    }

    fn fire_weapon(&mut self, ctx: &UnitCtx, _weapon: WeaponId) {
        match self.kind {
            Some(ScriptKind::Gravit) => {}
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => {
                robots::fire(ctx, Rc::clone(&self.robot_state));
            }
            _ => {}
        }
    }

    fn query_build_info(&mut self, _ctx: &UnitCtx) -> Piece {
        match self.kind {
            Some(ScriptKind::Gravit) => self.gravit_pieces.head,
            Some(
                ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss,
            ) => self.robot_pieces.center,
            _ => Piece::INVALID,
        }
    }

    fn query_nano_piece(&mut self, ctx: &UnitCtx) -> Piece {
        self.query_build_info(ctx)
    }
}

impl GravitasUnitScript {
    fn set_kind(&mut self, kind: ScriptKind) {
        self.kind = Some(kind);
        if let Some(robot_kind) = robot_kind(kind) {
            self.robot_state.borrow_mut().kind = robot_kind;
        }
    }

    fn named_call(&mut self, ctx: &UnitCtx, name: &str) -> bool {
        if self.kind != Some(ScriptKind::Gravit) {
            return false;
        }
        match name {
            "PreJump" => {
                animations::jump_task(ctx, Rc::clone(&self.gravit_state), "PreJump");
                true
            }
            "StartJump" => {
                self.gravit_state.borrow_mut().jumping = true;
                animations::jump_task(ctx, Rc::clone(&self.gravit_state), "StartJump");
                true
            }
            "HalfJump" => {
                animations::jump_task(ctx, Rc::clone(&self.gravit_state), "HalfJump");
                true
            }
            "StopJump" => {
                self.gravit_state.borrow_mut().jumping = false;
                animations::jump_task(ctx, Rc::clone(&self.gravit_state), "StopJump");
                true
            }
            "Jumping" => true,
            _ => false,
        }
    }
}

fn robot_kind(kind: ScriptKind) -> Option<RobotKind> {
    match kind {
        ScriptKind::Bob => Some(RobotKind::Bob),
        ScriptKind::Wallier => Some(RobotKind::Wallier),
        ScriptKind::Projector => Some(RobotKind::Projector),
        ScriptKind::Boss => Some(RobotKind::Boss),
        _ => None,
    }
}

const WALL2_FRAGMENTS: [&str; 11] = [
    "frag1", "frag2", "frag3", "frag4", "frag5", "frag6", "frag7", "frag8", "frag9", "frag10",
    "frag11",
];

fn immobile_explode(ctx: &UnitCtx, piece: &str, flags: SfxFlags) {
    let piece = common::unit_piece(ctx.unit(), piece);
    explode(ctx, piece, flags);
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
            let _ = self.registry.with(handle, |instance| {
                instance.with_state(|script| script.set_kind(kind));
            });
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
