use alloc::format;

use crate::level;
use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};
use crate::tests::{record, runner};
use crate::units;

/// The real level, after every other case: restart into it and check that it
/// spawned whole, linked its plates, handed the player its Gravit, and gave
/// the hazards their scenario sizes before any gadget's UnitCreated.
pub struct RealLevel;

impl RealLevel {
    pub fn new() -> Self {
        Self
    }
}

impl Case for RealLevel {
    fn name(&self) -> &'static str {
        "level"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            RESTART_FRAME => {
                runner::use_real_level(global);
                harness::press_restart();
                Step::Wait
            }
            VERDICT_FRAME => check(global),
            _ => Step::Wait,
        }
    }
}

/// After the restart case's last check.
const RESTART_FRAME: i32 = 200;
/// The restart goes through the ui module and back; then restart, spawn and
/// link take a frame each.
const VERDICT_FRAME: i32 = 212;
const PLAYER_DEF: &str = "gravit";

fn check(global: &Global) -> Step {
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
    if !boss.is_some_and(units::valid) {
        return fail("the boss is missing");
    }
    if global.outcome.get().is_some() {
        return fail("the level ended as soon as it started");
    }
    for spawn in level::SPAWNS {
        let Some(&(_, wanted)) = spawn.rules.iter().find(|(name, _)| {
            *name == crate::fire::SIZE_PARAM || *name == crate::electrafi::SIZE_PARAM
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
