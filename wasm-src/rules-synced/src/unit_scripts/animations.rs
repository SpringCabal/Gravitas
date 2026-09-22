#[path = "gravit_animation_data.rs"]
mod data;

use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{Axis, Duration, InitCtx, Piece, TaskDefinition, UnitCtx};

use super::common::{SIG_AIM, SIG_IDLE, SIG_JUMP, SIG_STOP, SIG_WALK, move_piece, turn};
use data::{AnimAxis, Command, CommandKind, Keyframe, PieceName};

#[derive(Clone, Copy)]
pub(super) struct GravitPieces {
    pub arm_right: Piece,
    pub arm_left: Piece,
    pub foot_left: Piece,
    pub foot_right: Piece,
    pub forearm_left: Piece,
    pub forearm_right: Piece,
    pub hand_left: Piece,
    pub hand_right: Piece,
    pub head: Piece,
    pub knee_left: Piece,
    pub knee_right: Piece,
    pub muzzle_left: Piece,
    pub muzzle_right: Piece,
    pub pelvis: Piece,
    pub thigh_left: Piece,
    pub thigh_right: Piece,
    pub torso: Piece,
}

impl GravitPieces {
    pub(super) fn from_init(ctx: &InitCtx<'_>) -> Self {
        Self {
            arm_right: ctx.piece("armright"),
            arm_left: ctx.piece("armleft"),
            foot_left: ctx.piece("footleft"),
            foot_right: ctx.piece("footright"),
            forearm_left: ctx.piece("forearmleft"),
            forearm_right: ctx.piece("forearmright"),
            hand_left: ctx.piece("handleft"),
            hand_right: ctx.piece("handright"),
            head: ctx.piece("head"),
            knee_left: ctx.piece("kneeleft"),
            knee_right: ctx.piece("kneeright"),
            muzzle_left: ctx.piece("muzzleleft"),
            muzzle_right: ctx.piece("muzzleright"),
            pelvis: ctx.piece("pelvis"),
            thigh_left: ctx.piece("thighleft"),
            thigh_right: ctx.piece("thighright"),
            torso: ctx.piece("torso"),
        }
    }

    fn piece(self, name: PieceName) -> Piece {
        match name {
            PieceName::Armright => self.arm_right,
            PieceName::Armleft => self.arm_left,
            PieceName::Footleft => self.foot_left,
            PieceName::Footright => self.foot_right,
            PieceName::Forearmleft => self.forearm_left,
            PieceName::Forearmright => self.forearm_right,
            PieceName::Handleft => self.hand_left,
            PieceName::Handright => self.hand_right,
            PieceName::Head => self.head,
            PieceName::Kneeleft => self.knee_left,
            PieceName::Kneeright => self.knee_right,
            PieceName::Muzzleleft => self.muzzle_left,
            PieceName::Muzzleright => self.muzzle_right,
            PieceName::Pelvis => self.pelvis,
            PieceName::Thighleft => self.thigh_left,
            PieceName::Thighright => self.thigh_right,
            PieceName::Torso => self.torso,
        }
    }
}

pub(super) struct GravitAnimationState {
    pub active: bool,
    pub jumping: bool,
    pub pieces: GravitPieces,
}

pub(super) fn activate(state: &Rc<RefCell<GravitAnimationState>>) {
    state.borrow_mut().active = true;
}

pub(super) fn deactivate(state: &Rc<RefCell<GravitAnimationState>>) {
    state.borrow_mut().active = false;
}

pub(super) fn create(
    ctx: &UnitCtx,
    pieces: GravitPieces,
    state: Rc<RefCell<GravitAnimationState>>,
) {
    let _ = state;
    apply_keyframe(ctx, pieces, data::STOP[0], false);
}

pub(super) fn start_moving(
    ctx: &UnitCtx,
    _pieces: GravitPieces,
    state: Rc<RefCell<GravitAnimationState>>,
) {
    ctx.signal(SIG_IDLE);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-gravit-walk",
        state,
        walk,
    ));
}

pub(super) fn stop_moving(
    ctx: &UnitCtx,
    _pieces: GravitPieces,
    state: Rc<RefCell<GravitAnimationState>>,
) {
    ctx.signal(SIG_WALK);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-gravit-idle",
        state,
        idle,
    ));
}

pub(super) fn aim(ctx: &UnitCtx, pieces: GravitPieces, left: bool, heading: f32, pitch: f32) {
    ctx.signal(SIG_AIM);
    ctx.signal(SIG_IDLE);
    ctx.signal(SIG_STOP);

    turn(ctx, pieces.torso, Axis::Z, heading, 5.0);
    let (arm, forearm) = if left {
        (pieces.arm_left, pieces.forearm_left)
    } else {
        (pieces.arm_right, pieces.forearm_right)
    };
    for axis in [Axis::Y, Axis::Z, Axis::X] {
        turn(ctx, forearm, axis, 0.0, 5.0);
    }
    turn(ctx, arm, Axis::Y, 0.0, 0.0);
    turn(
        ctx,
        arm,
        Axis::Z,
        if left {
            -core::f32::consts::FRAC_PI_2
        } else {
            core::f32::consts::FRAC_PI_2
        },
        0.0,
    );
    turn(ctx, arm, Axis::X, -pitch, 5.0);
}

pub(super) fn jump_task(
    ctx: &UnitCtx,
    state: Rc<RefCell<GravitAnimationState>>,
    name: &'static str,
) {
    ctx.signal(SIG_IDLE);
    ctx.signal(SIG_WALK);
    ctx.signal(SIG_STOP);
    ctx.signal(SIG_AIM);
    ctx.signal(SIG_JUMP);
    let task = match name {
        "PreJump" => TaskDefinition::with_state("gravitas-gravit-prejump", state, prejump),
        "StartJump" => TaskDefinition::with_state("gravitas-gravit-startjump", state, startjump),
        "HalfJump" => TaskDefinition::with_state("gravitas-gravit-midjump", state, midjump),
        "StopJump" => TaskDefinition::with_state("gravitas-gravit-stopjump", state, stopjump),
        _ => return,
    };
    ctx.spawn(task);
}

async fn idle(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_IDLE);
    let pieces = state.borrow().pieces;
    play_animation(&ctx, pieces, data::STOP, true).await;
    loop {
        play_animation(&ctx, pieces, data::IDLE, false).await;
    }
}

async fn walk(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_WALK);
    let pieces = state.borrow().pieces;
    play_animation(&ctx, pieces, data::WALK, true).await;
    loop {
        play_animation(&ctx, pieces, data::WALK, false).await;
    }
}

async fn prejump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_JUMP);
    let pieces = state.borrow().pieces;
    play_animation(&ctx, pieces, data::PREJUMP, false).await;
}

async fn startjump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_JUMP);
    let pieces = state.borrow().pieces;
    play_animation(&ctx, pieces, data::STARTJUMP, false).await;
}

async fn midjump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_JUMP);
    let pieces = state.borrow().pieces;
    play_animation(&ctx, pieces, data::MIDJUMP, false).await;
}

async fn stopjump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_JUMP);
    let pieces = state.borrow().pieces;
    play_animation(&ctx, pieces, data::STOPJUMP, false).await;
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-gravit-stop",
        state,
        stop,
    ));
}

async fn stop(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_STOP);
    ctx.sleep(Duration::from_millis(2000)).await;
    let pieces = state.borrow().pieces;
    play_animation(&ctx, pieces, data::STOP, false).await;
}

async fn play_animation(ctx: &UnitCtx, pieces: GravitPieces, animation: &[Keyframe], snap: bool) {
    for (index, frame) in animation.iter().enumerate() {
        for command in frame.commands {
            apply_command(ctx, pieces, *command, index == 0 && snap);
        }
        if let Some(next) = animation.get(index + 1) {
            let milliseconds = u64::from(next.time.saturating_sub(frame.time)) * 33;
            ctx.sleep(Duration::from_millis(milliseconds)).await;
        }
    }
}

fn apply_keyframe(ctx: &UnitCtx, pieces: GravitPieces, frame: Keyframe, snap: bool) {
    for command in frame.commands {
        apply_command(ctx, pieces, *command, snap);
    }
}

fn apply_command(ctx: &UnitCtx, pieces: GravitPieces, command: Command, snap: bool) {
    let piece = pieces.piece(command.piece);
    let axis = match command.axis {
        AnimAxis::X => Axis::X,
        AnimAxis::Y => Axis::Y,
        AnimAxis::Z => Axis::Z,
    };
    let speed = if snap { 0.0 } else { command.speed };
    match command.kind {
        CommandKind::Turn => turn(ctx, piece, axis, command.target, speed),
        CommandKind::Move => move_piece(ctx, piece, axis, command.target, speed),
    }
}
