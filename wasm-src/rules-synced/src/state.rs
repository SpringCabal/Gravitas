use core::cell::RefCell;

use alloc::vec::Vec;

use crate::diagnostics;
use crate::electrafi::Electrafi;
use crate::falldamage::FallDamage;
use crate::fire::Fires;
use crate::impulse::Impulses;
use crate::jump::Jumps;
use crate::mission::Mission;
use crate::plate::Plates;
use crate::player::Player;
use crate::respawn::Respawns;
use crate::robots::Robots;

pub struct World {
    pub frame_consumed: bool,
    pub pending_link: bool,
    pub initialized: bool,
    pub frame: i32,
    pub spawned: Vec<(i32, i32)>,
    created: Vec<i32>,
    duplicates_reported: bool,
    pub duplicate_creates: u32,
    pub mission: Mission,
    pub plates: Plates,
    pub impulses: Impulses,
    pub fall_damage: FallDamage,
    pub jumps: Jumps,
    pub electrafi: Electrafi,
    pub robots: Robots,
    pub fires: Fires,
    pub respawns: Respawns,
    pub player: Player,
}

thread_local! {
    static WORLD: RefCell<World> = RefCell::new(World::new());
}

impl World {
    pub fn observe_created(&mut self, unit_id: i32) -> bool {
        if self.created.contains(&unit_id) {
            self.duplicate_creates += 1;
            if !self.duplicates_reported {
                self.duplicates_reported = true;
                diagnostics::warning(&alloc::format!(
                    "unit_created repeat: first is unit {unit_id} after {} unique",
                    self.created.len()
                ));
            }
            return false;
        }
        self.created.push(unit_id);
        true
    }

    pub fn forget_created(&mut self, unit_id: i32) {
        self.created.retain(|id| *id != unit_id);
    }

    pub fn unit_of(&self, scenario_id: i32) -> Option<i32> {
        self.spawned
            .iter()
            .find(|(id, _)| *id == scenario_id)
            .map(|(_, unit_id)| *unit_id)
    }
}

impl World {
    pub fn reset(&mut self) {
        *self = World::new();
    }
}

pub fn with_world<R>(function: impl FnOnce(&mut World) -> R) -> Option<R> {
    WORLD.with(|world| match world.try_borrow_mut() {
        Ok(mut world) => Some(function(&mut world)),
        Err(_) => {
            diagnostics::warning("world state re-entered; skipping update");
            None
        }
    })
}

impl World {
    fn new() -> Self {
        Self {
            initialized: false,
            frame_consumed: false,
            pending_link: false,
            frame: 0,
            created: Vec::new(),
            duplicates_reported: false,
            duplicate_creates: 0,
            spawned: Vec::new(),
            mission: Mission::new(),
            plates: Plates::new(),
            impulses: Impulses::new(),
            fall_damage: FallDamage::new(),
            jumps: Jumps::new(),
            electrafi: Electrafi::new(),
            robots: Robots::new(),
            fires: Fires::new(),
            respawns: Respawns::new(),
            player: Player::new(),
        }
    }
}

#[derive(Default)]
pub struct Global;
