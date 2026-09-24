//! What the gadgets share. Everything else is owned by the gadget that uses it.
//!
//! Each part is its own `Resource`, borrowed only for the duration of a state
//! update: never across an engine call that can raise a callin (creating,
//! destroying, damaging or ordering a unit). A conflicting borrow panics.

use core::cell::Cell;

use alloc::vec::Vec;

use spring_addons::Resource;

use crate::falldamage::FallImmunity;
use crate::impulse::Impulses;
use crate::jump::Jumps;
use crate::plate::Plates;
use crate::player::PlayerCommand;
use crate::terrain::Terrain;

#[derive(Default)]
pub struct Global {
    /// The level being played.
    pub level: Resource<Level>,
    /// The unit the local player controls.
    pub player_unit: Cell<Option<i32>>,
    /// The scenario rules params of the unit being spawned, for the unit
    /// setup gadget's UnitCreated.
    pub pending_rules: Cell<crate::units::RulesParams>,
    /// The latest input from the ui module, until the player gadget takes it.
    pub player_input: Cell<Option<PlayerCommand>>,
    /// Units in flight. The player gadget starts jumps; the jump gadget flies
    /// and lands them.
    pub jumps: Resource<Jumps>,
    /// Units that don't take fall damage for a while (a jump landing).
    pub fall_immunity: Resource<FallImmunity>,
    /// Gravity-gun pushes collected in UnitPreDamaged, applied each frame.
    pub impulses: Resource<Impulses>,
    /// Plates and the gates they open, linked by the level setup.
    pub plates: Resource<Plates>,
    /// Terrain under walls and gates, levelled when they appear or open.
    pub terrain: Resource<Terrain>,
    pub restart: Restart,
    /// `Some(victory)` once the game is over.
    pub outcome: Cell<Option<bool>>,
    /// The e2e arena and what the gadgets report to its cases (empty without
    /// the `e2e` feature).
    pub tests: crate::tests::Shared,
}

/// The current level.
#[derive(Default)]
pub struct Level {
    pub phase: Phase,
    /// Scenario unit ids and the units spawned for them.
    pub spawned: Vec<(i32, i32)>,
    /// Plate and gate links that could not be made.
    pub link_failures: u32,
}

#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub enum Phase {
    /// Spawn the level on the next frame.
    #[default]
    Spawn,
    /// Spawned; link plates and gates on the next frame.
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

/// A restart, from the request to the new level.
#[derive(Default)]
pub struct Restart {
    requested: Cell<bool>,
    wiping: Cell<bool>,
    /// Bumped on every restart, so gadgets can drop per-level state.
    generation: Cell<u32>,
}

impl Restart {
    pub fn request(&self) {
        self.requested.set(true);
    }

    pub fn take_request(&self) -> bool {
        self.requested.replace(false)
    }

    /// True while the old level's units are being removed: their deaths are
    /// not gameplay (no explosions, respawns or game over).
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

/// Gadget state that belongs to one level and starts over on a restart.
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
    /// The value for the current level. Panics on a conflicting borrow.
    pub fn get(&self, global: &Global) -> core::cell::RefMut<'_, T> {
        let generation = global.restart.generation();
        let mut value = self.value.borrow_mut();
        if self.generation.replace(generation) != generation {
            *value = T::default();
        }
        value
    }
}
