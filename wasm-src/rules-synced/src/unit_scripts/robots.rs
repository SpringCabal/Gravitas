use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{
    AngularSpeed, Axis, Duration, InitCtx, Piece, SfxFlags, TaskDefinition, UnitCtx,
};

use super::common::{SIG_AIM, SIG_ANIMATION, explode, move_piece, turn};

#[derive(Clone, Copy)]
pub(super) struct RobotPieces {
    pub center: Piece,
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
    pub blade_up_01: Piece,
    pub blade_up_02: Piece,
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
            blade_up_01: p("bladeup01"),
            blade_up_02: p("bladeup02"),
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

pub(super) struct RobotAnimationState {
    pub pieces: RobotPieces,
    pub kind: RobotKind,
    pub moving: bool,
    pub aiming: bool,
}

pub(super) fn create(
    ctx: &UnitCtx,
    pieces: RobotPieces,
    kind: RobotKind,
    state: Rc<RefCell<RobotAnimationState>>,
) {
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
        if piece != Piece::INVALID {
            ctx.hide(piece);
        }
    }
    match kind {
        RobotKind::Bob => {
            for piece in [pieces.mg_01, pieces.mg_02, pieces.mg_b_01, pieces.mg_b_02] {
                if piece != Piece::INVALID {
                    ctx.show(piece);
                }
            }
        }
        RobotKind::Wallier => {
            for piece in [pieces.rocket_01, pieces.rocket_02] {
                if piece != Piece::INVALID {
                    ctx.show(piece);
                }
            }
        }
        RobotKind::Projector => {
            for piece in [pieces.turret, pieces.gun_01] {
                if piece != Piece::INVALID {
                    ctx.show(piece);
                }
            }
        }
        RobotKind::Boss => {
            if pieces.turret != Piece::INVALID {
                ctx.show(pieces.turret);
            }
        }
    }
    ctx.signal(SIG_ANIMATION);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-robot-animation",
        state,
        animate,
    ));
}

pub(super) fn start_moving(ctx: &UnitCtx, state: Rc<RefCell<RobotAnimationState>>) {
    state.borrow_mut().moving = true;
    ctx.signal(SIG_ANIMATION);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-robot-animation",
        state,
        animate,
    ));
}

pub(super) fn stop_moving(ctx: &UnitCtx, state: Rc<RefCell<RobotAnimationState>>) {
    state.borrow_mut().moving = false;
    ctx.signal(SIG_ANIMATION);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-robot-animation",
        state,
        animate,
    ));
}

pub(super) fn aim(
    ctx: &UnitCtx,
    state: Rc<RefCell<RobotAnimationState>>,
    weapon: i32,
    heading: f32,
    pitch: f32,
) -> bool {
    let mut state_ref = state.borrow_mut();
    state_ref.aiming = true;
    let pieces = state_ref.pieces;
    match state_ref.kind {
        RobotKind::Boss => {
            turn(ctx, pieces.arm_up_01, Axis::X, pitch, 19.0);
            turn(ctx, pieces.arm_up_02, Axis::X, pitch, 19.0);
            turn(ctx, pieces.turret, Axis::Y, -heading, 19.0);
            true
        }
        RobotKind::Wallier => {
            turn(ctx, pieces.turret, Axis::Y, heading, 19.0);
            true
        }
        RobotKind::Projector => {
            turn(ctx, pieces.arm_up_01, Axis::X, pitch, 19.0);
            turn(ctx, pieces.arm_up_02, Axis::X, pitch, 19.0);
            turn(ctx, pieces.turret, Axis::Y, heading, 19.0);
            true
        }
        RobotKind::Bob => {
            turn(ctx, pieces.turret, Axis::Y, heading, 19.0);
            ctx.spin(
                pieces.mg_b_01,
                Axis::Z,
                AngularSpeed::radians_per_second(5.0),
                AngularSpeed::radians_per_second(5.0),
            );
            ctx.spin(
                pieces.mg_b_02,
                Axis::Z,
                AngularSpeed::radians_per_second(5.0),
                AngularSpeed::radians_per_second(5.0),
            );
            let _ = weapon;
            true
        }
    }
}

pub(super) fn fire(ctx: &UnitCtx, state: Rc<RefCell<RobotAnimationState>>) {
    let mut state_ref = state.borrow_mut();
    state_ref.aiming = false;
    let pieces = state_ref.pieces;
    match state_ref.kind {
        RobotKind::Boss | RobotKind::Projector | RobotKind::Wallier => {
            turn(ctx, pieces.arm_up_01, Axis::X, -2.0, 5.0);
            turn(ctx, pieces.arm_up_02, Axis::X, -2.0, 5.0);
        }
        RobotKind::Bob => {
            ctx.stop_spin(
                pieces.mg_b_01,
                Axis::Z,
                AngularSpeed::radians_per_second(0.15),
            );
            ctx.stop_spin(
                pieces.mg_b_02,
                Axis::Z,
                AngularSpeed::radians_per_second(0.15),
            );
        }
    }
    let _ = ctx.spawn(TaskDefinition::new("gravitas-robot-fire-reset", reset_fire));
}

pub(super) fn aim_from(state: &RobotAnimationState, weapon: i32, next_gun: usize) -> Piece {
    match state.kind {
        RobotKind::Boss => state.pieces.turret,
        RobotKind::Wallier => state.pieces.rocket_01,
        RobotKind::Projector => match next_gun % 4 {
            0 => state.pieces.gun_01,
            1 => state.pieces.gun_02,
            2 => state.pieces.gun_03,
            _ => state.pieces.gun_04,
        },
        RobotKind::Bob => {
            if weapon == 0 {
                state.pieces.mg_01
            } else {
                state.pieces.mg_02
            }
        }
    }
}

pub(super) fn query_weapon(state: &RobotAnimationState, weapon: i32, next_gun: usize) -> Piece {
    match state.kind {
        RobotKind::Boss => state.pieces.turret,
        RobotKind::Wallier => state.pieces.rocket_01,
        RobotKind::Projector => match next_gun % 4 {
            0 => state.pieces.gun_01,
            1 => state.pieces.gun_02,
            2 => state.pieces.gun_03,
            _ => state.pieces.gun_04,
        },
        RobotKind::Bob => {
            if weapon == 0 {
                state.pieces.mg_01
            } else {
                state.pieces.mg_02
            }
        }
    }
}

pub(super) fn killed(ctx: &UnitCtx, state: &RobotAnimationState) -> spring::cus::WreckLevel {
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

async fn animate(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    let mut phase = false;
    loop {
        let (moving, aiming, pieces, kind) = {
            let state = state.borrow();
            (state.moving, state.aiming, state.pieces, state.kind)
        };
        let sign = if phase { 1.0 } else { -1.0 };
        if moving {
            robot_leg_pose(&ctx, pieces, kind, sign, 5.0);
        } else {
            robot_idle_pose(&ctx, pieces, kind, sign);
            move_piece(
                &ctx,
                pieces.center,
                Axis::Y,
                if phase { 0.0 } else { -5.0 },
                2.0,
            );
        }
        if !aiming {
            turn(
                &ctx,
                pieces.arm_up_01,
                Axis::X,
                if phase { 0.26 } else { 0.09 },
                0.25,
            );
            turn(
                &ctx,
                pieces.arm_up_02,
                Axis::X,
                if phase { 0.26 } else { 0.09 },
                0.25,
            );
        }
        ctx.sleep(Duration::from_millis(if moving { 100 } else { 200 }))
            .await;
        phase = !phase;
    }
}

async fn reset_fire(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_AIM);
    ctx.sleep(Duration::from_millis(1000)).await;
    let pieces = super::common::unit_piece(ctx.unit(), "turret");
    turn(&ctx, pieces, Axis::Y, 0.0, 5.0);
}

fn robot_leg_pose(ctx: &UnitCtx, p: RobotPieces, kind: RobotKind, sign: f32, speed: f32) {
    turn(ctx, p.leg_front_01, Axis::X, -0.26 * sign, speed);
    turn(ctx, p.fr_blade_01, Axis::X, 1.12 * sign, speed);
    turn(ctx, p.leg_rear_02, Axis::X, -1.03 * sign, speed);
    turn(ctx, p.r_blade_02, Axis::X, 1.55 * sign, speed);
    turn(ctx, p.leg_front_02, Axis::X, 0.26 * sign, speed);
    turn(ctx, p.fr_blade_02, Axis::X, -1.12 * sign, speed);
    turn(ctx, p.leg_rear_01, Axis::X, 1.03 * sign, speed);
    turn(ctx, p.r_blade_01, Axis::X, -1.55 * sign, speed);
    turn(ctx, p.blade_up_01, Axis::X, 0.2 * sign, speed);
    turn(ctx, p.blade_up_02, Axis::X, 0.2 * sign, speed);
    if kind == RobotKind::Boss {
        turn(ctx, p.leg_rear_03, Axis::X, 1.03 * sign, speed);
        turn(ctx, p.leg_rear_04, Axis::X, -1.03 * sign, speed);
        turn(ctx, p.r_blade_03, Axis::X, -1.55 * sign, speed);
        turn(ctx, p.r_blade_04, Axis::X, 1.55 * sign, speed);
    }
}

fn robot_idle_pose(ctx: &UnitCtx, p: RobotPieces, kind: RobotKind, sign: f32) {
    robot_leg_pose(ctx, p, kind, sign * 0.25, 0.25);
}
