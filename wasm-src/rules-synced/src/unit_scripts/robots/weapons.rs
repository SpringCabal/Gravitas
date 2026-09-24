//! Aiming, firing and the explicit-target watch of the robot scripts.

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

pub(super) async fn watch_explicit_target(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
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

pub(super) fn fire(ctx: &UnitCtx, state: Rc<RefCell<RobotAnimationState>>) {
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

async fn reset_turret(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_AIM);
    ctx.sleep(Duration::from_millis(1000)).await;
    let turret = state.borrow().pieces.turret;
    turn(&ctx, turret, Axis::Y, 0.0, 5.0);
}
