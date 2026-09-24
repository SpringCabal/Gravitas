use alloc::vec::Vec;

use spring::types::Float3;

use crate::state::Global;
use crate::units;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Started,
    Busy,
    OutOfRange,
    TooHigh,
    Blocked,
    OutsideMap,
    Invalid,
}

impl Outcome {
    pub fn reason(self) -> &'static str {
        match self {
            Outcome::Started => "started",
            Outcome::Busy => "already jumping",
            Outcome::OutOfRange => "out of range, walking closer",
            Outcome::TooHigh => "NOT SO HIGH GRAVIT!",
            Outcome::Blocked => "a wall is in the way",
            Outcome::OutsideMap => "the jump target is outside the map",
            Outcome::Invalid => "not a jumping unit",
        }
    }
}

#[derive(Default)]
pub struct Jumps {
    active: Vec<Jump>,
}

struct Jump {
    unit_id: i32,
    start: Float3,
    vector: Float3,
    height: f32,
    step: f32,
    progress: f32,
    delay: i32,
    half_done: bool,
    heading: f32,
    turn: f32,
}

/// What one jump does this frame.
struct Flight {
    unit_id: i32,
    /// Where the unit is and which way it faces, once it has taken off.
    pose: Option<(Float3, f32)>,
    calls: Vec<&'static str>,
    landed: bool,
}

impl Jumps {
    pub fn is_jumping(&self, unit_id: i32) -> bool {
        self.active.iter().any(|jump| jump.unit_id == unit_id)
    }

    pub fn forget(&mut self, unit_id: i32) {
        self.active.retain(|jump| jump.unit_id != unit_id);
    }

    fn advance(&mut self) -> Vec<Flight> {
        let mut flights = Vec::new();
        for jump in &mut self.active {
            let mut flight = Flight {
                unit_id: jump.unit_id,
                pose: None,
                calls: Vec::new(),
                landed: false,
            };
            if jump.delay > 0 {
                jump.delay -= 1;
                if jump.delay == 0 {
                    flight.calls.push("StartJump");
                }
                flights.push(flight);
                continue;
            }
            let progress = jump.progress;
            let arc = 1.0 - (2.0 * progress - 1.0) * (2.0 * progress - 1.0);
            let position = Float3 {
                x: jump.start.x + jump.vector.x * progress,
                y: jump.start.y + jump.vector.y * progress + arc * jump.height,
                z: jump.start.z + jump.vector.z * progress,
            };
            flight.pose = Some((position, jump.heading + jump.turn * progress));
            flight.calls.push("Jumping");
            if !jump.half_done && progress > 0.5 {
                jump.half_done = true;
                flight.calls.push("HalfJump");
            }
            jump.progress += jump.step;
            flight.landed = jump.progress > 1.0;
            flights.push(flight);
        }
        self.active.retain(|jump| jump.progress <= 1.0);
        flights
    }
}

/// Start a jump of `unit_id` to the goal, or say why not.
pub fn start(global: &Global, unit_id: i32, goal_x: f32, goal_y: f32, goal_z: f32) -> Outcome {
    if global.jumps.access().is_jumping(unit_id) {
        return Outcome::Busy;
    }
    let Some(def_id) = units::def_of(unit_id) else {
        return Outcome::Invalid;
    };
    if crate::defs::get().gravit != Some(def_id) {
        return Outcome::Invalid;
    }
    let Some(start) = units::position(unit_id) else {
        return Outcome::Invalid;
    };
    if !on_map(goal_x, goal_z) {
        return Outcome::OutsideMap;
    }
    if (start.y - goal_y).abs() > CLIMB_LIMIT {
        return Outcome::TooHigh;
    }
    let goal = Float3 {
        x: goal_x,
        y: units::ground_height(goal_x, goal_z),
        z: goal_z,
    };
    let vector = Float3 {
        x: goal.x - start.x,
        y: goal.y - start.y,
        z: goal.z - start.z,
    };
    let range = rules_param(unit_id, RANGE_PARAM).unwrap_or(RANGE);
    let planar_distance = (vector.x * vector.x + vector.z * vector.z).sqrt();
    if planar_distance > range {
        crate::diagnostics::check(
            spring::set_unit_move_goal(unit_id, goal, range, 0.0, false),
            || alloc::format!("move goal towards a jump for unit {unit_id}"),
        );
        return Outcome::OutOfRange;
    }
    let line_distance = length(vector).max(MINIMUM_DISTANCE);
    let vertex = Float3 {
        x: start.x + vector.x * 0.5,
        y: start.y + vector.y * 0.5 + HEIGHT,
        z: start.z + vector.z * 0.5,
    };
    let flight_distance = distance(start, vertex) + distance(vertex, goal);
    if flight_distance <= 0.0 {
        return Outcome::Invalid;
    }
    let speed =
        rules_param(unit_id, SPEED_PARAM).unwrap_or(SPEED) * line_distance / flight_distance;
    let step = speed / line_distance;
    // A jump that never advances (jumpSpeed 0) would never land.
    if !(step > 0.0 && step.is_finite()) {
        return Outcome::Invalid;
    }
    if blocked(start, vector, step) {
        return Outcome::Blocked;
    }
    let heading = spring::units_info::get_unit_heading(unit_id, true).unwrap_or(0.0);
    let turn = shortest_turn(heading, vector.x.atan2(vector.z));
    units::move_ctrl(unit_id, true);
    call_script(unit_id, "PreJump");
    global.jumps.access_mut().active.push(Jump {
        unit_id,
        start,
        vector,
        height: HEIGHT,
        step,
        progress: 0.0,
        delay: DELAY,
        half_done: false,
        heading,
        turn,
    });
    Outcome::Started
}

/// Move every jumping unit along its arc. Returns the units that landed.
pub fn fly(global: &Global) -> Vec<i32> {
    let flights = global.jumps.access_mut().advance();
    let mut landed = Vec::new();
    for flight in flights {
        if let Some((position, heading)) = flight.pose {
            let _ = spring::set_unit_position(flight.unit_id, position);
            units::face_radians(flight.unit_id, heading);
        }
        for name in flight.calls {
            call_script(flight.unit_id, name);
        }
        if flight.landed {
            landed.push(flight.unit_id);
        }
    }
    for unit_id in &landed {
        call_script(*unit_id, "StopJump");
        units::move_ctrl(*unit_id, false);
    }
    landed
}

pub const RANGE: f32 = 400.0;
pub const HEIGHT: f32 = 100.0;
pub const SPEED: f32 = 12.0;
pub const DELAY: i32 = 2;
pub const LANDING_IMMUNITY_FRAMES: i32 = 30;

const MINIMUM_DISTANCE: f32 = 0.00001;
const CLIMB_LIMIT: f32 = 50.0;
const WALL_CLEARANCE: f32 = 30.0;
const MAP_MARGIN: f32 = 32.0;
const RANGE_PARAM: &str = "jumpRange";
const SPEED_PARAM: &str = "jumpSpeed";

fn on_map(x: f32, z: f32) -> bool {
    let Ok(map) = spring::game::get_game_map_info(0) else {
        return true;
    };
    x >= MAP_MARGIN
        && z >= MAP_MARGIN
        && x <= map.map_size_x as f32 - MAP_MARGIN
        && z <= map.map_size_z as f32 - MAP_MARGIN
}

fn rules_param(unit_id: i32, name: &str) -> Option<f32> {
    spring::get_unit_rules_param_f32(unit_id, name).ok()
}

fn blocked(start: Float3, vector: Float3, step: f32) -> bool {
    let mut progress = 0.0;
    let mut x = start.x;
    let mut z = start.z;
    while progress <= 1.0 {
        x += vector.x * step;
        z += vector.z * step;
        let arc = 1.0 - (2.0 * progress - 1.0) * (2.0 * progress - 1.0);
        let arc_height = start.y + vector.y * progress + arc * HEIGHT;
        if units::ground_height(x, z) - WALL_CLEARANCE > arc_height {
            return true;
        }
        progress += step;
    }
    false
}

fn shortest_turn(from: f32, to: f32) -> f32 {
    let mut turn = to - from;
    while turn > core::f32::consts::PI {
        turn -= core::f32::consts::TAU;
    }
    while turn < -core::f32::consts::PI {
        turn += core::f32::consts::TAU;
    }
    turn
}

fn call_script(unit_id: i32, function_name: &'static str) {
    crate::unit_scripts::call(unit_id, function_name);
}

fn length(value: Float3) -> f32 {
    (value.x * value.x + value.y * value.y + value.z * value.z).sqrt()
}

fn distance(from: Float3, to: Float3) -> f32 {
    length(Float3 {
        x: to.x - from.x,
        y: to.y - from.y,
        z: to.z - from.z,
    })
}
