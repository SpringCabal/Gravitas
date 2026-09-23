use alloc::string::String;
use alloc::vec::Vec;

use crate::units;

/// Terrain levelling queued for walls and gates, flushed once a frame.
#[derive(Default)]
pub struct Terrain {
    pending: Vec<Pending>,
    /// How many walls of each def were levelled, for the log.
    levelled: Vec<(String, u32)>,
}

impl Terrain {
    /// Level the ground under a wall to `height` (0 = its base, 1 = its top).
    pub fn defer(&mut self, unit_id: i32, unit_def_id: i32, height: f32) {
        self.queue(unit_id, unit_def_id, height, i32::MIN);
    }

    /// Like `defer`, but not before `due_frame`.
    pub fn defer_until(&mut self, unit_id: i32, unit_def_id: i32, height: f32, due_frame: i32) {
        self.queue(unit_id, unit_def_id, height, due_frame);
    }

    /// Take what is due by `frame`, counting the walls among it.
    pub fn take_due(&mut self, frame: i32) -> Vec<(i32, f32)> {
        let mut due = Vec::new();
        let mut walls = Vec::new();
        self.pending.retain(|entry| {
            if entry.due_frame > frame {
                return true;
            }
            if is_wall_like(entry.unit_id, entry.unit_def_id) {
                walls.push(entry.unit_def_id);
                due.push((entry.unit_id, entry.height));
            }
            false
        });
        for unit_def_id in walls {
            self.record_levelled(unit_def_id);
        }
        due
    }

    pub fn levelled_report(&self) -> String {
        let mut out = String::new();
        for (name, count) in &self.levelled {
            out.push_str(name);
            out.push('=');
            out.push_str(&alloc::format!("{count} "));
        }
        out
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

    fn record_levelled(&mut self, unit_def_id: i32) {
        let name = &crate::defs::get().unit(unit_def_id).name;
        match self
            .levelled
            .iter_mut()
            .find(|(candidate, _)| candidate == name)
        {
            Some(entry) => entry.1 += 1,
            None => self.levelled.push((name.clone(), 1)),
        }
    }
}

/// Level the terrain under each wall.
pub fn level(due: Vec<(i32, f32)>) {
    for (unit_id, height) in due {
        adjust_wall_terrain(unit_id, height);
    }
}

#[derive(Clone, Copy)]
struct Pending {
    unit_id: i32,
    unit_def_id: i32,
    height: f32,
    due_frame: i32,
}

const STEP: f32 = 8.0;
const MARGIN: f32 = 2.0;
const SQUARE_SIZE: f32 = 8.0;
const MAX_STEPS: i32 = 256;

fn adjust_wall_terrain(unit_id: i32, height: f32) {
    let Some(position) = units::position(unit_id) else {
        return;
    };
    let Ok(volume) = spring::units_info::get_unit_collision_volume_data(unit_id) else {
        return;
    };
    let Ok(direction) = spring::units_info::get_unit_direction(unit_id) else {
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
        let _ = spring::level_height_map(
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

fn is_wall_like(unit_id: i32, unit_def_id: i32) -> bool {
    if crate::defs::get().gate == Some(unit_def_id) {
        return true;
    }
    crate::defs::get().unit(unit_def_id).wall
        && spring::get_unit_rules_param_f32(unit_id, "invulnerable").is_ok_and(|value| value == 0.0)
}

fn round_to_square(value: f32) -> f32 {
    let remainder = value % SQUARE_SIZE;
    if remainder > SQUARE_SIZE / 2.0 {
        value + SQUARE_SIZE - remainder
    } else {
        value - remainder
    }
}
