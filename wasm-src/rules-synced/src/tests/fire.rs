use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};

pub struct Burning {
    field: i32,
    field_at: Float3,
    victim: i32,
    park: Float3,
    entry_health: f32,
    settled_health: f32,
}

impl Burning {
    pub fn new() -> Self {
        Self {
            field: -1,
            field_at: Float3::default(),
            victim: -1,
            park: Float3::default(),
            entry_health: 0.0,
            settled_health: 0.0,
        }
    }
}

impl Case for Burning {
    fn name(&self) -> &'static str {
        "fire"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            0 => self.enter(global),
            BURN_VERDICT_FRAME => self.check_burning(),
            SETTLE_FRAME => self.settle(),
            EXTINGUISH_VERDICT_FRAME => self.check_extinguished(),
            _ => Step::Wait,
        }
    }

    fn rest(&self) -> Vec<Rest> {
        alloc::vec![
            Rest::at(self.field, self.field_at, STILL, "the fire"),
            Rest::at(
                self.victim,
                self.park,
                STILL,
                "the Gravit back from the fire"
            ),
        ]
    }
}

const BURN_VERDICT_FRAME: i32 = 30;
const SETTLE_FRAME: i32 = 105;
const EXTINGUISH_VERDICT_FRAME: i32 = 135;

impl Burning {
    fn enter(&mut self, global: &Global) -> Step {
        let Some((field, victim)) = harness::arena(global, |arena| (arena.fire, arena.burning))
        else {
            return fail("no arena");
        };
        let (Some(park), Some(centre), Some(health)) = (
            harness::position(victim),
            harness::position(field),
            harness::health(victim),
        ) else {
            return fail("no position or health reading");
        };
        self.field = field;
        self.field_at = centre;
        self.victim = victim;
        self.park = park;
        self.entry_health = health;
        harness::halt(victim);
        harness::place(victim, centre);
        Step::Wait
    }

    fn check_burning(&mut self) -> Step {
        let Some(health) = harness::health(self.victim) else {
            return fail("lost the burning unit");
        };
        if health >= self.entry_health {
            return fail("standing in fire did no damage");
        }
        harness::place(self.victim, self.park);
        harness::heal(self.victim, self.entry_health);
        Step::Wait
    }

    fn settle(&mut self) -> Step {
        let Some(health) = harness::health(self.victim) else {
            return fail("lost the burning unit");
        };
        self.settled_health = health;
        Step::Wait
    }

    fn check_extinguished(&mut self) -> Step {
        let Some(health) = harness::health(self.victim) else {
            return fail("lost the burning unit");
        };
        if health < self.settled_health {
            return Step::Fail(alloc::format!(
                "fire kept burning after leaving it, {} lost",
                self.settled_health - health
            ));
        }
        Step::Pass
    }
}
