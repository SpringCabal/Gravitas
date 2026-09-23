//! The scenery scripts: gate, plate and the pieces that break off crates,
//! canisters and walls.

use spring::cus::{Axis, Duration, SfxFlags, TaskDefinition, UnitCtx};

use super::ScriptKind;
use super::common::{
    SIG_ANIMATION, explode, move_piece, play_unit_sound, set_blocking, unit_piece,
};

const WALL2_FRAGMENTS: [&str; 11] = [
    "frag1", "frag2", "frag3", "frag4", "frag5", "frag6", "frag7", "frag8", "frag9", "frag10",
    "frag11",
];

pub(super) fn killed(ctx: &UnitCtx, kind: ScriptKind) {
    match kind {
        ScriptKind::Canister => explode_piece(ctx, "canister", SfxFlags::SHATTER),
        ScriptKind::Crate => explode_piece(ctx, "Cube", SfxFlags::EXPLODE),
        ScriptKind::Wall2 => {
            for name in WALL2_FRAGMENTS {
                explode_piece(ctx, name, SfxFlags(SfxFlags::FALL.0 | SfxFlags::EXPLODE.0));
            }
        }
        _ => {}
    }
}

fn explode_piece(ctx: &UnitCtx, piece: &str, flags: SfxFlags) {
    explode(ctx, unit_piece(ctx.unit(), piece), flags);
}

pub(super) fn activate(ctx: &UnitCtx, kind: ScriptKind) {
    match kind {
        ScriptKind::Gate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-gate-open", gate_open));
        }
        ScriptKind::Plate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-plate-on", plate_on));
        }
        _ => {}
    }
}

pub(super) fn deactivate(ctx: &UnitCtx, kind: ScriptKind) {
    match kind {
        ScriptKind::Gate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-gate-close", gate_close));
        }
        ScriptKind::Plate => {
            ctx.signal(SIG_ANIMATION);
            ctx.spawn(TaskDefinition::new("gravitas-plate-off", plate_off));
        }
        _ => {}
    }
}

async fn gate_open(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    ctx.sleep(Duration::from_millis(500)).await;
    play_unit_sound("sounds/gate.ogg", 0.7, ctx.unit());
    let gate = unit_piece(ctx.unit(), "Gate");
    move_piece(&ctx, gate, Axis::Z, -168.0, 180.0);
    ctx.wait_for_move(gate, Axis::Z).await;
    set_blocking(ctx.unit(), false);
}

async fn gate_close(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_unit_sound("sounds/gate.ogg", 0.7, ctx.unit());
    let gate = unit_piece(ctx.unit(), "Gate");
    move_piece(&ctx, gate, Axis::Z, 0.0, 190.0);
    ctx.wait_for_move(gate, Axis::Z).await;
    set_blocking(ctx.unit(), true);
}

async fn plate_on(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_unit_sound("sounds/click.ogg", 1.0, ctx.unit());
    let button = unit_piece(ctx.unit(), "button");
    let middle = unit_piece(ctx.unit(), "middle");
    move_piece(&ctx, button, Axis::Z, -15.0, 150.0);
    ctx.wait_for_move(button, Axis::Z).await;
    move_piece(&ctx, middle, Axis::Z, -5.0, 50.0);
}

async fn plate_off(ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_unit_sound("sounds/click.ogg", 1.0, ctx.unit());
    let button = unit_piece(ctx.unit(), "button");
    let middle = unit_piece(ctx.unit(), "middle");
    move_piece(&ctx, button, Axis::Z, 0.0, 150.0);
    ctx.wait_for_move(button, Axis::Z).await;
    move_piece(&ctx, middle, Axis::Z, 0.0, 50.0);
}
