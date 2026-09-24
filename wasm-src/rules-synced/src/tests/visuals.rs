use crate::state::Global;
use crate::tests::harness::{self, Case, Step, fail};
use crate::tests::record;

/// Point the camera at the arena and have the ui module take the screenshot
/// the shots suite compares; the ui module reports it taken.
pub struct Visuals;

impl Visuals {
    pub fn new() -> Self {
        Self
    }
}

impl Case for Visuals {
    fn name(&self) -> &'static str {
        "visuals"
    }

    fn step(&mut self, global: &Global, frame: i32) -> Step {
        match frame {
            LOOK_FRAME => self.look(global),
            SHOT_FRAME => {
                harness::capture(ARENA_SHOT);
                Step::Wait
            }
            VERDICT_FRAME => {
                if record::shot_taken(global, ARENA_SHOT) {
                    Step::Pass
                } else {
                    fail("the ui module did not take the arena screenshot")
                }
            }
            _ => Step::Wait,
        }
    }
}

const LOOK_FRAME: i32 = 1;
const SHOT_FRAME: i32 = 75;
const VERDICT_FRAME: i32 = 120;
const ARENA_SHOT: &str = "arena";
const CAMERA_HEIGHT: f32 = 1500.0;

impl Visuals {
    fn look(&mut self, global: &Global) -> Step {
        let Some(centre) = harness::arena(global, |arena| arena.centre) else {
            return fail("no arena");
        };
        harness::look(centre, CAMERA_HEIGHT);
        Step::Wait
    }
}
