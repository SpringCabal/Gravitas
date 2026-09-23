use core::cell::{Cell, RefCell};

use alloc::boxed::Box;
use alloc::vec::Vec;

use crate::diagnostics;
use crate::state::Global;
use crate::tests::arena::{self, Arena};
use crate::tests::harness::{Case, Step};

pub fn stage_level(_global: &Global) -> bool {
    let Some(built) = arena::build() else {
        diagnostics::warning("E2E arena could not be built");
        return true;
    };
    ARENA.with(|arena| *arena.borrow_mut() = Some(built));
    PENDING_LINK.with(|pending| pending.set(true));
    true
}

fn link_arena(global: &Global) {
    if !PENDING_LINK.with(|pending| pending.replace(false)) {
        return;
    }
    let Some((plate, gate)) = with_arena(|arena| (arena.plate, arena.gate)) else {
        return;
    };
    let mut plates = global.plates.access_mut();
    plates.simple_link(plate, gate);
    plates.enable_link_checks();
}

pub fn game_frame(global: &Global, frame: i32) {
    link_arena(global);
    let Some(local) = begin(frame) else {
        return;
    };
    super::harness::pump_held_input(global);
    let mut cases = take_cases();
    for case in &mut cases {
        step(case, global, local);
    }
    let finished = cases.iter().all(|case| case.verdict.is_some());
    put_cases(cases);
    if finished || local >= SUITE_TIMEOUT {
        finish();
    }
}

pub fn with_arena<R>(function: impl FnOnce(&Arena) -> R) -> Option<R> {
    ARENA.with(|arena| arena.borrow().as_ref().map(function))
}

thread_local! {
    static PENDING_LINK: Cell<bool> = const { Cell::new(false) };
}

const FIRST_CASE_FRAME: i32 = 6;
const SUITE_TIMEOUT: i32 = 240;

struct Entry {
    case: Box<dyn Case>,
    verdict: Option<bool>,
}

struct Runner {
    entries: Vec<Entry>,
    started: i32,
    booted: bool,
    done: bool,
}

thread_local! {
    static ARENA: RefCell<Option<Arena>> = const { RefCell::new(None) };
    static RUNNER: RefCell<Runner> = const {
        RefCell::new(Runner {
            entries: Vec::new(),
            started: 0,
            booted: false,
            done: false,
        })
    };
}

fn begin(frame: i32) -> Option<i32> {
    if frame < FIRST_CASE_FRAME {
        return None;
    }
    with_runner(|runner| {
        if runner.done {
            return None;
        }
        if !runner.booted {
            runner.booted = true;
            runner.started = frame;
            runner.entries = cases();
        }
        Some(frame - runner.started)
    })
    .flatten()
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

fn finish() {
    let _ = with_runner(|runner| {
        runner.done = true;
        let passed = runner
            .entries
            .iter()
            .filter(|entry| entry.verdict == Some(true))
            .count();
        let failed = runner.entries.len() - passed;
        diagnostics::info(&alloc::format!(
            "E2E summary passed={passed} failed={failed}"
        ));
    });
    let _ = spring::send_to_unsynced("GRAVITAS_SHOT|done");
}

fn take_cases() -> Vec<Entry> {
    with_runner(|runner| core::mem::take(&mut runner.entries)).unwrap_or_default()
}

fn put_cases(entries: Vec<Entry>) {
    let _ = with_runner(|runner| {
        if runner.entries.is_empty() {
            runner.entries = entries;
        }
    });
}

fn with_runner<R>(function: impl FnOnce(&mut Runner) -> R) -> Option<R> {
    RUNNER.with(|runner| match runner.try_borrow_mut() {
        Ok(mut runner) => Some(function(&mut runner)),
        Err(_) => None,
    })
}
