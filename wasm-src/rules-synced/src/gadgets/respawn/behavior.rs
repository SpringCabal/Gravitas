use alloc::vec::Vec;
use spring::ResultExt;

use spring::Float3;

use crate::effects;
use crate::units;

#[derive(Default)]
pub struct Respawns {
    watched: Vec<Watched>,
    dead: Vec<Rebirth>,
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
    pub fn scan(&mut self, spring: &spring::Spring<'_, spring::RulesSynced>, frame: i32) {
        let Ok(units) = spring.units().get_all_units() else {
            return;
        };
        for unit_id in units {
            if !effects::is_respawnable(spring, unit_id) {
                continue;
            }
            let (Some(unit_def_id), Ok(team_id)) = (
                units::def_of(spring, unit_id),
                spring.units_info().get_unit_team(unit_id),
            ) else {
                continue;
            };
            self.adopt(spring, unit_id, unit_def_id, team_id, frame);
        }
    }

    pub fn adopt(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        unit_id: i32,
        unit_def_id: i32,
        team_id: i32,
        frame: i32,
    ) {
        if self
            .watched
            .iter()
            .any(|watched| watched.unit_id == unit_id)
        {
            return;
        }
        let Some(origin) = units::position(spring, unit_id) else {
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

    pub fn destroyed(&mut self, unit_id: i32, wiping: bool) {
        if let Some(rebirth) = self.unwatch(unit_id)
            && !wiping
        {
            self.dead.push(rebirth);
        }
    }

    fn unwatch(&mut self, unit_id: i32) -> Option<Rebirth> {
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

    pub fn update(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
    ) -> Vec<Rebirth> {
        let mut due = core::mem::take(&mut self.dead);
        if frame % CHECK_INTERVAL != 0 {
            return due;
        }
        let mut stuck = Vec::new();
        for watched in &mut self.watched {
            let Some(position) = units::position(spring, watched.unit_id) else {
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
        due.extend(
            stuck
                .into_iter()
                .filter_map(|unit_id| self.unwatch(unit_id)),
        );
        due
    }
}

pub fn rebirth(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    rebirth: &Rebirth,
    reborn: impl FnOnce(&spring::Spring<'_, spring::RulesSynced>, i32) + 'static,
) {
    let old = rebirth.unit_id;
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            if units::valid(spring, old) {
                spring
                    .unit_control()
                    .destroy_unit(
                        old,
                        spring::unit_control::DestroyUnitOptions {
                            selfd: false,
                            reclaimed: false,
                            attacker_id: -1,
                            recycle_id: false,
                        },
                    )
                    .log_err_once(spring, "respawn: removing the old unit");
            }
        })
    });
    let (unit_def_id, origin) = (rebirth.unit_def_id, rebirth.origin);
    units::create(spring, unit_def_id, origin, rebirth.team_id).then(move |spring, unit_id| {
        let Some(unit_id) = unit_id else {
            spring::log_warning!(
                spring,
                crate::diagnostics::LOG_SECTION,
                "respawn failed: unit def {} at {} {}",
                unit_def_id,
                origin.x,
                origin.z
            );
            return;
        };
        effects::mark_respawnable(spring, unit_id);
        reborn(spring, unit_id);
    });
}

pub const SCAN_FRAME: i32 = 30;
const CHECK_INTERVAL: i32 = 30;

const LONG_IDLE_FRAMES: i32 = 30 * spring::consts::GAME_SPEED;
const SHORT_IDLE_FRAMES: i32 = 10 * spring::consts::GAME_SPEED;
const STUCK_HEIGHT: f32 = 20.0;
const DISPLACEMENT_THRESHOLD: f32 = 1.0;

fn displaced(from: Float3, to: Float3) -> bool {
    (from.x - to.x).abs() + (from.y - to.y).abs() + (from.z - to.z).abs() > DISPLACEMENT_THRESHOLD
}
