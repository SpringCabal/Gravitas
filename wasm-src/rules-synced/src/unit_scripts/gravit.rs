#[path = "gravit_animation_data.rs"]
mod data;

use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{
    Axis, Duration, InitCtx, Piece, SfxFlags, TaskDefinition, UnitCtx, WeaponId, WreckLevel,
};

use super::JumpCall;
use super::common::{
    SIG_AIM, SIG_IDLE, SIG_JUMP, SIG_STOP, SIG_WALK, explode, move_piece, play_sound, turn,
};
use data::{AnimAxis, Command, CommandKind, Keyframe, PieceName};

#[derive(Clone, Copy)]
struct GravitPieces {
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
    fn from_init(ctx: &InitCtx<'_>) -> Self {
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

struct GravitAnimationState {
    active: bool,
    jumping: bool,
    dying: bool,
    pieces: GravitPieces,
}

pub(super) struct GravitScript {
    pieces: GravitPieces,
    state: Rc<RefCell<GravitAnimationState>>,
}

impl GravitScript {
    pub(super) fn new(ctx: &InitCtx<'_>) -> Self {
        let pieces = GravitPieces::from_init(ctx);
        Self {
            pieces,
            state: Rc::new(RefCell::new(GravitAnimationState {
                active: false,
                jumping: false,
                dying: false,
                pieces,
            })),
        }
    }

    pub(super) fn create(&self, ctx: &UnitCtx) {
        apply_keyframe(ctx, self.pieces, data::STOP[0], false);
    }

    pub(super) fn start_moving(&self, ctx: &UnitCtx) {
        ctx.signal(SIG_IDLE);
        ctx.signal(SIG_WALK);
        ctx.spawn(TaskDefinition::with_state(
            "gravitas-gravit-walk",
            Rc::clone(&self.state),
            walk,
        ));
    }

    pub(super) fn stop_moving(&self, ctx: &UnitCtx) {
        ctx.signal(SIG_WALK);
        ctx.signal(SIG_IDLE);
        ctx.spawn(TaskDefinition::with_state(
            "gravitas-gravit-idle",
            Rc::clone(&self.state),
            idle,
        ));
    }

    pub(super) fn activate(&self) {
        self.state.borrow_mut().active = true;
    }

    pub(super) fn deactivate(&self) {
        self.state.borrow_mut().active = false;
    }

    pub(super) fn killed(&self, ctx: &UnitCtx) -> WreckLevel {
        ctx.signal(SIG_AIM);
        ctx.signal(SIG_WALK);
        ctx.signal(SIG_IDLE);
        play_sound("sounds/death.ogg", 1.0);
        self.state.borrow_mut().dying = true;
        ctx.spawn(TaskDefinition::with_state(
            "gravitas-gravit-die",
            Rc::clone(&self.state),
            die,
        ));
        WRECK_LEVEL
    }

    pub(super) fn killed_pending(&self) -> bool {
        self.state.borrow().dying
    }

    pub(super) fn query_weapon(&self) -> Piece {
        if self.state.borrow().active {
            self.pieces.muzzle_right
        } else {
            self.pieces.muzzle_left
        }
    }

    pub(super) fn aim_from_weapon(&self, weapon: WeaponId) -> Piece {
        match weapon.0 {
            0 => self.pieces.arm_right,
            _ => self.pieces.head,
        }
    }

    pub(super) fn aim_weapon(&self, ctx: &UnitCtx, weapon: WeaponId, heading: f32, pitch: f32) {
        if !self.state.borrow().jumping {
            aim(ctx, self.pieces, weapon.0 != 0, heading, pitch);
        }
    }

    pub(super) fn build_piece(&self) -> Piece {
        self.pieces.head
    }

    pub(super) fn jump(&self, ctx: &UnitCtx, call: JumpCall) {
        let state = Rc::clone(&self.state);
        let task = match call {
            JumpCall::PreJump => {
                TaskDefinition::with_state("gravitas-gravit-prejump", state, prejump)
            }
            JumpCall::StartJump => {
                self.state.borrow_mut().jumping = true;
                TaskDefinition::with_state("gravitas-gravit-startjump", state, startjump)
            }
            JumpCall::HalfJump => {
                TaskDefinition::with_state("gravitas-gravit-midjump", state, midjump)
            }
            JumpCall::StopJump => {
                self.state.borrow_mut().jumping = false;
                TaskDefinition::with_state("gravitas-gravit-stopjump", state, stopjump)
            }
            JumpCall::Jumping => return,
        };
        ctx.signal(SIG_IDLE);
        ctx.signal(SIG_WALK);
        ctx.signal(SIG_STOP);
        ctx.signal(SIG_AIM);
        ctx.signal(SIG_JUMP);
        ctx.spawn(task);
    }
}

fn aim(ctx: &UnitCtx, pieces: GravitPieces, left: bool, heading: f32, pitch: f32) {
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
    ctx.signal(SIG_STOP);
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

async fn die(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    let p = state.borrow().pieces;
    play_animation(&ctx, p, data::DIE, false).await;
    for piece in [
        p.thigh_left,
        p.thigh_right,
        p.head,
        p.pelvis,
        p.arm_left,
        p.arm_right,
    ] {
        explode(&ctx, piece, SfxFlags::EXPLODE);
    }
    state.borrow_mut().dying = false;
    ctx.set_killed_finished(WRECK_LEVEL);
}

const WRECK_LEVEL: WreckLevel = WreckLevel::One;

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
