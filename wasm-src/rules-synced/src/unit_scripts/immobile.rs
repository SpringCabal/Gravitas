use spring::cus::{Axis, Duration, TaskDefinition, UnitCtx};

use super::common::{SIG_ANIMATION, move_piece, play_unit_sound, set_blocking, unit_piece};

pub(super) fn activate(ctx: &UnitCtx, kind: super::ScriptKind) {
    match kind {
        super::ScriptKind::Gate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-gate-open", gate_open));
        }
        super::ScriptKind::Plate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-plate-on", plate_on));
        }
        _ => {}
    }
}

pub(super) fn deactivate(ctx: &UnitCtx, kind: super::ScriptKind) {
    match kind {
        super::ScriptKind::Gate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-gate-close", gate_close));
        }
        super::ScriptKind::Plate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-plate-off", plate_off));
        }
        _ => {}
    }
}

pub(super) async fn gate_open(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    ctx.sleep(Duration::from_millis(500)).await;
    play_unit_sound("sounds/gate.ogg", 0.7, ctx.unit());
    let gate = unit_piece(ctx.unit(), "Gate");
    move_piece(&ctx, gate, Axis::Z, -168.0, 180.0);
    ctx.wait_for_move(gate, Axis::Z).await;
    set_blocking(ctx.unit(), false);
}

pub(super) async fn gate_close(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_unit_sound("sounds/gate.ogg", 0.7, ctx.unit());
    let gate = unit_piece(ctx.unit(), "Gate");
    move_piece(&ctx, gate, Axis::Z, 0.0, 190.0);
    ctx.wait_for_move(gate, Axis::Z).await;
    set_blocking(ctx.unit(), true);
}

pub(super) async fn plate_on(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_unit_sound("sounds/click.ogg", 1.0, ctx.unit());
    let button = unit_piece(ctx.unit(), "button");
    let middle = unit_piece(ctx.unit(), "middle");
    move_piece(&ctx, button, Axis::Z, -15.0, 150.0);
    ctx.wait_for_move(button, Axis::Z).await;
    move_piece(&ctx, middle, Axis::Z, -5.0, 50.0);
}

pub(super) async fn plate_off(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_unit_sound("sounds/click.ogg", 1.0, ctx.unit());
    let button = unit_piece(ctx.unit(), "button");
    let middle = unit_piece(ctx.unit(), "middle");
    move_piece(&ctx, button, Axis::Z, 0.0, 150.0);
    ctx.wait_for_move(button, Axis::Z).await;
    move_piece(&ctx, middle, Axis::Z, 0.0, 50.0);
}
