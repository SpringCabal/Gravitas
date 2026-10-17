use alloc::vec::Vec;

use spring::consts::SQUARE_SIZE_F;

use crate::units;

#[derive(Default)]
pub struct Terrain {
    pending: Vec<Pending>,
}

impl Terrain {
    pub fn defer(&mut self, unit_id: i32, unit_def_id: i32, height: f32) {
        self.queue(unit_id, unit_def_id, height, i32::MIN);
    }

    pub fn defer_until(&mut self, unit_id: i32, unit_def_id: i32, height: f32, due_frame: i32) {
        self.queue(unit_id, unit_def_id, height, due_frame);
    }

    pub fn take_due(
        &mut self,
        spring: &spring::Spring<'_, spring::RulesSynced>,
        frame: i32,
    ) -> Vec<(i32, f32)> {
        let mut due = Vec::new();
        self.pending.retain(|entry| {
            if entry.due_frame > frame {
                return true;
            }
            if is_wall_like(spring, entry.unit_id, entry.unit_def_id) {
                due.push((entry.unit_id, entry.height));
            }
            false
        });
        due
    }

    fn queue(&mut self, unit_id: i32, unit_def_id: i32, height: f32, due_frame: i32) {
        match self
            .pending
            .iter_mut()
            .find(|entry| entry.unit_id == unit_id)
        {
            Some(entry) => {
                entry.height = height;
                entry.due_frame = due_frame;
            }
            None => self.pending.push(Pending {
                unit_id,
                unit_def_id,
                height,
                due_frame,
            }),
        }
    }
}

pub fn level(spring: &spring::Spring<'_, spring::RulesSynced>, due: Vec<(i32, f32)>) {
    for (unit_id, height) in due {
        adjust_wall_terrain(spring, unit_id, height);
    }
}

#[derive(Clone, Copy)]
struct Pending {
    unit_id: i32,
    unit_def_id: i32,
    height: f32,
    due_frame: i32,
}

const STEP: f32 = SQUARE_SIZE_F;
const MARGIN: f32 = 2.0;
const MAX_STEPS: i32 = 256;

fn adjust_wall_terrain(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    height: f32,
) {
    let Some(position) = units::position(spring, unit_id) else {
        return;
    };
    let Ok(volume) = spring.units_info().get_unit_collision_volume_data(unit_id) else {
        return;
    };
    let Ok(direction) = spring.units_info().get_unit_direction(unit_id) else {
        return;
    };
    let half_x = volume.scale_x * direction.z / 2.0;
    let half_z = volume.scale_z * direction.x / 2.0;
    let start_x = position.x - (half_x + half_z);
    let start_z = position.z + volume.scale_x * direction.x / 2.0;
    let end_x = position.x + (half_x + half_z);
    let end_z = position.z - volume.scale_x * direction.x / 2.0;
    let step_x = direction.z * STEP;
    let step_z = -(direction.x * STEP);
    let steps = if step_x.abs() > step_z.abs() {
        ((start_x - end_x) / step_x).abs().ceil() as i32 + 1
    } else {
        ((start_z - end_z) / step_z).abs().ceil() as i32 + 1
    };
    let level = position.y + height * volume.scale_y;
    let mut x = start_x;
    let mut z = start_z;
    for _ in 0..steps.clamp(1, MAX_STEPS) {
        let snapped_x = round_to_square(x);
        let snapped_z = round_to_square(z);
        let _ = spring.terrain_control().level_height_map(
            snapped_x - MARGIN,
            snapped_z - MARGIN,
            snapped_x + MARGIN,
            snapped_z + MARGIN,
            level,
        );
        x += step_x;
        z += step_z;
    }
}

fn is_wall_like(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    unit_def_id: i32,
) -> bool {
    if crate::defs::get(spring).gate == Some(unit_def_id) {
        return true;
    }
    crate::defs::get(spring).unit(unit_def_id).wall
        && spring
            .rules_params()
            .get_unit_rules_param_f32(unit_id, crate::effects::INVULNERABLE)
            .is_ok_and(|value| value == 0.0)
}

fn round_to_square(value: f32) -> f32 {
    let remainder = value % SQUARE_SIZE_F;
    if remainder > SQUARE_SIZE_F / 2.0 {
        value + SQUARE_SIZE_F - remainder
    } else {
        value - remainder
    }
}
