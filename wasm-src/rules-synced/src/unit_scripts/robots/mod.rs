use alloc::rc::Rc;
use core::cell::RefCell;
use core::f32::consts::PI;

use spring::cus::{
    AngularSpeed, Axis, Duration, InitCtx, Piece, SfxFlags, TaskDefinition, UnitCtx, WeaponId,
};

mod gait;
mod stun;

use super::common::{SIG_AIM, SIG_ANIMATION, explode, turn};
use gait::{animate, wait_for_turn};

pub(super) const DEGREES: f32 = PI / 180.0;

#[derive(Clone, Copy)]
struct RobotPieces {
    pub center: Piece,
    pub body: Piece,
    pub blade_up_01: Piece,
    pub blade_up_02: Piece,
    pub leg_front_01: Piece,
    pub leg_front_02: Piece,
    pub leg_rear_01: Piece,
    pub leg_rear_02: Piece,
    pub leg_rear_03: Piece,
    pub leg_rear_04: Piece,
    pub fr_blade_01: Piece,
    pub fr_blade_02: Piece,
    pub r_blade_01: Piece,
    pub r_blade_02: Piece,
    pub r_blade_03: Piece,
    pub r_blade_04: Piece,
    pub arm_up_01: Piece,
    pub arm_up_02: Piece,
    pub turret: Piece,
    pub gun_01: Piece,
    pub gun_02: Piece,
    pub gun_03: Piece,
    pub gun_04: Piece,
    pub rocket_01: Piece,
    pub rocket_02: Piece,
    pub mg_01: Piece,
    pub mg_02: Piece,
    pub mg_b_01: Piece,
    pub mg_b_02: Piece,
}

impl RobotPieces {
    pub(super) fn from_init(ctx: &InitCtx<'_>) -> Self {
        let p = |name: &str| ctx.piece(name);
        Self {
            center: p("center"),
            body: p("Body"),
            blade_up_01: p("bladeup01"),
            blade_up_02: p("bladeup02"),
            leg_front_01: p("legfront01"),
            leg_front_02: p("legfront02"),
            leg_rear_01: p("legrear01"),
            leg_rear_02: p("legrear02"),
            leg_rear_03: p("legrear03"),
            leg_rear_04: p("legrear04"),
            fr_blade_01: p("frblade01"),
            fr_blade_02: p("frblade02"),
            r_blade_01: p("rblade01"),
            r_blade_02: p("rblade02"),
            r_blade_03: p("rblade03"),
            r_blade_04: p("rblade04"),
            arm_up_01: p("armup01"),
            arm_up_02: p("armup02"),
            turret: p("turret"),
            gun_01: p("gun01"),
            gun_02: p("gun02"),
            gun_03: p("gun03"),
            gun_04: p("gun04"),
            rocket_01: p("rocket01"),
            rocket_02: p("rocket02"),
            mg_01: p("MG01"),
            mg_02: p("MG02"),
            mg_b_01: p("MG_B01"),
            mg_b_02: p("MG_B02"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum RobotKind {
    Bob,
    Wallier,
    Projector,
    Boss,
}

struct RobotAnimationState {
    pub pieces: RobotPieces,
    pub kind: RobotKind,
    pub started: bool,
    pub moving: bool,
    /// Paralyzed: the walk and idle cycles give way to the stun spasm.
    pub stunned: bool,
    pub aiming: bool,
    pub explicit_target: bool,
    pub last_heading: f32,
    pub last_pitch: f32,
}

/// projector.lua (B.O.B, Wallier, Projector) and projector_boss.lua.
pub(super) struct RobotScript {
    state: Rc<RefCell<RobotAnimationState>>,
    next_gun: usize,
}

impl RobotScript {
    pub(super) fn new(ctx: &InitCtx<'_>, kind: RobotKind) -> Self {
        Self {
            state: Rc::new(RefCell::new(RobotAnimationState {
                pieces: RobotPieces::from_init(ctx),
                kind,
                started: false,
                moving: false,
                stunned: false,
                aiming: false,
                explicit_target: false,
                last_heading: 0.0,
                last_pitch: 0.0,
            })),
            next_gun: 0,
        }
    }

    pub(super) fn create(&self, ctx: &UnitCtx) {
        let (pieces, kind) = {
            let state = self.state.borrow();
            (state.pieces, state.kind)
        };
        create(ctx, pieces, kind, Rc::clone(&self.state));
    }

    pub(super) fn start_moving(&self) {
        let mut state = self.state.borrow_mut();
        state.started = true;
        state.moving = true;
    }

    pub(super) fn stop_moving(&self) {
        let mut state = self.state.borrow_mut();
        state.started = true;
        state.moving = false;
    }

    pub(super) fn killed(&self, ctx: &UnitCtx) -> spring::cus::WreckLevel {
        killed(ctx, &self.state.borrow())
    }

    pub(super) fn query_weapon(&mut self, weapon: WeaponId) -> Piece {
        let piece = query_weapon(&self.state.borrow(), weapon.0, self.next_gun);
        self.next_gun = self.next_gun.wrapping_add(1);
        piece
    }

    pub(super) fn aim_from_weapon(&self, weapon: WeaponId) -> Piece {
        aim_from(&self.state.borrow(), weapon.0, self.next_gun)
    }

    pub(super) fn aim_weapon(
        &self,
        ctx: &UnitCtx,
        weapon: WeaponId,
        heading: f32,
        pitch: f32,
    ) -> Option<bool> {
        aim(ctx, Rc::clone(&self.state), weapon.0, heading, pitch)
    }

    pub(super) fn fire_weapon(&self, ctx: &UnitCtx) {
        fire(ctx, Rc::clone(&self.state));
    }

    pub(super) fn build_piece(&self) -> Piece {
        self.state.borrow().pieces.center
    }
}

fn create(
    ctx: &UnitCtx,
    pieces: RobotPieces,
    kind: RobotKind,
    state: Rc<RefCell<RobotAnimationState>>,
) {
    match kind {
        RobotKind::Boss => {
            // projector_boss.lua hides only these three pieces.  In particular,
            // it leaves the turret and gun04 at their model defaults.
            hide(ctx, pieces.gun_01);
            hide(ctx, pieces.gun_02);
            hide(ctx, pieces.gun_03);
        }
        RobotKind::Bob | RobotKind::Wallier | RobotKind::Projector => {
            for piece in [
                pieces.mg_01,
                pieces.mg_02,
                pieces.mg_b_01,
                pieces.mg_b_02,
                pieces.rocket_01,
                pieces.rocket_02,
                pieces.turret,
                pieces.gun_01,
                pieces.gun_02,
                pieces.gun_03,
                pieces.gun_04,
            ] {
                hide(ctx, piece);
            }
            match kind {
                RobotKind::Bob => {
                    for piece in [pieces.mg_01, pieces.mg_02, pieces.mg_b_01, pieces.mg_b_02] {
                        show(ctx, piece);
                    }
                }
                RobotKind::Wallier => {
                    show(ctx, pieces.rocket_01);
                    show(ctx, pieces.rocket_02);
                }
                RobotKind::Projector => {
                    show(ctx, pieces.turret);
                    show(ctx, pieces.gun_01);
                }
                RobotKind::Boss => unreachable!(),
            }
        }
    }

    ctx.signal(SIG_ANIMATION);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-robot-animation",
        Rc::clone(&state),
        animate,
    ));
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-robot-stun-detect",
        Rc::clone(&state),
        stun::detect,
    ));
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-robot-target-watch",
        state,
        watch_explicit_target,
    ));
}

fn aim(
    ctx: &UnitCtx,
    state: Rc<RefCell<RobotAnimationState>>,
    _weapon: i32,
    heading: f32,
    pitch: f32,
) -> Option<bool> {
    let explicit_target = has_explicit_target(ctx.unit());
    {
        let mut state_ref = state.borrow_mut();
        state_ref.last_heading = heading;
        state_ref.last_pitch = pitch;
        if explicit_target {
            state_ref.explicit_target = true;
        }
    }

    // The engine CUS call-in also asks the script to aim during its
    // movement-target pass.  The direct movement task owns that transition;
    // defer those synthetic aim requests while the walk task is active.
    if state.borrow().moving && !explicit_target {
        return Some(false);
    }
    apply_aim(ctx, state);
    Some(true)
}

fn apply_aim(ctx: &UnitCtx, state: Rc<RefCell<RobotAnimationState>>) {
    let (kind, p, heading, pitch) = {
        let mut state_ref = state.borrow_mut();
        state_ref.aiming = true;
        (
            state_ref.kind,
            state_ref.pieces,
            state_ref.last_heading,
            state_ref.last_pitch,
        )
    };

    // projector.lua uses SIG_RESET for the delayed turret reset.  The direct
    // CUS task uses SIG_AIM for that same reset, so cancel it when a new aim
    // starts just as the Lua call-in does.
    ctx.signal(SIG_AIM);
    match kind {
        RobotKind::Wallier => {
            turn(ctx, p.turret, Axis::Y, heading, 19.0);
            // Wallier's AimWeapon1 also starts ReloadMissiles: after 50 ms it
            // returns both arm pieces to zero.  This is observable through
            // their rocket/MG child matrices, even though the turret itself
            // is the only piece turned by the call-in.
            ctx.spawn(TaskDefinition::with_state(
                "gravitas-wallier-reload-missiles",
                Rc::clone(&state),
                reload_missiles,
            ));
        }
        RobotKind::Projector => {
            turn(ctx, p.arm_up_01, Axis::X, pitch, 19.0);
            turn(ctx, p.turret, Axis::Y, heading, 19.0);
            turn(ctx, p.arm_up_02, Axis::X, pitch, 19.0);
        }
        RobotKind::Bob => {
            turn(ctx, p.turret, Axis::Y, heading, 19.0);
            spin(ctx, p.mg_b_01);
            spin(ctx, p.mg_b_02);
        }
        RobotKind::Boss => {
            turn(ctx, p.arm_up_01, Axis::X, pitch, 19.0);
            turn(ctx, p.turret, Axis::Y, -heading, 19.0);
            turn(ctx, p.arm_up_02, Axis::X, pitch, 19.0);
        }
    }
}

async fn watch_explicit_target(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    loop {
        let explicit_target = has_explicit_target(ctx.unit());
        let should_apply = {
            let mut state_ref = state.borrow_mut();
            if explicit_target {
                if state_ref.explicit_target {
                    false
                } else {
                    state_ref.explicit_target = true;
                    true
                }
            } else {
                state_ref.explicit_target = false;
                false
            }
        };
        if should_apply {
            apply_aim(&ctx, Rc::clone(&state));
        }
        ctx.next_frame().await;
    }
}

fn has_explicit_target(unit: spring::UnitId) -> bool {
    spring::get_unit_rules_param_f32(unit.0, "targetID")
        .ok()
        .is_some_and(|target| target >= 0.0)
        || (spring::get_unit_rules_param_f32(unit.0, "targetCoordX")
            .ok()
            .is_some_and(|target| target != -1.0)
            && spring::get_unit_rules_param_f32(unit.0, "targetCoordZ")
                .ok()
                .is_some_and(|target| target != -1.0))
}

async fn reload_missiles(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    ctx.sleep(Duration::from_millis(50)).await;
    let p = state.borrow().pieces;
    turn(&ctx, p.arm_up_01, Axis::X, 0.0, 0.1);
    turn(&ctx, p.arm_up_02, Axis::X, 0.0, 0.1);
    wait_for_turn(&ctx, p.arm_up_02).await;
}

fn fire(ctx: &UnitCtx, state: Rc<RefCell<RobotAnimationState>>) {
    let mut state_ref = state.borrow_mut();
    state_ref.aiming = false;
    let p = state_ref.pieces;
    let kind = state_ref.kind;
    drop(state_ref);

    match kind {
        // projector.lua's Wallier branch shares the generic FireWeapon1 and
        // therefore also kicks both arm pieces down.
        RobotKind::Wallier => {
            turn(ctx, p.arm_up_01, Axis::X, -2.0, 5.0);
            turn(ctx, p.arm_up_02, Axis::X, -2.0, 5.0);
        }
        RobotKind::Bob => {
            stop_spin(ctx, p.mg_b_01);
            stop_spin(ctx, p.mg_b_02);
        }
        RobotKind::Projector | RobotKind::Boss => {}
    }

    ctx.signal(SIG_AIM);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-robot-fire-reset",
        state,
        reset_turret,
    ));
}

fn aim_from(state: &RobotAnimationState, _weapon: i32, next_gun: usize) -> Piece {
    match state.kind {
        RobotKind::Boss | RobotKind::Wallier => {
            if state.kind == RobotKind::Boss {
                state.pieces.turret
            } else {
                state.pieces.rocket_01
            }
        }
        RobotKind::Projector => match next_gun % 4 {
            0 => state.pieces.gun_01,
            1 => state.pieces.gun_02,
            2 => state.pieces.gun_03,
            _ => state.pieces.gun_04,
        },
        RobotKind::Bob => {
            if next_gun.is_multiple_of(2) {
                state.pieces.mg_01
            } else {
                state.pieces.mg_02
            }
        }
    }
}

fn query_weapon(state: &RobotAnimationState, _weapon: i32, next_gun: usize) -> Piece {
    match state.kind {
        RobotKind::Boss => state.pieces.turret,
        RobotKind::Wallier => state.pieces.rocket_01,
        // QueryWeapon2 rotates before returning.  next_gun is incremented by
        // the caller after this function, so the first query returns gun02.
        RobotKind::Projector => match (next_gun + 1) % 4 {
            0 => state.pieces.gun_01,
            1 => state.pieces.gun_02,
            2 => state.pieces.gun_03,
            _ => state.pieces.gun_04,
        },
        // QueryWeapon3 likewise returns MG02 on its first call.
        RobotKind::Bob => {
            if next_gun.is_multiple_of(2) {
                state.pieces.mg_02
            } else {
                state.pieces.mg_01
            }
        }
    }
}

fn killed(ctx: &UnitCtx, state: &RobotAnimationState) -> spring::cus::WreckLevel {
    ctx.signal(SIG_ANIMATION);
    let p = state.pieces;
    for piece in [
        p.leg_front_01,
        p.leg_front_02,
        p.fr_blade_01,
        p.fr_blade_02,
        p.leg_rear_01,
        p.leg_rear_02,
        p.r_blade_01,
        p.r_blade_02,
        p.center,
    ] {
        explode(ctx, piece, SfxFlags::EXPLODE);
    }
    spring::cus::WreckLevel::One
}

async fn reset_turret(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_AIM);
    ctx.sleep(Duration::from_millis(1000)).await;
    let turret = state.borrow().pieces.turret;
    turn(&ctx, turret, Axis::Y, 0.0, 5.0);
}

fn hide(ctx: &UnitCtx, piece: Piece) {
    if piece != Piece::INVALID {
        ctx.hide(piece);
    }
}

fn show(ctx: &UnitCtx, piece: Piece) {
    if piece != Piece::INVALID {
        ctx.show(piece);
    }
}

fn spin(ctx: &UnitCtx, piece: Piece) {
    if piece != Piece::INVALID {
        ctx.spin(
            piece,
            Axis::Z,
            AngularSpeed::radians_per_second(5.0),
            AngularSpeed::radians_per_second(5.0),
        );
    }
}

fn stop_spin(ctx: &UnitCtx, piece: Piece) {
    if piece != Piece::INVALID {
        ctx.stop_spin(piece, Axis::Z, AngularSpeed::radians_per_second(0.15));
    }
}
