use alloc::vec::Vec;

use spring::types::Float3;

use crate::units;

pub struct Respawns {
    watched: Vec<Watched>,
}

struct Watched {
    unit_id: i32,
    unit_def_id: i32,
    team_id: i32,
    origin: Float3,
    last_position: Float3,
    last_moved_frame: i32,
}

pub struct Rebirth {
    pub unit_id: i32,
    pub unit_def_id: i32,
    pub team_id: i32,
    pub origin: Float3,
}

impl Respawns {
    pub fn new() -> Self {
        Self {
            watched: Vec::new(),
        }
    }

    pub fn scan(&mut self, frame: i32) {
        let Ok(units) = spring::get_all_units() else {
            return;
        };
        for unit_id in units {
            if !is_respawnable(unit_id) {
                continue;
            }
            let (Some(unit_def_id), Ok(team_id)) = (
                units::def_of(unit_id),
                spring::units_info::get_unit_team(unit_id),
            ) else {
                continue;
            };
            self.adopt(unit_id, unit_def_id, team_id, frame);
        }
    }

    pub fn adopt(&mut self, unit_id: i32, unit_def_id: i32, team_id: i32, frame: i32) {
        if self
            .watched
            .iter()
            .any(|watched| watched.unit_id == unit_id)
        {
            return;
        }
        let Some(origin) = units::position(unit_id) else {
            return;
        };
        self.watched.push(Watched {
            unit_id,
            unit_def_id,
            team_id,
            origin,
            last_position: origin,
            last_moved_frame: frame,
        });
    }

    pub fn destroyed(&mut self, unit_id: i32) -> Option<Rebirth> {
        let index = self
            .watched
            .iter()
            .position(|watched| watched.unit_id == unit_id)?;
        let watched = self.watched.swap_remove(index);
        Some(Rebirth {
            unit_id: watched.unit_id,
            unit_def_id: watched.unit_def_id,
            team_id: watched.team_id,
            origin: watched.origin,
        })
    }

    pub fn update(&mut self, frame: i32) -> Vec<Rebirth> {
        let mut stuck = Vec::new();
        for watched in &mut self.watched {
            let Some(position) = units::position(watched.unit_id) else {
                stuck.push(watched.unit_id);
                continue;
            };
            if displaced(position, watched.last_position) {
                watched.last_position = position;
                watched.last_moved_frame = frame;
                continue;
            }
            if !displaced(position, watched.origin) {
                continue;
            }
            let idle = frame - watched.last_moved_frame;
            if idle > LONG_IDLE_FRAMES
                || (idle > SHORT_IDLE_FRAMES && position.y > watched.origin.y + STUCK_HEIGHT)
            {
                stuck.push(watched.unit_id);
            }
        }
        stuck
            .into_iter()
            .filter_map(|unit_id| self.destroyed(unit_id))
            .collect()
    }
}

pub fn rebirth(rebirth: &Rebirth) -> Option<i32> {
    if units::valid(rebirth.unit_id) {
        let _ = spring::destroy_unit(
            rebirth.unit_id,
            spring::generated::owned::unit_control::DestroyUnitOptions {
                selfd: false,
                reclaimed: false,
                attacker_id: -1,
                recycle_id: false,
            },
        );
    }
    let name = units::def_name(rebirth.unit_def_id)?;
    units::create(&name, rebirth.origin, rebirth.team_id)
}

pub const SCAN_FRAME: i32 = 30;

const LONG_IDLE_FRAMES: i32 = 30 * 30;
const SHORT_IDLE_FRAMES: i32 = 10 * 30;
const STUCK_HEIGHT: f32 = 20.0;
const DISPLACEMENT_THRESHOLD: f32 = 1.0;

fn is_respawnable(unit_id: i32) -> bool {
    spring::get_unit_rules_param_f32(unit_id, RESPAWNABLE).is_ok_and(|value| value == 1.0)
}

const RESPAWNABLE: &str = "respawnable";

fn displaced(from: Float3, to: Float3) -> bool {
    (from.x - to.x).abs() + (from.y - to.y).abs() + (from.z - to.z).abs() > DISPLACEMENT_THRESHOLD
}
