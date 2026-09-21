use alloc::rc::Rc;
use core::cell::RefCell;

use spring::cus::{Axis, Duration, InitCtx, Piece, TaskDefinition, UnitCtx};

use super::common::{SIG_ANIMATION, SIG_STOP, turn};

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

    fn all(self) -> [Piece; 17] {
        [
            self.arm_right,
            self.arm_left,
            self.foot_left,
            self.foot_right,
            self.forearm_left,
            self.forearm_right,
            self.hand_left,
            self.hand_right,
            self.head,
            self.knee_left,
            self.knee_right,
            self.muzzle_left,
            self.muzzle_right,
            self.pelvis,
            self.thigh_left,
            self.thigh_right,
            self.torso,
        ]
    }
}

pub(super) struct GravitAnimationState {
    pub moving: bool,
    pub jumping: bool,
    pub pieces: GravitPieces,
}

pub(super) fn create(
    ctx: &UnitCtx,
    pieces: GravitPieces,
    state: Rc<RefCell<GravitAnimationState>>,
) {
    stop_pose(ctx, pieces, true);
    ctx.signal(SIG_ANIMATION);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-gravit-idle",
        state,
        idle,
    ));
}

pub(super) fn start_moving(
    ctx: &UnitCtx,
    pieces: GravitPieces,
    state: Rc<RefCell<GravitAnimationState>>,
) {
    state.borrow_mut().moving = true;
    ctx.signal(SIG_ANIMATION);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-gravit-walk",
        state,
        move_animation,
    ));
    let _ = pieces;
}

pub(super) fn stop_moving(
    ctx: &UnitCtx,
    pieces: GravitPieces,
    state: Rc<RefCell<GravitAnimationState>>,
) {
    state.borrow_mut().moving = false;
    ctx.signal(SIG_ANIMATION);
    ctx.spawn(TaskDefinition::with_state(
        "gravitas-gravit-idle",
        state,
        idle,
    ));
    let _ = pieces;
}

pub(super) fn aim(ctx: &UnitCtx, pieces: GravitPieces, left: bool, heading: f32, pitch: f32) {
    ctx.signal(SIG_ANIMATION);
    ctx.signal(SIG_STOP);
    turn(ctx, pieces.torso, Axis::Z, heading, 5.0);
    let (arm, forearm) = if left {
        (pieces.arm_left, pieces.forearm_left)
    } else {
        (pieces.arm_right, pieces.forearm_right)
    };
    for axis in [Axis::X, Axis::Y, Axis::Z] {
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
    state.borrow_mut().moving = false;
    ctx.signal(SIG_ANIMATION);
    let task = match name {
        "PreJump" => {
            TaskDefinition::with_state("gravitas-gravit-prejump", Rc::clone(&state), prejump)
        }
        "StartJump" => {
            TaskDefinition::with_state("gravitas-gravit-startjump", Rc::clone(&state), startjump)
        }
        "HalfJump" => {
            TaskDefinition::with_state("gravitas-gravit-midjump", Rc::clone(&state), midjump)
        }
        "StopJump" => {
            TaskDefinition::with_state("gravitas-gravit-stopjump", Rc::clone(&state), stopjump)
        }
        _ => return,
    };
    ctx.spawn(task);
}

pub(super) async fn idle(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    let pieces = state.borrow().pieces;
    stop_pose(&ctx, pieces, true);
    loop {
        if state.borrow().moving {
            return;
        }
        turn(&ctx, pieces.torso, Axis::X, -0.08, 0.8);
        turn(&ctx, pieces.pelvis, Axis::X, 0.08, 0.8);
        turn(&ctx, pieces.knee_left, Axis::X, 0.12, 0.8);
        turn(&ctx, pieces.knee_right, Axis::X, -0.12, 0.8);
        ctx.sleep(Duration::from_millis(330)).await;
        if state.borrow().moving {
            return;
        }
        turn(&ctx, pieces.torso, Axis::X, 0.08, 0.8);
        turn(&ctx, pieces.pelvis, Axis::X, -0.08, 0.8);
        turn(&ctx, pieces.knee_left, Axis::X, -0.12, 0.8);
        turn(&ctx, pieces.knee_right, Axis::X, 0.12, 0.8);
        ctx.sleep(Duration::from_millis(330)).await;
    }
}

pub(super) async fn move_animation(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    ctx.set_signal_mask(SIG_ANIMATION);
    let pieces = state.borrow().pieces;
    stop_pose(&ctx, pieces, true);
    let mut phase = false;
    while state.borrow().moving {
        let (left, right) = if phase { (0.65, -0.65) } else { (-0.65, 0.65) };
        turn(&ctx, pieces.thigh_left, Axis::X, left, 4.0);
        turn(&ctx, pieces.thigh_right, Axis::X, right, 4.0);
        turn(&ctx, pieces.knee_left, Axis::X, -right, 5.0);
        turn(&ctx, pieces.knee_right, Axis::X, -left, 5.0);
        turn(&ctx, pieces.foot_left, Axis::X, right * 0.6, 4.0);
        turn(&ctx, pieces.foot_right, Axis::X, left * 0.6, 4.0);
        ctx.sleep(Duration::from_millis(165)).await;
        phase = !phase;
    }
}

pub(super) async fn prejump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    let pieces = state.borrow().pieces;
    ctx.set_signal_mask(SIG_ANIMATION);
    turn(&ctx, pieces.pelvis, Axis::X, -0.35, 4.0);
    turn(&ctx, pieces.knee_left, Axis::X, 0.7, 5.0);
    turn(&ctx, pieces.knee_right, Axis::X, 0.7, 5.0);
    ctx.sleep(Duration::from_millis(66)).await;
}

pub(super) async fn startjump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    let pieces = state.borrow().pieces;
    ctx.set_signal_mask(SIG_ANIMATION);
    turn(&ctx, pieces.arm_left, Axis::X, -1.2, 5.0);
    turn(&ctx, pieces.arm_right, Axis::X, -1.2, 5.0);
    turn(&ctx, pieces.thigh_left, Axis::X, -0.8, 5.0);
    turn(&ctx, pieces.thigh_right, Axis::X, -0.8, 5.0);
    ctx.sleep(Duration::from_millis(132)).await;
}

pub(super) async fn midjump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    let pieces = state.borrow().pieces;
    ctx.set_signal_mask(SIG_ANIMATION);
    turn(&ctx, pieces.arm_left, Axis::X, 0.9, 4.0);
    turn(&ctx, pieces.arm_right, Axis::X, 0.9, 4.0);
    turn(&ctx, pieces.torso, Axis::X, -0.2, 3.0);
}

pub(super) async fn stopjump(state: Rc<RefCell<GravitAnimationState>>, ctx: UnitCtx) {
    let pieces = state.borrow().pieces;
    ctx.set_signal_mask(SIG_ANIMATION);
    turn(&ctx, pieces.arm_left, Axis::X, 0.0, 4.0);
    turn(&ctx, pieces.arm_right, Axis::X, 0.0, 4.0);
    turn(&ctx, pieces.thigh_left, Axis::X, 0.0, 4.0);
    turn(&ctx, pieces.thigh_right, Axis::X, 0.0, 4.0);
    ctx.sleep(Duration::from_millis(132)).await;
}

fn stop_pose(ctx: &UnitCtx, pieces: GravitPieces, snap: bool) {
    for piece in pieces.all() {
        for axis in [Axis::X, Axis::Y, Axis::Z] {
            if snap {
                turn(ctx, piece, axis, 0.0, 0.0);
            } else {
                turn(ctx, piece, axis, 0.0, 4.0);
            }
        }
    }
}
