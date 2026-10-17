use alloc::vec::Vec;
use spring::ResultExt;

use crate::gadgets::level::data as level;
use crate::state::Global;
use crate::units;

#[derive(Default)]
pub struct Mission {
    fired: Vec<i32>,
    inside: Vec<i32>,
}

pub struct Assault {
    pub units: Vec<i32>,
    pub target_x: f32,
    pub target_z: f32,
}

impl Mission {
    pub fn update(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        player_unit: Option<i32>,
        frame: i32,
    ) -> Vec<Assault> {
        if frame % ENTER_AREA_INTERVAL != 0 {
            return Vec::new();
        }
        let Some(player_unit) = player_unit else {
            return Vec::new();
        };
        let Some(position) = units::position(spring, player_unit) else {
            return Vec::new();
        };
        let mut assaults = Vec::new();
        for trigger in level::ENTER_AREA_TRIGGERS {
            let Some(area) = area(trigger.area) else {
                continue;
            };
            let inside = contains(area, position.x, position.z);
            let was_inside = self.inside.contains(&trigger.area);
            self.track(trigger.area, inside);
            if !inside || was_inside || self.fired.contains(&trigger.area) {
                continue;
            }
            self.fired.push(trigger.area);
            if let Some(assault) = build_assault(spring, trigger) {
                assaults.push(assault);
            }
        }
        assaults
    }
}

pub fn stun_electrafi_proximity(spring: &spring::Spring<'_, spring::RulesSynced>) {
    let Some(electrafi) = crate::defs::get(spring).electrafi else {
        return;
    };
    let mut stunned = Vec::new();
    for field in spring.units().get_all_units().unwrap_or_default() {
        if units::def_of(spring, field) != Some(electrafi) {
            continue;
        }
        let Some(position) = units::position(spring, field) else {
            continue;
        };
        let radius = spring
            .rules_params()
            .get_unit_rules_param_f32(field, crate::gadgets::electrafi::behavior::SIZE_PARAM)
            .unwrap_or(crate::gadgets::electrafi::behavior::DEFAULT_RADIUS);
        for unit_id in units::units_in_sphere(
            spring,
            position,
            radius + crate::gadgets::electrafi::behavior::PROXIMITY_MARGIN,
        ) {
            if !stunned.contains(&unit_id) {
                stunned.push(unit_id);
                stun(spring, unit_id);
            }
        }
    }
}

pub fn issue_assault(spring: &spring::Spring<'_, spring::RulesSynced>, assault: &Assault) {
    for unit_id in &assault.units {
        let target_y = units::ground_height(spring, assault.target_x, assault.target_z);
        units::order(
            spring,
            *unit_id,
            spring::cmd::FIGHT,
            &[assault.target_x, target_y, assault.target_z],
        );
    }
}

pub fn win(spring: &spring::Spring<'_, spring::RulesSynced>, global: &Global, ally_teams: &[i32]) {
    let victory = ally_teams.contains(&PLAYER_ALLY_TEAM);
    global.outcome.set(Some(victory));
    springcabal::message::send_to_ui(spring, &gravitas_shared::game_over(victory));
}

const PLAYER_ALLY_TEAM: i32 = 0;

const ENTER_AREA_INTERVAL: i32 = 5;
const STUN_MULTIPLIER: f32 = 1.5;

impl Mission {
    fn track(&mut self, area_id: i32, inside: bool) {
        let position = self.inside.iter().position(|value| *value == area_id);
        match (inside, position) {
            (true, None) => self.inside.push(area_id),
            (false, Some(index)) => {
                self.inside.swap_remove(index);
            }
            _ => {}
        }
    }
}

fn area(area_id: i32) -> Option<&'static level::Area> {
    level::AREAS.iter().find(|area| area.id == area_id)
}

fn contains(area: &level::Area, x: f32, z: f32) -> bool {
    let (x1, x2) = (area.x1.min(area.x2), area.x1.max(area.x2));
    let (z1, z2) = (area.z1.min(area.z2), area.z1.max(area.z2));
    x >= x1 && x <= x2 && z >= z1 && z <= z2
}

fn build_assault(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    trigger: &level::EnterAreaTrigger,
) -> Option<Assault> {
    let source = area(trigger.units_area)?;
    let target = area(trigger.target_area)?;
    let units = units_in_area(spring, source);
    if units.is_empty() {
        return None;
    }
    Some(Assault {
        units,
        target_x: (target.x1 + target.x2) / 2.0,
        target_z: (target.z1 + target.z2) / 2.0,
    })
}

fn units_in_area(spring: &spring::Spring<'_, spring::RulesSynced>, area: &level::Area) -> Vec<i32> {
    units::units_in_rectangle(
        spring,
        area.x1.min(area.x2),
        area.z1.min(area.z2),
        area.x1.max(area.x2),
        area.z1.max(area.z2),
    )
}

fn stun(_spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) {
    crate::with_addon_context(|ctx| {
        ctx.commands().run(move |spring| {
            let Ok(health) = spring.units_info().get_unit_health(unit_id) else {
                return;
            };
            spring
                .unit_control()
                .set_unit_health(
                    unit_id,
                    spring::unit_control::UnitHealthValue {
                        health: health.health,
                        capture: health.capture_progress,
                        paralyze: health.max_health * STUN_MULTIPLIER,
                        build: health.build_progress,
                        use_amounts: true,
                    },
                )
                .log_err_once(spring, "stunning a unit near an electric field");
        })
    });
}
