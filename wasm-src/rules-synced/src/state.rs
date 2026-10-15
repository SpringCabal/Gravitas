use core::cell::Cell;

use alloc::vec::Vec;

use spring_addons::Resource;

use crate::gadgets::fall::falldamage::FallImmunity;
use crate::gadgets::impulse::behavior::Impulses;
use crate::gadgets::jump::behavior::Jumps;
use crate::gadgets::plates::behavior::Plates;
use crate::gadgets::player::behavior::PlayerCommand;
use crate::gadgets::terrain::behavior::Terrain;

#[derive(Default)]
pub struct Global {
    pub level: Resource<Level>,
    pub player_unit: Cell<Option<i32>>,
    pub pending_rules: Cell<crate::units::RulesParams>,
    pub player_input: Cell<Option<PlayerCommand>>,
    pub jumps: Resource<Jumps>,
    pub fall_immunity: Resource<FallImmunity>,
    pub impulses: Resource<Impulses>,
    pub plates: Resource<Plates>,
    pub terrain: Resource<Terrain>,
    pub restart: Restart,
    pub outcome: Cell<Option<bool>>,
    pub tests: crate::tests::Shared,
}

#[derive(Default)]
pub struct Level {
    pub phase: Phase,
    pub spawned: Vec<(i32, i32)>,
    pub link_failures: u32,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Phase {
    #[default]
    Spawn,
    Link,
    Running,
}

impl Level {
    pub fn unit_of(&self, scenario_id: i32) -> Option<i32> {
        self.spawned
            .iter()
            .find(|(id, _)| *id == scenario_id)
            .map(|(_, unit_id)| *unit_id)
    }
}

#[derive(Default)]
pub struct Restart {
    requested: Cell<bool>,
    wiping: Cell<bool>,
    generation: Cell<u32>,
}

impl Restart {
    pub fn request(&self) {
        self.requested.set(true);
    }

    pub fn take_request(&self) -> bool {
        self.requested.replace(false)
    }

    pub fn wiping(&self) -> bool {
        self.wiping.get()
    }

    pub fn begin_wipe(&self) {
        self.wiping.set(true);
        self.generation.set(self.generation.get().wrapping_add(1));
    }

    pub fn finish_wipe(&self) {
        self.wiping.set(false);
    }

    pub fn generation(&self) -> u32 {
        self.generation.get()
    }
}

pub struct PerLevel<T> {
    generation: Cell<u32>,
    value: core::cell::RefCell<T>,
}

impl<T: Default> Default for PerLevel<T> {
    fn default() -> Self {
        Self {
            generation: Cell::new(0),
            value: core::cell::RefCell::new(T::default()),
        }
    }
}

impl<T: Default> PerLevel<T> {
    pub fn get(&self, global: &Global) -> core::cell::RefMut<'_, T> {
        let generation = global.restart.generation();
        let mut value = self.value.borrow_mut();
        if self.generation.replace(generation) != generation {
            *value = T::default();
        }
        value
    }
}
