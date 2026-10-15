use alloc::vec::Vec;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};

pub struct Barrel {
    barrel: i32,
    bystander: i32,
    bystander_at: spring::types::Float3,
    bystander_health: f32,
    bystander_killed: bool,
}

impl Barrel {
    pub fn new() -> Self {
        Self {
            barrel: -1,
            bystander: -1,
            bystander_at: spring::types::Float3::default(),
            bystander_health: 0.0,
            bystander_killed: false,
        }
    }
}

impl Case for Barrel {
    fn name(&self) -> &'static str {
        "barrel"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            BLAST_FRAME => self.blast(global),
            VERDICT_FRAME => self.check(global),
            _ => Step::Wait,
        }
    }

    fn rest(&self) -> Vec<Rest> {
        let barrel = Rest::Gone {
            unit: self.barrel,
            what: "the exploded barrel",
        };
        let bystander = if !self.bystander_killed {
            Rest::at(
                self.bystander,
                self.bystander_at,
                STILL,
                "the robot by the barrel",
            )
        } else {
            Rest::Gone {
                unit: self.bystander,
                what: "the robot the barrel killed",
            }
        };
        alloc::vec![barrel, bystander]
    }
}

const BLAST_FRAME: i32 = 40;
const VERDICT_FRAME: i32 = 50;

impl Barrel {
    fn blast(&mut self, global: &Global) -> Step {
        let Some((barrel, bystander)) =
            harness::arena(global, |arena| (arena.barrel, arena.bystander))
        else {
            return fail("no arena");
        };
        let (Some(health), Some(bystander_at)) =
            (harness::health(bystander), harness::position(bystander))
        else {
            return fail("no health or position reading");
        };
        self.barrel = barrel;
        self.bystander = bystander;
        self.bystander_at = bystander_at;
        self.bystander_health = health;
        harness::destroy(barrel);
        Step::Wait
    }

    fn check(&mut self, global: &Global) -> Step {
        self.bystander_killed = crate::tests::record::destroyed(global, self.bystander);
        let Some(health) = harness::health(self.bystander) else {
            return Step::Pass;
        };
        if health >= self.bystander_health {
            return fail("an exploding barrel did not hurt the unit beside it");
        }
        Step::Pass
    }
}
