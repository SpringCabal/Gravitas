use alloc::vec::Vec;

use spring::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Rest, STILL, Step, fail};

pub struct Invulnerable {
    protected: i32,
    exposed: i32,
    protected_health: f32,
    exposed_health: f32,
    protected_at: Float3,
    exposed_at: Float3,
}

impl Invulnerable {
    pub fn new() -> Self {
        Self {
            protected: -1,
            exposed: -1,
            protected_health: 0.0,
            exposed_health: 0.0,
            protected_at: Float3::default(),
            exposed_at: Float3::default(),
        }
    }
}

impl Case for Invulnerable {
    fn name(&self) -> &'static str {
        "invulnerable"
    }

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        match frame {
            STRIKE_FRAME => self.strike(spring, global),
            VERDICT_FRAME => self.check(spring),
            _ => Step::Wait,
        }
    }

    fn rest(&self, _spring: &spring::Spring<'_, spring::RulesSynced>) -> Vec<Rest> {
        alloc::vec![
            Rest::at(
                self.protected,
                self.protected_at,
                STILL,
                "the invulnerable wall"
            ),
            Rest::at(self.exposed, self.exposed_at, STILL, "the struck robot"),
        ]
    }
}

const STRIKE_FRAME: i32 = 2;
const VERDICT_FRAME: i32 = 8;
const DAMAGE: f32 = 10.0;
const NO_WEAPON: i32 = -1;

impl Invulnerable {
    fn strike(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
    ) -> Step {
        let Some((protected, exposed)) =
            harness::arena(global, |arena| (arena.protected, arena.exposed))
        else {
            return fail("no arena");
        };
        let (Some(protected_health), Some(exposed_health)) = (
            harness::health(spring, protected),
            harness::health(spring, exposed),
        ) else {
            return fail("no health readings");
        };
        let (Some(protected_at), Some(exposed_at)) = (
            harness::position(spring, protected),
            harness::position(spring, exposed),
        ) else {
            return fail("no positions");
        };
        self.protected_at = protected_at;
        self.exposed_at = exposed_at;
        self.protected = protected;
        self.exposed = exposed;
        self.protected_health = protected_health;
        self.exposed_health = exposed_health;
        damage(protected, exposed);
        damage(exposed, protected);
        Step::Wait
    }

    fn check(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>) -> Step {
        let (Some(protected_health), Some(exposed_health)) = (
            harness::health(spring, self.protected),
            harness::health(spring, self.exposed),
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
    let _ = crate::with_addon_context(|ctx| {
        ctx.commands().add_unit_damage(
            unit_id,
            DAMAGE,
            0.0,
            NO_WEAPON,
            attacker_id,
            spring::Float3::default(),
        )
    });
}
