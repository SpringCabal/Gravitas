use alloc::vec::Vec;

use crate::state::Global;
use crate::tests::harness::{self, Case, Phase, Step, fail};
use crate::tests::{record, runner};
use crate::units;

/// Losing the player's Gravit ends the game, and the HUD's restart button
/// (a round trip through the ui module) brings the arena back. The case
/// waits for each of those instead of counting frames.
pub struct Restarting {
    generation: u32,
    /// The units alive when restart was pressed, and the destroyed-unit mark
    /// then.
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

    /// A restart rebuilds the whole arena.
    fn phase(&self) -> Phase {
        Phase::Restart
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match self.stage {
            Stage::KillOther => self.kill_other_gravit(global),
            Stage::KillPlayer if frame >= KILL_FRAME => self.kill(global),
            Stage::GameOver if global.outcome.get().is_some() => self.press(global),
            Stage::Rebuilt if self.rebuilt(global) => self.check(global),
            Stage::OverAgain if global.outcome.get().is_some() => self.check_over_again(global),
            _ => Step::Wait,
        }
    }
}

/// A few frames after the other Gravit died, the game must still be on.
const KILL_FRAME: i32 = 4;

impl Restarting {
    fn kill_other_gravit(&mut self, global: &Global) -> Step {
        let Some(vaulter) = harness::arena(global, |arena| arena.vaulter) else {
            return fail("no arena");
        };
        destroy(vaulter);
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
        destroy(jumper);
        self.stage = Stage::GameOver;
        Step::Wait
    }

    fn press(&mut self, global: &Global) -> Step {
        if global.outcome.get() != Some(false) {
            return fail("losing the player unit won the game");
        }
        // Ids come back after a restart, so look for each unit's UnitDestroyed
        // (a dying unit already had its own).
        self.destroyed_mark = record::destroyed_count(global);
        self.old_units = spring::get_all_units()
            .unwrap_or_default()
            .into_iter()
            .filter(|unit_id| !record::destroyed(global, *unit_id))
            .collect();
        harness::press_restart();
        self.stage = Stage::Rebuilt;
        Step::Wait
    }

    fn rebuilt(&self, global: &Global) -> bool {
        runner::arena_ready(global)
            && harness::arena(global, |arena| arena.generation) != Some(self.generation)
    }

    fn check(&mut self, global: &Global) -> Step {
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
            .filter(|unit_id| !units::valid(**unit_id))
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
        destroy(jumper);
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

pub fn destroy(unit_id: i32) {
    let _ = spring::destroy_unit(
        unit_id,
        spring::generated::owned::unit_control::DestroyUnitOptions {
            selfd: false,
            reclaimed: false,
            attacker_id: -1,
            recycle_id: false,
        },
    );
}
