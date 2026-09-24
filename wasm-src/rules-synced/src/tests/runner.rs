use core::cell::RefCell;

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::diagnostics;
use crate::state::Global;
use crate::tests::arena;
use crate::tests::harness::{Case, Step};

/// Build the e2e arena instead of the level. Returns true: the level is
/// replaced even if the arena could not be built (the cases then fail).
pub fn stage_level(global: &Global) -> bool {
    match arena::build() {
        Some(built) => {
            global.tests.arena.set(Some(built));
            global.tests.pending_link.set(true);
        }
        None => diagnostics::warning("E2E arena could not be built"),
    }
    true
}

/// The e2e cases and their verdicts, owned by the tests gadget.
#[derive(Default)]
pub struct Runner {
    state: RefCell<State>,
}

#[derive(Default)]
struct State {
    entries: Vec<Entry>,
    started: i32,
    booted: bool,
    done: bool,
}

struct Entry {
    case: Box<dyn Case>,
    verdict: Option<bool>,
}

const FIRST_CASE_FRAME: i32 = 6;
const SUITE_TIMEOUT: i32 = 240;

impl Runner {
    pub fn game_frame(&self, global: &Global, frame: i32) {
        link_arena(global);
        let Some(local) = self.begin(frame) else {
            return;
        };
        // The cases create, destroy and damage units, which raises callins in
        // this gadget: no borrow of the state is held while they run.
        let mut entries = core::mem::take(&mut self.state.borrow_mut().entries);
        for entry in &mut entries {
            step(entry, global, local);
        }
        let finished = entries.iter().all(|entry| entry.verdict.is_some());
        self.state.borrow_mut().entries = entries;
        if finished || local >= SUITE_TIMEOUT {
            self.finish();
        }
    }

    fn begin(&self, frame: i32) -> Option<i32> {
        if frame < FIRST_CASE_FRAME {
            return None;
        }
        let mut state = self.state.borrow_mut();
        if state.done {
            return None;
        }
        if !state.booted {
            state.booted = true;
            state.started = frame;
            state.entries = cases();
        }
        Some(frame - state.started)
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
        let _ = spring::send_to_unsynced("GRAVITAS_SHOT|done");
    }
}

/// Link the arena's plate and gate on the frame after it was built.
fn link_arena(global: &Global) {
    if !global.tests.pending_link.replace(false) {
        return;
    }
    let Some(arena) = global.tests.arena.get() else {
        return;
    };
    let mut plates = global.plates.access_mut();
    plates.simple_link(arena.plate, arena.gate);
    plates.enable_link_checks();
}

fn cases() -> Vec<Entry> {
    let cases: Vec<Box<dyn Case>> = alloc::vec![
        Box::new(crate::tests::spawn::Spawned::new()),
        Box::new(crate::tests::plate::PlateAndGate::new()),
        Box::new(crate::tests::impulse::PushAndPull::new()),
        Box::new(crate::tests::invulnerable::Invulnerable::new()),
        Box::new(crate::tests::fire::Burning::new()),
        Box::new(crate::tests::electrafi::Shocking::new()),
        Box::new(crate::tests::jump::Jumping::new()),
        Box::new(crate::tests::input::Input::new()),
        Box::new(crate::tests::falling::Falling::new()),
        Box::new(crate::tests::barrel::Barrel::new()),
        Box::new(crate::tests::terrain::Terrain::new()),
        Box::new(crate::tests::respawn::Respawning::new()),
        Box::new(crate::tests::restarting::Restarting::new()),
        Box::new(crate::tests::visuals::Visuals::new()),
    ];
    cases
        .into_iter()
        .map(|case| Entry {
            case,
            verdict: None,
        })
        .collect()
}

fn step(entry: &mut Entry, global: &Global, local: i32) {
    if entry.verdict.is_some() {
        return;
    }
    let name = entry.case.name();
    match entry.case.step(global, local) {
        Step::Wait if local < SUITE_TIMEOUT => {}
        Step::Wait => {
            entry.verdict = Some(false);
            diagnostics::info(&alloc::format!("E2E case {name} FAIL timed out"));
        }
        Step::Pass => {
            entry.verdict = Some(true);
            diagnostics::info(&alloc::format!("E2E case {name} PASS"));
        }
        Step::Fail(reason) => {
            entry.verdict = Some(false);
            diagnostics::info(&alloc::format!("E2E case {name} FAIL {reason}"));
        }
    }
}
