use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};

pub struct Invulnerable {
    protected: i32,
    exposed: i32,
    protected_health: f32,
    exposed_health: f32,
}

impl Invulnerable {
    pub fn new() -> Self {
        Self {
            protected: -1,
            exposed: -1,
            protected_health: 0.0,
            exposed_health: 0.0,
        }
    }
}

impl Case for Invulnerable {
    fn name(&self) -> &'static str {
        "invulnerable"
    }

    fn step(&mut self, _global: &Global, frame: i32) -> Step {
        match frame {
            STRIKE_FRAME => self.strike(),
            VERDICT_FRAME => self.check(),
            _ => Step::Wait,
        }
    }
}

const STRIKE_FRAME: i32 = 2;
const VERDICT_FRAME: i32 = 8;
const DAMAGE: f32 = 10.0;
const NO_WEAPON: i32 = -1;

impl Invulnerable {
    fn strike(&mut self) -> Step {
        let Some((protected, exposed)) = harness::arena(|arena| (arena.protected, arena.exposed))
        else {
            return fail("no arena");
        };
        let (Some(protected_health), Some(exposed_health)) =
            (harness::health(protected), harness::health(exposed))
        else {
            return fail("no health readings");
        };
        self.protected = protected;
        self.exposed = exposed;
        self.protected_health = protected_health;
        self.exposed_health = exposed_health;
        damage(protected, exposed);
        damage(exposed, protected);
        Step::Wait
    }

    fn check(&mut self) -> Step {
        let (Some(protected_health), Some(exposed_health)) = (
            harness::health(self.protected),
            harness::health(self.exposed),
        ) else {
            return fail("no health readings");
        };
        if protected_health < self.protected_health {
            return Step::Fail(alloc::format!(
                "invulnerable wall lost {} health",
                self.protected_health - protected_health
            ));
        }
        if exposed_health >= self.exposed_health {
            return fail("a normal unit took no damage");
        }
        Step::Pass
    }
}

fn damage(unit_id: i32, attacker_id: i32) {
    let _ = spring::add_unit_damage(
        unit_id,
        DAMAGE,
        0.0,
        NO_WEAPON,
        attacker_id,
        spring::types::Float3::default(),
    );
}
