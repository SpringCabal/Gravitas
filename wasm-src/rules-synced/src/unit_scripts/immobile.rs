use alloc::rc::Rc;
use core::cell::RefCell;

use spring::ResultExt;
use spring::cus::{Axis, Duration, InitCtx, Piece, SfxFlags, TaskDefinition, UnitCtx};

use super::ScriptKind;
use super::common::{SIG_ANIMATION, explode, move_piece, play_sound, set_blocking};

const WALL2_FRAGMENTS: [&str; 11] = [
    "frag1", "frag2", "frag3", "frag4", "frag5", "frag6", "frag7", "frag8", "frag9", "frag10",
    "frag11",
];

#[derive(Clone, Copy)]
struct Pieces {
    gate: Piece,
    button: Piece,
    middle: Piece,
    canister: Piece,
    cube: Piece,
    fragments: [Piece; WALL2_FRAGMENTS.len()],
}

pub(super) struct ImmobileScript {
    kind: ScriptKind,
    pieces: Rc<RefCell<Pieces>>,
}

impl ImmobileScript {
    pub(super) fn new(ctx: &InitCtx<'_>, kind: ScriptKind) -> Self {
        let pieces = Pieces {
            gate: ctx.piece("Gate"),
            button: ctx.piece("button"),
            middle: ctx.piece("middle"),
            canister: ctx.piece("canister"),
            cube: ctx.piece("Cube"),
            fragments: WALL2_FRAGMENTS.map(|name| ctx.piece(name)),
        };
        Self {
            kind,
            pieces: Rc::new(RefCell::new(pieces)),
        }
    }

    pub(super) fn killed(&self, ctx: &UnitCtx) {
        let p = *self.pieces.borrow();
        match self.kind {
            ScriptKind::Canister => explode(ctx, p.canister, SfxFlags::SHATTER),
            ScriptKind::Crate => explode(ctx, p.cube, SfxFlags::EXPLODE),
            ScriptKind::Wall2 => {
                for piece in p.fragments {
                    if let Some(fragment) =
                        spring::units_pieces::get_unit_piece_pos_dir(ctx.unit().0, piece.0 + 1)
                            .log_err_once("locating a wall fragment's explosion")
                    {
                        spring::spawn_ceg(
                            "flashnuke",
                            fragment.position,
                            spring::Float3::default(),
                            0.0,
                            0.0,
                            0.0,
                        )
                        .log_err_once("spawning a wall fragment's explosion");
                    }
                    explode(ctx, piece, SfxFlags(SfxFlags::FALL.0 | SfxFlags::EXPLODE.0));
                }
            }
            _ => {}
        }
    }

    pub(super) fn activate(&self, ctx: &UnitCtx) {
        let task = match self.kind {
            ScriptKind::Gate => {
                TaskDefinition::with_state("gravitas-gate-open", Rc::clone(&self.pieces), gate_open)
            }
            ScriptKind::Plate => {
                TaskDefinition::with_state("gravitas-plate-on", Rc::clone(&self.pieces), plate_on)
            }
            _ => return,
        };
        ctx.signal(SIG_ANIMATION);
        ctx.spawn(task);
    }

    pub(super) fn deactivate(&self, ctx: &UnitCtx) {
        let task = match self.kind {
            ScriptKind::Gate => TaskDefinition::with_state(
                "gravitas-gate-close",
                Rc::clone(&self.pieces),
                gate_close,
            ),
            ScriptKind::Plate => {
                TaskDefinition::with_state("gravitas-plate-off", Rc::clone(&self.pieces), plate_off)
            }
            _ => return,
        };
        ctx.signal(SIG_ANIMATION);
        ctx.spawn(task);
    }
}

async fn gate_open(pieces: Rc<RefCell<Pieces>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    ctx.sleep(Duration::from_millis(500)).await;
    play_sound("sounds/gate.ogg", 0.7);
    let gate = pieces.borrow().gate;
    move_piece(&ctx, gate, Axis::Z, -168.0, 180.0);
    ctx.wait_for_move(gate, Axis::Z).await;
    set_blocking(ctx.unit(), false);
}

async fn gate_close(pieces: Rc<RefCell<Pieces>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_sound("sounds/gate.ogg", 0.7);
    let gate = pieces.borrow().gate;
    move_piece(&ctx, gate, Axis::Z, 0.0, 190.0);
    ctx.wait_for_move(gate, Axis::Z).await;
    set_blocking(ctx.unit(), true);
}

async fn plate_on(pieces: Rc<RefCell<Pieces>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_sound("sounds/click.ogg", 1.0);
    let Pieces { button, middle, .. } = *pieces.borrow();
    move_piece(&ctx, button, Axis::Z, -15.0, 150.0);
    ctx.wait_for_move(button, Axis::Z).await;
    move_piece(&ctx, middle, Axis::Z, -5.0, 50.0);
}

async fn plate_off(pieces: Rc<RefCell<Pieces>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    play_sound("sounds/click.ogg", 1.0);
    let Pieces { button, middle, .. } = *pieces.borrow();
    move_piece(&ctx, button, Axis::Z, 0.0, 150.0);
    ctx.wait_for_move(button, Axis::Z).await;
    move_piece(&ctx, middle, Axis::Z, 0.0, 50.0);
}
