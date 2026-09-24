use alloc::vec::Vec;

use spring::UnitId;
use spring::cus::{Angle, AngularSpeed, Axis, Piece, PieceResolver, UnitCtx};

pub(super) const SIG_ANIMATION: spring::cus::SignalMask = spring::cus::SignalMask::new(1);
pub(super) const SIG_AIM: spring::cus::SignalMask = spring::cus::SignalMask::new(2);
pub(super) const SIG_STOP: spring::cus::SignalMask = spring::cus::SignalMask::new(4);
pub(super) const SIG_IDLE: spring::cus::SignalMask = spring::cus::SignalMask::new(8);
pub(super) const SIG_JUMP: spring::cus::SignalMask = spring::cus::SignalMask::new(16);
pub(super) const SIG_WALK: spring::cus::SignalMask = spring::cus::SignalMask::new(32);

pub(super) struct EnginePieces {
    pub(super) map: Vec<spring::owned::units_pieces::PieceMapEntry>,
    pub(super) root: Piece,
}

impl PieceResolver for EnginePieces {
    fn piece(&self, name: &str) -> Piece {
        if name == "__gravitas_model_root" {
            return self.root;
        }
        self.map
            .iter()
            .find(|entry| entry.name == name)
            .map_or(Piece::INVALID, |entry| Piece(entry.piece_num - 1))
    }
}

pub(super) fn turn(ctx: &UnitCtx, piece: Piece, axis: Axis, target: f32, speed: f32) {
    if piece == Piece::INVALID {
        return;
    }
    if speed == 0.0 {
        ctx.turn_now(piece, axis, Angle::radians(target));
    } else {
        ctx.turn(
            piece,
            axis,
            Angle::radians(target),
            AngularSpeed::radians_per_second(speed),
        );
    }
}

pub(super) fn move_piece(ctx: &UnitCtx, piece: Piece, axis: Axis, target: f32, speed: f32) {
    if piece == Piece::INVALID {
        return;
    }
    if speed == 0.0 {
        ctx.move_now(piece, axis, target);
    } else {
        ctx.move_piece(piece, axis, target, speed);
    }
}

pub(super) fn play_unit_sound(name: &str, volume: f32, unit: UnitId) {
    let Some(position) = spring::get_unit_position(unit.0, false, false).ok() else {
        return;
    };
    let _ =
        spring::sound::play_sound_file(name, volume, position, spring::types::Float3::default(), 0);
}

pub(super) fn set_blocking(unit: UnitId, blocking: bool) {
    let _ = spring::set_unit_blocking(
        unit.0,
        spring::rules_synced::unit_control::SetUnitBlockingOptions {
            blocking,
            solid_objects: blocking,
            projectiles: blocking,
            quad_map_rays: blocking,
            crushable: false,
            block_enemy_pushing: blocking,
            block_height_changes: blocking,
        },
    );
}

pub(super) fn explode(ctx: &UnitCtx, piece: Piece, flags: spring::cus::SfxFlags) {
    if piece != Piece::INVALID {
        ctx.explode(piece, flags);
    }
}
