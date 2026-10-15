use core::cell::RefCell;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::diagnostics;
use crate::state::Global;
use crate::tests::arena;
use crate::tests::harness::{Case, PHASES, Phase, Step};

pub fn stage_level(global: &Global) -> bool {
    if global.tests.real_level.get() {
        return false;
    }
    let generation = global
        .tests
        .arena
        .get()
        .map_or(0, |arena| arena.generation + 1);
    arena::build(generation, |built| {
        crate::with_global(|global| match built {
            Some(built) => {
                diagnostics::info(&alloc::format!(
                    "E2E arena at {:.0} {:.0}",
                    built.centre.x,
                    built.centre.z
                ));
                global.tests.arena.set(Some(built));
                global.tests.pending_link.set(true);
            }
            None => {
                global.tests.arena.set(None);
                spring::log_warning!(
                    crate::diagnostics::LOG_SECTION,
                    "E2E arena could not be built"
                );
            }
        })
    });
    true
}

pub fn arena_ready(global: &Global) -> bool {
    global.tests.arena.get().is_some() && !global.tests.pending_link.get()
}

#[derive(Default)]
pub struct Runner {
    state: RefCell<State>,
}

#[derive(Default)]
struct State {
    entries: Vec<Entry>,
    booted: bool,
    done: bool,
}

struct Entry {
    case: Box<dyn Case>,
    started: Option<i32>,
    verdict: Option<bool>,
}

const CASE_TIMEOUT: i32 = 600;
const ARENA_TIMEOUT: i32 = 300;

impl Runner {
    pub fn game_frame(&self, global: &Global, frame: i32) {
        link_arena(global);
        if !self.ready(global, frame) {
            return;
        }
        let mut entries = core::mem::take(&mut self.state.borrow_mut().entries);
        if let Some(phase) = current_phase(&entries) {
            for entry in entries
                .iter_mut()
                .filter(|entry| entry.case.phase() == phase)
            {
                step(entry, global, frame);
            }
        }
        let finished = current_phase(&entries).is_none();
        self.state.borrow_mut().entries = entries;
        if finished {
            self.finish();
        }
    }

    fn ready(&self, global: &Global, frame: i32) -> bool {
        let mut state = self.state.borrow_mut();
        if state.done {
            return false;
        }
        if state.booted {
            return true;
        }
        if arena_ready(global) {
            state.booted = true;
            state.entries = cases();
            return true;
        }
        if frame >= ARENA_TIMEOUT {
            state.entries = cases();
            for entry in &mut state.entries {
                entry.verdict = Some(false);
                report(entry.case.name(), Step::Fail("no arena".into()));
            }
            drop(state);
            self.finish();
        }
        false
    }

    fn finish(&self) {
        {
            let mut state = self.state.borrow_mut();
            state.done = true;
            let passed = state
                .entries
                .iter()
                .filter(|entry| entry.verdict == Some(true))
                .count();
            let failed = state.entries.len() - passed;
            diagnostics::info(&alloc::format!(
                "E2E summary passed={passed} failed={failed}"
            ));
        }
        crate::tests::harness::send_to_ui(gravitas_shared::e2e::Request::Done);
    }
}

fn current_phase(entries: &[Entry]) -> Option<Phase> {
    PHASES.into_iter().find(|phase| {
        entries
            .iter()
            .any(|entry| entry.case.phase() == *phase && entry.verdict.is_none())
    })
}

pub fn use_real_level(global: &Global) {
    global.tests.real_level.set(true);
}

fn link_arena(global: &Global) {
    if !global.tests.pending_link.replace(false) {
        return;
    }
    let Some(arena) = global.tests.arena.get() else {
        return;
    };
    let mut plates = global.plates.access_mut();
    for (plate, gate) in arena.links() {
        plates.simple_link(plate, gate);
    }
    plates.enable_link_checks();
}

fn cases() -> Vec<Entry> {
    let cases: Vec<Box<dyn Case>> = alloc::vec![
        Box::new(crate::tests::spawn::Spawned::new()),
        Box::new(crate::tests::plate::PlateAndGate::new()),
        Box::new(crate::tests::impulse::PushAndPull::new()),
        Box::new(crate::tests::invulnerable::Invulnerable::new()),
        Box::new(crate::tests::fire::Burning::new()),
        Box::new(crate::tests::flee::Fleeing::new()),
        Box::new(crate::tests::shoot::Shooting::new()),
        Box::new(crate::tests::death::Dying::new()),
        Box::new(crate::tests::electrafi::Shocking::new()),
        Box::new(crate::tests::jump::Jumping::new()),
        Box::new(crate::tests::input::Input::new()),
        Box::new(crate::tests::falling::Falling::new()),
        Box::new(crate::tests::collision::Collision::new()),
        Box::new(crate::tests::barrel::Barrel::new()),
        Box::new(crate::tests::terrain::Terrain::new()),
        Box::new(crate::tests::respawn::Respawning::new()),
        Box::new(crate::tests::visuals::Visuals::new()),
        Box::new(crate::tests::restarting::Restarting::new()),
        Box::new(crate::tests::level::RealLevel::new()),
    ];
    cases
        .into_iter()
        .map(|case| Entry {
            case,
            started: None,
            verdict: None,
        })
        .collect()
}

fn step(entry: &mut Entry, global: &Global, frame: i32) {
    if entry.verdict.is_some() {
        return;
    }
    let started = *entry.started.get_or_insert(frame);
    let local = frame - started;
    let name = entry.case.name();
    let outcome = match entry.case.step(global, local) {
        Step::Wait if local < CASE_TIMEOUT => return,
        Step::Wait => Step::Fail("timed out".into()),
        outcome => outcome,
    };
    let passed = matches!(outcome, Step::Pass);
    if passed {
        global.tests.rests.borrow_mut().extend(entry.case.rest());
    }
    entry.verdict = Some(passed);
    report(name, outcome);
}

fn report(name: &str, outcome: Step) {
    match outcome {
        Step::Pass => diagnostics::info(&alloc::format!("E2E case {name} PASS")),
        Step::Fail(reason) => diagnostics::info(&alloc::format!("E2E case {name} FAIL {reason}")),
        Step::Wait => {}
    }
}
