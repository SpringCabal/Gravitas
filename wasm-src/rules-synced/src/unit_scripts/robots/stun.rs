use alloc::rc::Rc;
use alloc::vec::Vec;
use core::cell::RefCell;

use spring::cus::{Axis, Duration, Piece, UnitCtx};
use spring::random::synced as random;

use super::super::common::{move_piece, turn};
use super::gait::{turn_degrees, wait_for_turn};
use super::{DEGREES, RobotAnimationState, RobotKind, RobotPieces};

pub(super) async fn detect(state: Rc<RefCell<RobotAnimationState>>, ctx: UnitCtx) {
    loop {
        ctx.sleep(Duration::from_millis(50)).await;
        while paralyzed(ctx.unit()) {
            state.borrow_mut().stunned = true;
            ctx.sleep(Duration::from_millis(500)).await;
        }
        state.borrow_mut().stunned = false;
    }
}

fn paralyzed(unit: spring::UnitId) -> bool {
    spring::units_info::get_unit_health(unit.0)
        .is_ok_and(|health| health.health < health.paralyze_damage)
}

pub(super) async fn while_stunned(state: &Rc<RefCell<RobotAnimationState>>, ctx: &UnitCtx) {
    while state.borrow().stunned {
        let (pieces, kind) = {
            let state = state.borrow();
            (state.pieces, state.kind)
        };
        spasm(ctx, pieces, kind, state).await;
    }
}

async fn spasm(
    ctx: &UnitCtx,
    p: RobotPieces,
    kind: RobotKind,
    state: &Rc<RefCell<RobotAnimationState>>,
) {
    let mut decrease: f32 = 44.0;
    while decrease > 1.0 {
        twitch_legs(ctx, p).await;
        for (piece, angle) in [
            (p.leg_front_01, -decrease),
            (p.leg_front_02, -decrease),
            (p.fr_blade_01, decrease),
            (p.fr_blade_02, decrease),
            (p.leg_rear_01, decrease),
            (p.leg_rear_02, decrease),
            (p.r_blade_01, -decrease),
            (p.r_blade_02, -decrease),
        ] {
            snap(ctx, piece, [X_AXIS_INDEX, angle, 5.0]);
        }
        decrease = -decrease;
        if random::int(0, 1) == 1 {
            for _ in 0..5 {
                for piece in [p.fr_blade_01, p.fr_blade_02, p.r_blade_01, p.r_blade_02] {
                    turn_all(ctx, piece, jitter(-10, 10) + decrease, SPASM_SPEED);
                }
            }
        }
        let spread = decrease as i32;
        opposite(
            ctx,
            p.blade_up_01,
            p.arm_up_01,
            random::int(spread, -spread) as f32,
        );
        opposite(
            ctx,
            p.blade_up_02,
            p.arm_up_02,
            random::int(spread, -spread) as f32,
        );
        opposite(
            ctx,
            p.fr_blade_02,
            p.leg_front_02,
            jitter(-10, 0) + decrease,
        );
        opposite(
            ctx,
            p.fr_blade_01,
            p.leg_front_01,
            jitter(-10, 0) + decrease,
        );
        opposite(ctx, p.leg_rear_01, p.r_blade_01, jitter(-10, 0) + decrease);
        opposite(ctx, p.leg_rear_02, p.r_blade_02, jitter(-10, 0) + decrease);
        ctx.sleep(Duration::from_millis(500)).await;
        twitch_legs(ctx, p).await;
        decrease = -decrease / 2.0;
        ctx.sleep(Duration::from_millis(100)).await;
    }
    let body = body_pieces(p, kind);
    while state.borrow().stunned {
        let offset = random::int(11, 11) as f32 * WAVE_PHASE;
        let step = core::f32::consts::TAU / body.len() as f32;
        for (index, piece) in body.iter().enumerate() {
            let angle = WAVE_AMPLITUDE * (offset + (index + 1) as f32 * step).sin();
            turn_degrees(ctx, *piece, Axis::X, angle, WAVE_SPEED);
        }
        ctx.sleep(Duration::from_millis(1000)).await;
    }
    for piece in body {
        reset(ctx, piece);
    }
}

const SPASM_SPEED: f32 = 5.0;
const RELEASE_SPEED: f32 = 4.5;
const WAVE_PHASE: f32 = core::f32::consts::PI * 0.45;
const WAVE_AMPLITUDE: f32 = 8.5;
const WAVE_SPEED: f32 = 0.14;
const RESET_SPEED: f32 = 15.0;
const X_AXIS_INDEX: f32 = 1.0;

fn jitter(low: i32, high: i32) -> f32 {
    random::int(low, high) as f32
}

async fn twitch_legs(ctx: &UnitCtx, p: RobotPieces) {
    for piece in [p.leg_front_02, p.leg_front_01, p.leg_rear_01, p.leg_rear_02] {
        turn_all(ctx, piece, jitter(-10, 10), SPASM_SPEED);
    }
    wait_for_turn(ctx, p.leg_rear_02).await;
}

fn turn_all(ctx: &UnitCtx, piece: Piece, x_degrees: f32, speed: f32) {
    turn_degrees(ctx, piece, Axis::X, x_degrees, speed);
    turn(ctx, piece, Axis::Y, 0.0, speed);
    turn(ctx, piece, Axis::Z, 0.0, speed);
}

fn snap(ctx: &UnitCtx, piece: Piece, degrees: [f32; 3]) {
    for (axis, angle) in [Axis::X, Axis::Y, Axis::Z].into_iter().zip(degrees) {
        turn(ctx, piece, axis, angle * DEGREES, 0.0);
    }
}

fn opposite(ctx: &UnitCtx, first: Piece, second: Piece, degrees: f32) {
    turn_degrees(ctx, first, Axis::X, degrees, RELEASE_SPEED);
    turn_degrees(ctx, second, Axis::X, -degrees, RELEASE_SPEED);
}

fn reset(ctx: &UnitCtx, piece: Piece) {
    for axis in [Axis::X, Axis::Y, Axis::Z] {
        turn(ctx, piece, axis, 0.0, RESET_SPEED);
        move_piece(ctx, piece, axis, 0.0, RESET_SPEED);
    }
}

fn body_pieces(p: RobotPieces, kind: RobotKind) -> Vec<Piece> {
    let mut pieces = alloc::vec![
        p.center,
        p.body,
        p.leg_front_02,
        p.fr_blade_02,
        p.leg_rear_02,
        p.r_blade_02,
        p.leg_rear_01,
        p.r_blade_01,
        p.leg_front_01,
        p.fr_blade_01,
    ];
    if kind == RobotKind::Boss {
        pieces.extend([
            p.arm_up_01,
            p.blade_up_01,
            p.arm_up_02,
            p.blade_up_02,
            p.turret,
            p.gun_01,
            p.gun_02,
            p.gun_03,
            p.gun_04,
        ]);
    } else {
        pieces.extend([
            p.arm_up_01,
            p.mg_01,
            p.mg_b_01,
            p.blade_up_01,
            p.rocket_01,
            p.arm_up_02,
            p.mg_02,
            p.mg_b_02,
            p.blade_up_02,
            p.rocket_02,
            p.turret,
            p.gun_01,
            p.gun_02,
            p.gun_03,
            p.gun_04,
        ]);
    }
    pieces
}
