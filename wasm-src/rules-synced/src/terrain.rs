use core::cell::RefCell;

use alloc::vec::Vec;

use crate::units;

thread_local! {
    static PENDING: RefCell<Vec<Pending>> = const { RefCell::new(Vec::new()) };
    static LEVELLED: RefCell<Vec<(alloc::string::String, u32)>> = const { RefCell::new(Vec::new()) };
}

pub fn levelled_report() -> alloc::string::String {
    LEVELLED.with(|levelled| {
        let levelled = levelled.borrow();
        let mut out = alloc::string::String::new();
        for (name, count) in levelled.iter() {
            out.push_str(name);
            out.push('=');
            out.push_str(&alloc::format!("{count} "));
        }
        out
    })
}

pub fn clear() {
    PENDING.with(|pending| pending.borrow_mut().clear());
    LEVELLED.with(|levelled| levelled.borrow_mut().clear());
}

pub fn defer_wall_terrain(unit_id: i32, unit_def_id: i32, height: f32) {
    queue(unit_id, unit_def_id, height, i32::MIN);
}

pub fn defer_wall_terrain_until(unit_id: i32, unit_def_id: i32, height: f32, due_frame: i32) {
    queue(unit_id, unit_def_id, height, due_frame);
}

pub fn flush(frame: i32) {
    let due = PENDING.with(|pending| {
        let mut pending = pending.borrow_mut();
        let mut due = Vec::new();
        pending.retain(|entry| {
            if entry.due_frame > frame {
                return true;
            }
            due.push(Pending { ..*entry });
            false
        });
        due
    });
    for entry in due {
        if !is_wall_like(entry.unit_id, entry.unit_def_id) {
            continue;
        }
        record_levelled(entry.unit_def_id);
        adjust_wall_terrain(entry.unit_id, entry.height);
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
const GATE: &str = "gate";

fn queue(unit_id: i32, unit_def_id: i32, height: f32, due_frame: i32) {
    PENDING.with(|pending| {
        let mut pending = pending.borrow_mut();
        match pending.iter_mut().find(|entry| entry.unit_id == unit_id) {
            Some(entry) => {
                entry.height = height;
                entry.due_frame = due_frame;
            }
            None => pending.push(Pending {
                unit_id,
                unit_def_id,
                height,
                due_frame,
            }),
        }
    });
}

fn record_levelled(unit_def_id: i32) {
    let Some(name) = units::def_name(unit_def_id) else {
        return;
    };
    LEVELLED.with(|levelled| {
        let mut levelled = levelled.borrow_mut();
        match levelled
            .iter_mut()
            .find(|(candidate, _)| *candidate == name)
        {
            Some(entry) => entry.1 += 1,
            None => levelled.push((name, 1)),
        }
    });
}

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
    if units::def_id(GATE) == Some(unit_def_id) {
        return true;
    }
    units::has_custom_param(unit_def_id, "wall")
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
