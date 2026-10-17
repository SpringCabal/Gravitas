use alloc::format;

use crate::gadgets::level::data as level;
use crate::state::{Global, Phase as LevelPhase};
use crate::tests::harness::{self, Case, Phase, Step, fail};
use crate::tests::{record, runner};
use crate::units;

pub struct RealLevel {
    restarted: bool,
}

impl RealLevel {
    pub fn new() -> Self {
        Self { restarted: false }
    }
}

impl Case for RealLevel {
    fn name(&self) -> &'static str {
        "level"
    }

    fn phase(&self) -> Phase {
        Phase::Level
    }

    fn step(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        global: &Global,
        _frame: i32,
    ) -> Step {
        if !self.restarted {
            runner::use_real_level(global);
            harness::press_restart(spring);
            self.restarted = true;
            return Step::Wait;
        }
        if level_up(global) {
            check(spring, global)
        } else {
            Step::Wait
        }
    }
}

fn level_up(global: &Global) -> bool {
    let level = global.level.access();
    level.phase == LevelPhase::Running && !level.spawned.is_empty()
}
const PLAYER_DEF: &str = "gravit";

fn check(spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global) -> Step {
    let (spawned, link_failures, player_spawn, boss) = {
        let level = global.level.access();
        let player_spawn = level::SPAWNS
            .iter()
            .find(|spawn| spawn.def_name == PLAYER_DEF)
            .and_then(|spawn| level.unit_of(spawn.id));
        (
            level.spawned.clone(),
            level.link_failures,
            player_spawn,
            level.unit_of(level::BOSS_UNIT),
        )
    };
    if spawned.len() != level::SPAWNS.len() {
        return Step::Fail(format!(
            "{} of {} level units spawned",
            spawned.len(),
            level::SPAWNS.len()
        ));
    }
    if link_failures != 0 {
        return Step::Fail(format!("{link_failures} plate link(s) failed"));
    }
    if player_spawn.is_none() || global.player_unit.get() != player_spawn {
        return fail("the player does not drive the level's Gravit");
    }
    if !boss.is_some_and(|id| units::valid(spring, id)) {
        return fail("the boss is missing");
    }
    if global.outcome.get().is_some() {
        return fail("the level ended as soon as it started");
    }
    for spawn in level::SPAWNS {
        let Some(&(_, wanted)) = spawn.rules.iter().find(|(name, _)| {
            *name == crate::gadgets::fire::behavior::SIZE_PARAM
                || *name == crate::gadgets::electrafi::behavior::SIZE_PARAM
        }) else {
            continue;
        };
        let Some(&(_, unit_id)) = spawned.iter().find(|(id, _)| *id == spawn.id) else {
            continue;
        };
        let seen = record::hazard_size_at_creation(global, unit_id).flatten();
        if seen != Some(wanted) {
            return Step::Fail(format!(
                "{} {} was {seen:?} in UnitCreated, the scenario says {wanted}",
                spawn.def_name, spawn.id
            ));
        }
    }
    Step::Pass
}
