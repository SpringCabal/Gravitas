//! The walk and idle cycles of projector.lua and projector_boss.lua.

use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{Axis, Duration, Piece, UnitCtx};

use super::super::common::{SIG_ANIMATION, move_piece, turn};
use super::{DEGREES, RobotAnimationState, RobotKind};

pub(super) async fn animate(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    let boss = state.borrow().kind == RobotKind::Boss;
    if boss {
        animate_boss(state, ctx).await;
    } else {
        animate_projector(state, ctx).await;
    }
}

async fn animate_projector(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    let mut walk_phase = 0usize;
    loop {
        while !state.borrow().started {
            ctx.sleep(Duration::from_millis(100)).await;
        }
        while state.borrow().moving {
            let second = !walk_phase.is_multiple_of(2);
            let wait_frames = match walk_phase {
                0 => 0,
                1 => 3,
                2 => 6,
                _ => 9,
            };
            let pause_ms = if walk_phase == 0 { 100 } else { 66 };
            projector_walk_phase(&state, &ctx, second, wait_frames, pause_ms).await;
            walk_phase += 1;
            if !state.borrow().moving {
                break;
            }
        }

        while !state.borrow().moving {
            projector_idle_phase(&state, &ctx, false).await;
            if state.borrow().moving {
                break;
            }
            projector_idle_phase(&state, &ctx, true).await;
        }

        ctx.sleep(Duration::from_millis(100)).await;
    }
}

async fn animate_boss(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    let mut walk_phase = 0usize;
    loop {
        while !state.borrow().started {
            ctx.sleep(Duration::from_millis(100)).await;
        }
        while state.borrow().moving {
            let second = !walk_phase.is_multiple_of(2);
            let wait_frames = match walk_phase {
                0 => 0,
                1 => 3,
                2 => 6,
                _ => 9,
            };
            let pause_ms = if walk_phase == 0 { 100 } else { 66 };
            boss_walk_phase(&state, &ctx, second, wait_frames, pause_ms).await;
            walk_phase += 1;
            if !state.borrow().moving {
                // projector_boss.lua has no boolMove check between its A and
                // B halves. Preserve that final B half after StopMoving.
                if !second {
                    let wait_frames = match walk_phase {
                        0 => 0,
                        1 => 3,
                        2 => 6,
                        _ => 9,
                    };
                    ctx.sleep_frames(2).await;
                    boss_walk_phase(&state, &ctx, true, wait_frames, 0).await;
                    walk_phase += 1;
                }
                break;
            }
        }

        while !state.borrow().moving {
            // This is the two-half idle loop from projector_boss.lua.  The
            // phase function contains the script's WaitForTurn and Sleep
            // calls, so the next half starts only at the same suspension
            // points as Lua.
            boss_idle_phase(&state, &ctx, false).await;
            boss_idle_phase(&state, &ctx, true).await;
        }

        ctx.sleep(Duration::from_millis(100)).await;
    }
}

async fn projector_walk_phase(
    state: &Rc<RefCell<RobotAnimationState>>,
    ctx: &UnitCtx,
    second: bool,
    wait_frames: u64,
    pause_ms: u64,
) {
    ctx.sleep(Duration::from_millis(pause_ms)).await;
    let p = state.borrow().pieces;
    if second {
        turn_degrees(ctx, p.leg_front_02, Axis::X, -15.0, 5.0);
        turn_degrees(ctx, p.leg_front_02, Axis::Y, -33.0, 5.0);
        turn_degrees(ctx, p.fr_blade_02, Axis::X, 64.0, 5.0);
        turn_degrees(ctx, p.leg_rear_01, Axis::X, -59.0, 5.0);
        turn_degrees(ctx, p.r_blade_01, Axis::X, 89.0, 5.0);
        reset_piece(ctx, p.leg_front_01, 5.0, 0).await;
        reset_piece(ctx, p.fr_blade_01, 5.0, 0).await;
        reset_piece(ctx, p.leg_rear_02, 5.0, 0).await;
        reset_piece(ctx, p.r_blade_02, 5.0, wait_frames).await;
    } else {
        turn_degrees(ctx, p.leg_front_01, Axis::X, -15.0, 5.0);
        turn_degrees(ctx, p.leg_front_01, Axis::Y, -33.0, 5.0);
        turn_degrees(ctx, p.fr_blade_01, Axis::X, 64.0, 5.0);
        turn_degrees(ctx, p.leg_rear_02, Axis::X, -59.0, 5.0);
        turn_degrees(ctx, p.r_blade_02, Axis::X, 89.0, 5.0);
        reset_piece(ctx, p.leg_front_02, 5.0, 0).await;
        reset_piece(ctx, p.fr_blade_02, 5.0, 0).await;
        reset_piece(ctx, p.leg_rear_01, 5.0, 0).await;
        reset_piece(ctx, p.r_blade_01, 5.0, wait_frames).await;
    }
    if !state.borrow().aiming {
        ctx.sleep_frames(1).await;
        let target = if second { 15.0 } else { -5.0 };
        turn_degrees(ctx, p.arm_up_01, Axis::X, target, 0.25);
        turn_degrees(ctx, p.arm_up_02, Axis::X, target, 0.25);
    }
}

async fn projector_idle_phase(
    state: &Rc<RefCell<RobotAnimationState>>,
    ctx: &UnitCtx,
    second: bool,
) {
    let p = state.borrow().pieces;
    let sign = if second { -1.0 } else { 1.0 };
    for (piece, degrees) in [
        (p.leg_front_01, -4.0),
        (p.leg_front_02, -4.0),
        (p.fr_blade_01, 4.0),
        (p.fr_blade_02, 4.0),
        (p.leg_rear_01, 4.0),
        (p.leg_rear_02, 4.0),
        (p.r_blade_01, -4.0),
        (p.r_blade_02, -4.0),
    ] {
        turn_x(ctx, piece, degrees * sign, 0.25);
    }
    // tP(..., true) on rblade02 is a real Lua WaitForTurn.  Preserve that
    // yield instead of replacing it with a guessed timer.
    wait_for_turn(ctx, p.r_blade_02).await;
    if !state.borrow().aiming {
        turn_x(ctx, p.arm_up_01, if second { 10.0 } else { 5.0 }, 0.05);
        turn_x(ctx, p.arm_up_02, if second { 10.0 } else { 5.0 }, 0.05);
    }
    move_piece(
        ctx,
        p.center,
        Axis::Y,
        if second { 0.0 } else { -5.0 },
        if second { 1.0 } else { 2.0 },
    );
}

async fn boss_walk_phase(
    state: &Rc<RefCell<RobotAnimationState>>,
    ctx: &UnitCtx,
    second: bool,
    wait_frames: u64,
    pause_ms: u64,
) {
    if pause_ms > 0 {
        ctx.sleep(Duration::from_millis(pause_ms)).await;
    }
    let p = state.borrow().pieces;
    if second {
        turn_degrees(ctx, p.leg_front_02, Axis::X, -15.0, 5.0);
        turn_degrees(ctx, p.leg_front_02, Axis::Y, -33.0, 5.0);
        turn_degrees(ctx, p.fr_blade_02, Axis::X, 64.0, 5.0);
        turn_degrees(ctx, p.leg_rear_01, Axis::X, -59.0, 5.0);
        turn_degrees(ctx, p.leg_rear_03, Axis::X, -59.0, 5.0);
        turn_degrees(ctx, p.r_blade_03, Axis::X, 89.0, 5.0);
        turn_degrees(ctx, p.r_blade_01, Axis::X, 89.0, 5.0);
        reset_piece(ctx, p.leg_front_01, 5.0, 0).await;
        reset_piece(ctx, p.fr_blade_01, 5.0, 0).await;
        reset_piece(ctx, p.leg_rear_04, 5.0, 0).await;
        reset_piece(ctx, p.leg_rear_02, 5.0, 0).await;
        reset_piece(ctx, p.r_blade_04, 5.0, 0).await;
        reset_piece(ctx, p.r_blade_02, 5.0, wait_frames).await;
    } else {
        turn_degrees(ctx, p.leg_front_01, Axis::X, -15.0, 5.0);
        turn_degrees(ctx, p.leg_front_01, Axis::Y, -33.0, 5.0);
        turn_degrees(ctx, p.fr_blade_01, Axis::X, 64.0, 5.0);
        turn_degrees(ctx, p.leg_rear_02, Axis::X, -59.0, 5.0);
        turn_degrees(ctx, p.leg_rear_04, Axis::X, -59.0, 5.0);
        turn_degrees(ctx, p.r_blade_02, Axis::X, 89.0, 5.0);
        turn_degrees(ctx, p.r_blade_04, Axis::X, 89.0, 5.0);
        reset_piece(ctx, p.leg_front_02, 5.0, 0).await;
        reset_piece(ctx, p.fr_blade_02, 5.0, 0).await;
        reset_piece(ctx, p.leg_rear_01, 5.0, 0).await;
        reset_piece(ctx, p.leg_rear_03, 5.0, 0).await;
        reset_piece(ctx, p.r_blade_03, 5.0, 0).await;
        reset_piece(ctx, p.r_blade_01, 5.0, wait_frames).await;
    }
    if !state.borrow().aiming {
        ctx.sleep_frames(1).await;
        let target = if second { 15.0 } else { -5.0 };
        turn_degrees(ctx, p.arm_up_01, Axis::X, target, 0.25);
        turn_degrees(ctx, p.arm_up_02, Axis::X, target, 0.25);
    }
}

async fn boss_idle_phase(state: &Rc<RefCell<RobotAnimationState>>, ctx: &UnitCtx, second: bool) {
    let p = state.borrow().pieces;
    if !second {
        for (piece, value) in [
            (p.leg_front_01, -4.0),
            (p.leg_front_02, -4.0),
            (p.fr_blade_01, 4.0),
            (p.fr_blade_02, 4.0),
            (p.leg_rear_01, 4.0),
            (p.leg_rear_03, 4.0),
            (p.leg_rear_04, 4.0),
            (p.leg_rear_02, 4.0),
            (p.r_blade_04, -4.0),
            (p.r_blade_01, -4.0),
        ] {
            turn_x(ctx, piece, value, 0.25);
        }
        turn_x(ctx, p.r_blade_03, -4.0, 0.25);
        wait_for_turn(ctx, p.r_blade_03).await;
        turn_x(ctx, p.r_blade_02, -4.0, 0.25);
        wait_for_turn(ctx, p.r_blade_02).await;
    } else {
        for (piece, value) in [
            (p.leg_front_01, 4.0),
            (p.leg_front_02, 4.0),
            (p.fr_blade_01, -4.0),
            (p.fr_blade_02, -4.0),
            (p.leg_rear_01, -4.0),
            (p.leg_rear_03, -4.0),
            (p.leg_rear_02, -4.0),
            (p.leg_rear_04, -4.0),
            (p.r_blade_03, 4.0),
            (p.r_blade_04, 4.0),
            (p.r_blade_01, 4.0),
        ] {
            turn_x(ctx, piece, value, 0.25);
        }
        turn_x(ctx, p.r_blade_02, 4.0, 0.25);
        wait_for_turn(ctx, p.r_blade_02).await;
    }
    if !state.borrow().aiming {
        turn_x(ctx, p.arm_up_01, if second { 10.0 } else { 5.0 }, 0.05);
        turn_x(ctx, p.arm_up_02, if second { 10.0 } else { 5.0 }, 0.05);
    }
    move_piece(
        ctx,
        p.center,
        Axis::Y,
        if second { 0.0 } else { -5.0 },
        if second { 1.0 } else { 2.0 },
    );
    ctx.sleep(Duration::from_millis(100)).await;
}

pub(super) fn turn_degrees(ctx: &UnitCtx, piece: Piece, axis: Axis, degrees: f32, speed: f32) {
    turn(ctx, piece, axis, degrees * DEGREES, speed);
}

/// The Lua toolkit's `tP(piece, x, 0, 0, speed)`: turn about X, zero Y and Z.
fn turn_x(ctx: &UnitCtx, piece: Piece, x_degrees: f32, speed: f32) {
    turn_degrees(ctx, piece, Axis::X, x_degrees, speed);
    turn(ctx, piece, Axis::Y, 0.0, speed);
    turn(ctx, piece, Axis::Z, 0.0, speed);
}

pub(super) async fn wait_for_turn(ctx: &UnitCtx, piece: Piece) {
    if piece == Piece::INVALID {
        return;
    }
    ctx.wait_for_turn(piece, Axis::X).await;
    ctx.wait_for_turn(piece, Axis::Y).await;
    ctx.wait_for_turn(piece, Axis::Z).await;
}

async fn reset_piece(ctx: &UnitCtx, piece: Piece, speed: f32, wait_frames: u64) {
    if piece == Piece::INVALID {
        return;
    }
    turn(ctx, piece, Axis::X, 0.0, speed);
    turn(ctx, piece, Axis::Y, 0.0, speed);
    turn(ctx, piece, Axis::Z, 0.0, speed);
    move_piece(ctx, piece, Axis::X, 0.0, speed);
    move_piece(ctx, piece, Axis::Y, 0.0, speed);
    move_piece(ctx, piece, Axis::Z, 0.0, speed);
    if wait_frames > 0 {
        // The Lua helper waits for all three turns, but only the X turn can
        // take time in these gait phases.  The native CUS listener currently
        // loses the completion callback for this exact reset, so use the
        // engine's fixed 30 Hz step directly.  The opposite blade is still
        // part-way through the preceding 89-degree turn when this reset is
        // issued.  The first three waits are 0/3/6 frames as the two blades
        // settle into alternation; steady-state waits are 9 frames at speed5.
        ctx.sleep_frames(wait_frames).await;
    }
}
