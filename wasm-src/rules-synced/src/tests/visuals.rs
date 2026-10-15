use alloc::string::String;
use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::tests::harness::{self, Case, Phase, Rest, Step, fail};
use crate::tests::record;

pub struct Visuals {
    positions: Vec<(i32, Float3)>,
    misplaced: Option<String>,
    asked: bool,
}

impl Visuals {
    pub fn new() -> Self {
        Self {
            positions: Vec::new(),
            misplaced: None,
            asked: false,
        }
    }
}

impl Case for Visuals {
    fn name(&self) -> &'static str {
        "visuals"
    }

    fn phase(&self) -> Phase {
        Phase::Scene
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        if frame == 0 {
            return self.look(global);
        }
        if self.asked {
            return if record::shot_taken(global, ARENA_SHOT) {
                self.check()
            } else {
                Step::Wait
            };
        }
        let Some(units) = harness::arena(global, |arena| arena.units()) else {
            return fail("no arena");
        };
        let now = positions(&units);
        let moving = moving(&self.positions, &now);
        self.positions = now;
        match moving {
            _ if frame < CAMERA_FRAMES => Step::Wait,
            None => self.shoot(global),
            Some(_) if frame < SETTLE_LIMIT => Step::Wait,
            Some(unit_id) => Step::Fail(alloc::format!(
                "{} was still moving {SETTLE_LIMIT} frames after the last case",
                describe(unit_id)
            )),
        }
    }
}

const CAMERA_FRAMES: i32 = 15;
const SETTLE_LIMIT: i32 = 300;
const STILL_PER_FRAME: f32 = 0.05;
const ARENA_SHOT: &str = "arena";
const CAMERA_HEIGHT: f32 = 1800.0;
const AT_REST: f32 = 1.0;

impl Visuals {
    fn look(&mut self, global: &Global) -> Step {
        let Some(centre) = harness::arena(global, |arena| arena.centre) else {
            return fail("no arena");
        };
        harness::look(centre, CAMERA_HEIGHT);
        Step::Wait
    }

    fn shoot(&mut self, global: &Global) -> Step {
        self.misplaced = misplaced(global);
        crate::diagnostics::info(&alloc::format!(
            "E2E scene: {} units checked against where their cases left them",
            global.tests.rests.borrow().len()
        ));
        harness::capture(ARENA_SHOT);
        self.asked = true;
        Step::Wait
    }

    fn check(&mut self) -> Step {
        if let Some(reason) = self.misplaced.take() {
            return Step::Fail(reason);
        }
        for (unit_id, at) in &self.positions {
            let now = harness::position(*unit_id);
            if harness::moved(Some(*at), now, AT_REST) {
                return Step::Fail(alloc::format!(
                    "{} moved while the arena was shot",
                    describe(*unit_id)
                ));
            }
        }
        Step::Pass
    }
}

fn positions(units: &[i32]) -> Vec<(i32, Float3)> {
    units
        .iter()
        .filter_map(|unit_id| harness::position(*unit_id).map(|at| (*unit_id, at)))
        .collect()
}

fn moving(before: &[(i32, Float3)], now: &[(i32, Float3)]) -> Option<i32> {
    if before.len() != now.len() {
        return Some(now.first().or(before.first())?.0);
    }
    before
        .iter()
        .zip(now)
        .find(|((_, from), (_, to))| harness::distance(*from, *to) > STILL_PER_FRAME)
        .map(|((unit_id, _), _)| *unit_id)
}

fn describe(unit_id: i32) -> String {
    let name = crate::units::def_of(unit_id)
        .map(|def_id| crate::defs::get().unit(def_id).name.clone())
        .unwrap_or_default();
    alloc::format!("{name} (unit {unit_id})")
}

fn misplaced(global: &Global) -> Option<String> {
    let rests = global.tests.rests.borrow();
    for rest in rests.iter() {
        match *rest {
            Rest::At {
                unit,
                at,
                within,
                what,
            } => {
                let Some(position) = harness::position(unit) else {
                    return Some(alloc::format!("{what} (unit {unit}) is gone"));
                };
                let off = harness::planar_distance(at, position);
                if off > within {
                    return Some(alloc::format!(
                        "{what} (unit {unit}) is {off:.0} elmos from where its case \
                         left it ({:.0} {:.0}), {within:.0} allowed",
                        at.x,
                        at.z
                    ));
                }
            }
            Rest::Gone { unit, what } => {
                if !record::destroyed(global, unit) {
                    return Some(alloc::format!("{what} (unit {unit}) was never destroyed"));
                }
            }
        }
    }
    if rests.is_empty() {
        return Some(String::from("no case reported where it left its units"));
    }
    None
}
