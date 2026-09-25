use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::arena::RIG_SPACING;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};

/// A robot on an assault that gets hurt by a hazard steps aside and then
/// carries on: the escape goes in front of its FIGHT order (unit_robots.lua
/// inserts it), and a second escape replaces the first.
pub struct Fleeing {
    robot: i32,
    origin: Float3,
    fire: i32,
    fire_at: Float3,
}

impl Fleeing {
    pub fn new() -> Self {
        Self {
            robot: -1,
            origin: Float3::default(),
            fire: -1,
            fire_at: Float3::default(),
        }
    }
}

impl Case for Fleeing {
    fn name(&self) -> &'static str {
        "flee"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            ASSAULT_FRAME => self.assault(global),
            FIRST_HIT_FRAME => self.hit_and_check("the first escape"),
            // After the flee cooldown, while the first escape is still queued.
            SECOND_HIT_FRAME => match self.hit_and_check("the second escape") {
                Step::Wait => {
                    harness::halt(self.robot);
                    Step::Pass
                }
                verdict => verdict,
            },
            _ => Step::Wait,
        }
    }

    /// The escapes decide where the robot stops; it stays in its rig.
    fn rest(&self) -> Vec<Rest> {
        alloc::vec![
            Rest::at(
                self.fire,
                self.fire_at,
                STILL,
                "the fire that scared the robot"
            ),
            Rest::at(
                self.robot,
                self.origin,
                RIG_SPACING * 0.5,
                "the fleeing robot"
            ),
        ]
    }
}

const ASSAULT_FRAME: i32 = 2;
const FIRST_HIT_FRAME: i32 = 4;
const SECOND_HIT_FRAME: i32 = 40;
const ASSAULT_DISTANCE: f32 = 300.0;
const HIT_DAMAGE: f32 = 1.0;

impl Fleeing {
    fn assault(&mut self, global: &Global) -> Step {
        let Some((robot, fire)) = harness::arena(global, |arena| (arena.fleeing, arena.scare))
        else {
            return fail("no arena");
        };
        let (Some(origin), Some(fire_at)) = (harness::position(robot), harness::position(fire))
        else {
            return fail("no robot or fire position");
        };
        self.robot = robot;
        self.origin = origin;
        self.fire = fire;
        self.fire_at = fire_at;
        // Up the arena screenshot, away from the HUD.
        let z = origin.z - ASSAULT_DISTANCE;
        let target = Float3 {
            x: origin.x,
            y: crate::units::ground_height(origin.x, z),
            z,
        };
        crate::units::order(robot, spring::cmd::FIGHT, &[target.x, target.y, target.z]);
        Step::Wait
    }

    /// Hurt the robot with the fire as the attacker, then read its queue.
    fn hit_and_check(&mut self, what: &str) -> Step {
        let _ = spring::add_unit_damage(
            self.robot,
            HIT_DAMAGE,
            0.0,
            -1,
            self.fire,
            Float3::default(),
        );
        let Ok(queue) = spring::get_unit_commands(self.robot, QUEUE_READ) else {
            return fail("could not read the robot's orders");
        };
        let ids: alloc::vec::Vec<i32> = queue.iter().map(|command| command.cmd_id).collect();
        let moves = ids.iter().filter(|id| **id == spring::cmd::MOVE).count();
        let assault_kept = ids.contains(&spring::cmd::FIGHT);
        if ids.first() != Some(&spring::cmd::MOVE) || moves != 1 || !assault_kept {
            return Step::Fail(alloc::format!(
                "after {what} the orders are {ids:?}, want one MOVE in front of the FIGHT"
            ));
        }
        Step::Wait
    }
}

const QUEUE_READ: u32 = 8;
