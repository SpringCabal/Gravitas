//! Gravitas unit scripts, run through the engine's Core-CUS API.
//!
//! The Lua game has three families of scripts: small scenery scripts, the
//! Cassos robots, and Gravit. One module hosts all of them.

mod common;
mod gravit;
mod immobile;
mod module;
mod robots;

use spring::cus::{InitCtx, Piece, ScriptCapabilities, UnitCtx, UnitScript, WeaponId, WreckLevel};

use self::gravit::GravitScript;
use self::robots::{RobotKind, RobotScript};

pub use self::module::{GravitasCusModule, attach, call};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScriptKind {
    Barrel,
    Canister,
    Crate,
    Gate,
    Electrafi,
    Fire,
    Plate,
    Wall1,
    Wall2,
    Wall3,
    Wall4,
    Gravit,
    Bob,
    Wallier,
    Projector,
    Boss,
}

impl ScriptKind {
    /// The script a unit def uses, by def name.
    pub fn for_def_name(name: &str) -> Option<Self> {
        Some(match name {
            "barrel" => Self::Barrel,
            "canister" => Self::Canister,
            "crate" => Self::Crate,
            "gate" => Self::Gate,
            "electrafi" => Self::Electrafi,
            "fire" => Self::Fire,
            "plate" => Self::Plate,
            "wall1" => Self::Wall1,
            "wall2" => Self::Wall2,
            "wall3" => Self::Wall3,
            "wall4" => Self::Wall4,
            "gravit" => Self::Gravit,
            "bob" | "b.o.b" => Self::Bob,
            "wallier" => Self::Wallier,
            "projector" => Self::Projector,
            "cometopapa" | "come_to_papa" => Self::Boss,
            _ => return None,
        })
    }
}

fn capabilities(kind: ScriptKind) -> ScriptCapabilities {
    let mut bits = ScriptCapabilities::KILLED;
    match kind {
        ScriptKind::Gravit => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::START_MOVING
                | ScriptCapabilities::STOP_MOVING
                | ScriptCapabilities::QUERY_NANO_PIECE
                | ScriptCapabilities::QUERY_BUILD_INFO
                | ScriptCapabilities::FIRE_WEAPON
                | ScriptCapabilities::QUERY_WEAPON
                | ScriptCapabilities::AIM_WEAPON
                | ScriptCapabilities::AIM_FROM_WEAPON;
        }
        ScriptKind::Bob | ScriptKind::Wallier | ScriptKind::Projector | ScriptKind::Boss => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::START_MOVING
                | ScriptCapabilities::STOP_MOVING
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE
                | ScriptCapabilities::QUERY_NANO_PIECE
                | ScriptCapabilities::QUERY_BUILD_INFO
                | ScriptCapabilities::FIRE_WEAPON
                | ScriptCapabilities::QUERY_WEAPON
                | ScriptCapabilities::AIM_WEAPON
                | ScriptCapabilities::AIM_FROM_WEAPON;
        }
        ScriptKind::Gate | ScriptKind::Plate => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE;
        }
        _ => {
            bits |= ScriptCapabilities::CREATE
                | ScriptCapabilities::ACTIVATE
                | ScriptCapabilities::DEACTIVATE;
        }
    }
    ScriptCapabilities::new(bits)
}

pub struct GravitasUnitScript {
    script: Script,
}

enum Script {
    Gravit(GravitScript),
    Robot(RobotScript),
    Immobile(ScriptKind),
    None,
}

impl UnitScript for GravitasUnitScript {
    fn new(ctx: &mut InitCtx<'_>) -> Self {
        let kind = crate::units::def_of(ctx.unit().0)
            .and_then(|def_id| crate::defs::get().unit(def_id).script);
        let script = match kind {
            Some(ScriptKind::Gravit) => Script::Gravit(GravitScript::new(ctx)),
            Some(kind) => match robot_kind(kind) {
                Some(robot) => Script::Robot(RobotScript::new(ctx, robot)),
                None => Script::Immobile(kind),
            },
            None => Script::None,
        };
        Self { script }
    }

    fn create(&mut self, ctx: &UnitCtx) {
        match &self.script {
            Script::Gravit(gravit) => gravit.create(ctx),
            Script::Robot(robot) => robot.create(ctx),
            Script::Immobile(_) | Script::None => {}
        }
    }

    fn start_moving(&mut self, ctx: &UnitCtx, _reversing: bool) {
        match &self.script {
            Script::Gravit(gravit) => gravit.start_moving(ctx),
            Script::Robot(robot) => robot.start_moving(),
            Script::Immobile(_) | Script::None => {}
        }
    }

    fn stop_moving(&mut self, ctx: &UnitCtx) {
        match &self.script {
            Script::Gravit(gravit) => gravit.stop_moving(ctx),
            Script::Robot(robot) => robot.stop_moving(),
            Script::Immobile(_) | Script::None => {}
        }
    }

    fn activate(&mut self, ctx: &UnitCtx) {
        match &self.script {
            Script::Gravit(gravit) => gravit.activate(),
            Script::Robot(_) => {}
            Script::Immobile(kind) => immobile::activate(ctx, *kind),
            Script::None => {}
        }
    }

    fn deactivate(&mut self, ctx: &UnitCtx) {
        match &self.script {
            Script::Gravit(gravit) => gravit.deactivate(),
            Script::Robot(_) => {}
            Script::Immobile(kind) => immobile::deactivate(ctx, *kind),
            Script::None => {}
        }
    }

    fn killed(&mut self, ctx: &UnitCtx, _recent_damage: f32, _max_health: f32) -> WreckLevel {
        match &self.script {
            Script::Gravit(gravit) => gravit.killed(ctx),
            Script::Robot(robot) => robot.killed(ctx),
            Script::Immobile(kind) => {
                immobile::killed(ctx, *kind);
                WreckLevel::One
            }
            Script::None => WreckLevel::One,
        }
    }

    fn killed_pending(&self) -> bool {
        match &self.script {
            Script::Gravit(gravit) => gravit.killed_pending(),
            _ => false,
        }
    }

    fn query_weapon(&mut self, _ctx: &UnitCtx, weapon: WeaponId) -> Piece {
        match &mut self.script {
            Script::Gravit(gravit) => gravit.query_weapon(),
            Script::Robot(robot) => robot.query_weapon(weapon),
            Script::Immobile(_) | Script::None => Piece::INVALID,
        }
    }

    fn aim_from_weapon(&mut self, _ctx: &UnitCtx, weapon: WeaponId) -> Piece {
        match &self.script {
            Script::Gravit(gravit) => gravit.aim_from_weapon(weapon),
            Script::Robot(robot) => robot.aim_from_weapon(weapon),
            Script::Immobile(_) | Script::None => Piece::INVALID,
        }
    }

    fn aim_weapon(
        &mut self,
        ctx: &UnitCtx,
        weapon: WeaponId,
        heading: f32,
        pitch: f32,
    ) -> Option<bool> {
        match &self.script {
            Script::Gravit(gravit) => {
                gravit.aim_weapon(ctx, weapon, heading, pitch);
                Some(true)
            }
            Script::Robot(robot) => robot.aim_weapon(ctx, weapon, heading, pitch),
            Script::Immobile(_) | Script::None => None,
        }
    }

    fn fire_weapon(&mut self, ctx: &UnitCtx, _weapon: WeaponId) {
        if let Script::Robot(robot) = &self.script {
            robot.fire_weapon(ctx);
        }
    }

    fn query_build_info(&mut self, _ctx: &UnitCtx) -> Piece {
        match &self.script {
            Script::Gravit(gravit) => gravit.build_piece(),
            Script::Robot(robot) => robot.build_piece(),
            Script::Immobile(_) | Script::None => Piece::INVALID,
        }
    }

    fn query_nano_piece(&mut self, ctx: &UnitCtx) -> Piece {
        self.query_build_info(ctx)
    }
}

impl GravitasUnitScript {
    fn named_call(&mut self, ctx: &UnitCtx, name: &str) -> bool {
        match &self.script {
            Script::Gravit(gravit) => gravit.named_call(ctx, name),
            _ => false,
        }
    }
}

fn robot_kind(kind: ScriptKind) -> Option<RobotKind> {
    match kind {
        ScriptKind::Bob => Some(RobotKind::Bob),
        ScriptKind::Wallier => Some(RobotKind::Wallier),
        ScriptKind::Projector => Some(RobotKind::Projector),
        ScriptKind::Boss => Some(RobotKind::Boss),
        _ => None,
    }
}
