use spring::ResultExt;
use spring::cus::{Angle, AngularSpeed, Axis, Piece, UnitCtx};

pub(super) const SIG_ANIMATION: spring::cus::SignalMask = spring::cus::SignalMask::new(1);
pub(super) const SIG_AIM: spring::cus::SignalMask = spring::cus::SignalMask::new(2);
pub(super) const SIG_STOP: spring::cus::SignalMask = spring::cus::SignalMask::new(4);
pub(super) const SIG_IDLE: spring::cus::SignalMask = spring::cus::SignalMask::new(8);
pub(super) const SIG_JUMP: spring::cus::SignalMask = spring::cus::SignalMask::new(16);
pub(super) const SIG_WALK: spring::cus::SignalMask = spring::cus::SignalMask::new(32);

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

pub(super) fn play_sound(ctx: &UnitCtx, name: &str, volume: f32) {
    ctx.with_spring(|spring| {
        let none = spring::Float3::default();
        spring
            .sound()
            .play_sound_file(name, volume, none, none, 0)
            .log_err_once(spring, "playing a Gravitas unit-script sound");
        Ok(())
    })
    .expect("unit-script engine scope");
}

pub(super) fn set_blocking(ctx: &UnitCtx, blocking: bool) {
    ctx.with_spring(|spring| {
        spring
            .unit_control()
            .set_unit_blocking(
                ctx.unit().0,
                spring::unit_control::SetUnitBlockingOptions {
                    blocking,
                    solid_objects: blocking,
                    projectiles: blocking,
                    quad_map_rays: blocking,
                    crushable: false,
                    block_enemy_pushing: blocking,
                    block_height_changes: blocking,
                },
            )
            .log_err_once(spring, "setting Gravitas unit-script blocking");
        Ok(())
    })
    .expect("unit-script engine scope");
}

pub(super) fn explode(ctx: &UnitCtx, piece: Piece, flags: spring::cus::SfxFlags) {
    if piece != Piece::INVALID {
        ctx.explode(piece, flags);
    }
}
