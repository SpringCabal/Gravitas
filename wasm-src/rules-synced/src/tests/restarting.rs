use alloc::vec::Vec;

use crate::state::Global;
use crate::tests::harness::{self, Case, Phase, Step, fail};
use crate::tests::{record, runner};
use crate::units;

pub struct Restarting {
    generation: u32,
    old_units: Vec<i32>,
    destroyed_mark: usize,
    stage: Stage,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Stage {
    KillOther,
    KillPlayer,
    GameOver,
    Rebuilt,
    OverAgain,
}

impl Restarting {
    pub fn new() -> Self {
        Self {
            generation: 0,
            old_units: Vec::new(),
            destroyed_mark: 0,
            stage: Stage::KillOther,
        }
    }
}

impl Case for Restarting {
    fn name(&self) -> &'static str {
        "restart"
    }

    fn phase(&self) -> Phase {
        Phase::Restart
    }

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        frame: i32,
    ) -> Step {
        match self.stage {
            Stage::KillOther => self.kill_other_gravit(global),
            Stage::KillPlayer if frame >= KILL_FRAME => self.kill(global),
            Stage::GameOver if global.outcome.get().is_some() => self.press(spring, global),
            Stage::Rebuilt if self.rebuilt(global) => self.check(spring, global),
            Stage::OverAgain if global.outcome.get().is_some() => self.check_over_again(global),
            _ => Step::Wait,
        }
    }
}

const KILL_FRAME: i32 = 4;

impl Restarting {
    fn kill_other_gravit(&mut self, global: &Global) -> Step {
        let Some(vaulter) = harness::arena(global, |arena| arena.vaulter) else {
            return fail("no arena");
        };
        harness::destroy(vaulter);
        self.stage = Stage::KillPlayer;
        Step::Wait
    }

    fn kill(&mut self, global: &Global) -> Step {
        if global.outcome.get().is_some() {
            return fail("losing a Gravit that is not the player's ended the game");
        }
        let Some((jumper, generation)) =
            harness::arena(global, |arena| (arena.jumper, arena.generation))
        else {
            return fail("no arena");
        };
        self.generation = generation;
        harness::destroy(jumper);
        self.stage = Stage::GameOver;
        Step::Wait
    }

    fn press(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) -> Step {
        if global.outcome.get() != Some(false) {
            return fail("losing the player unit won the game");
        }
        self.destroyed_mark = record::destroyed_count(global);
        self.old_units = spring
            .units()
            .get_all_units()
            .unwrap_or_default()
            .into_iter()
            .filter(|unit_id| !record::destroyed(global, *unit_id))
            .collect();
        harness::press_restart(spring);
        self.stage = Stage::Rebuilt;
        Step::Wait
    }

    fn rebuilt(&self, global: &Global) -> bool {
        runner::arena_ready(global)
            && harness::arena(global, |arena| arena.generation) != Some(self.generation)
    }

    fn check(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) -> Step {
        let kept = self
            .old_units
            .iter()
            .filter(|unit_id| !record::destroyed_since(global, self.destroyed_mark, **unit_id))
            .count();
        if self.old_units.is_empty() || kept != 0 {
            return Step::Fail(alloc::format!(
                "restart kept {kept} of the old level's {} units",
                self.old_units.len()
            ));
        }
        let Some((units, jumper)) = harness::arena(global, |arena| (arena.units(), arena.jumper))
        else {
            return fail("no arena after the restart");
        };
        let missing = units
            .iter()
            .filter(|unit_id| !units::valid(spring, **unit_id))
            .count();
        if missing != 0 {
            return Step::Fail(alloc::format!(
                "{missing} of {} units missing after the restart",
                units.len()
            ));
        }
        if global.outcome.get().is_some() {
            return fail("the restart left the game in its over state");
        }
        if global.player_unit.get() != Some(jumper) {
            return fail("the player does not drive the new arena's Gravit");
        }
        harness::destroy(jumper);
        self.stage = Stage::OverAgain;
        Step::Wait
    }

    fn check_over_again(&mut self, global: &Global) -> Step {
        if global.outcome.get() != Some(false) {
            return fail("dying after a restart won the game");
        }
        Step::Pass
    }
}
