use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};

pub struct Barrel {
    bystander: i32,
    bystander_health: f32,
}

impl Barrel {
    pub fn new() -> Self {
        Self {
            bystander: -1,
            bystander_health: 0.0,
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
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
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
        let Some(health) = harness::health(bystander) else {
            return fail("no health reading");
        };
        self.bystander = bystander;
        self.bystander_health = health;
        let _ = spring::destroy_unit(
            barrel,
            spring::generated::owned::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: false,
                attacker_id: -1,
                recycle_id: false,
            },
        );
        Step::Wait
    }

    fn check(&mut self) -> Step {
        let Some(health) = harness::health(self.bystander) else {
            return Step::Pass;
        };
        if health >= self.bystander_health {
            return fail("an exploding barrel did not hurt the unit beside it");
        }
        Step::Pass
    }
}
