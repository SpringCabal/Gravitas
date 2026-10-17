use alloc::string::String;
use alloc::vec::Vec;

use gravitas_shared::e2e::Request;

use spring::Float3;

use crate::state::Global;
use crate::tests::arena::Arena;
use crate::units;

pub trait Case {
    fn name(&self) -> &'static str;
    fn phase(&self) -> Phase {
        Phase::Arena
    }
    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step;
    fn rest(&self, _spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<Rest> {
        Vec::new()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Setup,
    Arena,
    Scene,
    Restart,
    Level,
}

pub const PHASES: [Phase; 5] = [
    Phase::Setup,
    Phase::Arena,
    Phase::Scene,
    Phase::Restart,
    Phase::Level,
];

pub enum Rest {
    At {
        unit: i32,
        at: Float3,
        within: f32,
        what: &'static str,
    },
    Gone {
        unit: i32,
        what: &'static str,
    },
}

impl Rest {
    pub fn at(unit: i32, at: Float3, within: f32, what: &'static str) -> Self {
        Rest::At {
            unit,
            at,
            within,
            what,
        }
    }

    pub fn here(
        spring: &spring::Spring<'_, spring::RulesSynced>,
        unit: i32,
        within: f32,
        what: &'static str,
    ) -> Option<Self> {
        position(spring, unit).map(|at| Rest::at(unit, at, within, what))
    }
}

pub enum Step {
    Wait,
    Pass,
    Fail(String),
}

pub const STILL: f32 = 10.0;

pub fn fail(reason: &str) -> Step {
    Step::Fail(String::from(reason))
}

pub fn arena<R>(global: &Global, function: impl FnOnce(&Arena) -> R) -> Option<R> {
    global.tests.arena.get().map(|arena| function(&arena))
}

pub fn look(spring: &spring::Spring<'_, spring::RulesSynced>, position: Float3, height: f32) {
    send_to_ui(
        spring,
        Request::Look {
            x: position.x,
            z: position.z,
            height: Some(height),
        },
    );
}

pub fn press_jump(spring: &spring::Spring<'_, spring::RulesSynced>, x: f32, z: f32) {
    let y = units::ground_height(spring, x, z);
    send_to_ui(
        spring,
        Request::Pointer {
            target: [x, y, z],
            jump: true,
        },
    );
}

pub fn release_pointer(spring: &spring::Spring<'_, spring::RulesSynced>) {
    send_to_ui(spring, Request::PointerOff);
}

pub fn press_restart(spring: &spring::Spring<'_, spring::RulesSynced>) {
    send_to_ui(spring, Request::Restart);
}

pub fn send_to_ui(spring: &spring::Spring<'_, spring::RulesSynced>, request: Request<'_>) {
    springcabal::message::send_to_ui(spring, &request.encode());
}

pub fn capture(spring: &spring::Spring<'_, spring::RulesSynced>, name: &str) {
    send_to_ui(spring, Request::Shot(name));
}

pub fn health(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> Option<f32> {
    spring
        .units_info()
        .get_unit_health(unit_id)
        .ok()
        .map(|health| health.health)
}

pub fn paralyze(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> Option<f32> {
    spring
        .units_info()
        .get_unit_health(unit_id)
        .ok()
        .map(|health| health.paralyze_damage)
}

pub fn heal(_spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, value: f32) {
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            let Ok(health) = spring.units_info().get_unit_health(unit_id) else {
                return;
            };
            let _ = spring.unit_control().set_unit_health(
                unit_id,
                spring::unit_control::UnitHealthValue {
                    health: value,
                    capture: health.capture_progress,
                    paralyze: 0.0,
                    build: health.build_progress,
                    use_amounts: true,
                },
            );
        })
    });
}

pub fn place(_spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, position: Float3) {
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            let _ = spring
                .unit_control()
                .set_unit_velocity(unit_id, Float3::default());
            let _ = spring.unit_control().set_unit_position(unit_id, position);
        })
    });
}

pub fn position(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> Option<Float3> {
    units::position(spring, unit_id)
}

pub fn piece_position(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    name: &str,
) -> Option<Float3> {
    let names = spring.units_pieces().get_unit_piece_list(unit_id).ok()?;
    let index = names.iter().position(|candidate| candidate == name)?;
    spring
        .units_pieces()
        .get_unit_piece_position(unit_id, index as i32 + 1)
        .ok()
}

pub fn distance(from: Float3, to: Float3) -> f32 {
    let dx = to.x - from.x;
    let dy = to.y - from.y;
    let dz = to.z - from.z;
    (dx * dx + dy * dy + dz * dz).sqrt()
}

pub fn planar_distance(from: Float3, to: Float3) -> f32 {
    let dx = to.x - from.x;
    let dz = to.z - from.z;
    (dx * dx + dz * dz).sqrt()
}

pub fn moved(from: Option<Float3>, to: Option<Float3>, minimum: f32) -> bool {
    let (Some(from), Some(to)) = (from, to) else {
        return false;
    };
    distance(from, to) > minimum
}

pub fn halt(unit_id: i32) {
    let _ = crate::with_addon_context(|ctx| {
        ctx.commands()
            .give_order_to_unit(unit_id, spring::cmd::STOP, &[], 0, 0)
    });
}

pub fn destroy(unit_id: i32) {
    let _ = crate::with_addon_context(|ctx| {
        ctx.commands().destroy_unit(
            unit_id,
            spring::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: false,
                attacker_id: -1,
                recycle_id: false,
            },
        )
    });
}
