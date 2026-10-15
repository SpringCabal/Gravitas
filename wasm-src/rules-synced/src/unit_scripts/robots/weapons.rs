use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{Axis, Duration, Piece, TaskDefinition, UnitCtx};

use super::super::common::{SIG_AIM, turn};
use super::gait::wait_for_turn;
use super::{RobotAnimationState, RobotKind, spin, stop_spin};

pub(super) fn aim(
    ctx: &UnitCtx,
    state: Rc<RefCell<RobotAnimationState>>,
    _weapon: i32,
    heading: f32,
    pitch: f32,
) -> Option<bool> {
    let (kind, p) = {
        let mut state_ref = state.borrow_mut();
        state_ref.aiming = true;
        (state_ref.kind, state_ref.pieces)
    };

    ctx.signal(SIG_AIM);
    match kind {
        RobotKind::Wallier => {
            turn(ctx, p.turret, Axis::Y, heading, 19.0);
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
    Some(true)
}

async fn reload_missiles(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    ctx.sleep(Duration::from_millis(50)).await;
    let p = state.borrow().pieces;
    turn(&ctx, p.arm_up_01, Axis::X, 0.0, 0.1);
    turn(&ctx, p.arm_up_02, Axis::X, 0.0, 0.1);
    wait_for_turn(&ctx, p.arm_up_02).await;
}

pub(super) fn fire(ctx: &UnitCtx, state: Rc<RefCell<RobotAnimationState>>) {
    let mut state_ref = state.borrow_mut();
    state_ref.aiming = false;
    let p = state_ref.pieces;
    let kind = state_ref.kind;
    drop(state_ref);

    match kind {
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

pub(super) fn aim_from(state: &RobotAnimationState, _weapon: i32, next_gun: usize) -> Piece {
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

pub(super) fn query_weapon(state: &RobotAnimationState, _weapon: i32, next_gun: usize) -> Piece {
    match state.kind {
        RobotKind::Boss => state.pieces.turret,
        RobotKind::Wallier => state.pieces.rocket_01,
        RobotKind::Projector => match (next_gun + 1) % 4 {
            0 => state.pieces.gun_01,
            1 => state.pieces.gun_02,
            2 => state.pieces.gun_03,
            _ => state.pieces.gun_04,
        },
        RobotKind::Bob => {
            if next_gun.is_multiple_of(2) {
                state.pieces.mg_02
            } else {
                state.pieces.mg_01
            }
        }
    }
}

async fn reset_turret(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_AIM);
    ctx.sleep(Duration::from_millis(1000)).await;
    let turret = state.borrow().pieces.turret;
    turn(&ctx, turret, Axis::Y, 0.0, 5.0);
}
