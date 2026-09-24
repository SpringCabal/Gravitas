use core::cell::RefCell;

use alloc::vec::Vec;

use spring::ResultExt;
use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent, UnitPreDamagedEvent};

use crate::state::Global;
use crate::{effects, falldamage};

/// Units bounce off the ground and take damage from hard landings; units that
/// fall off the map die.
///
/// A collision with another unit (a robot thrown into a crate or at the
/// player) carries the other unit as the attacker and does the engine's
/// damage. A collision without one (a feature) does nothing at once and the
/// multiplied engine damage on the next frame, as `game_fall_damage.lua` does.
/// Walls don't block: they raise the terrain, so hitting one is a ground
/// collision.
#[derive(Default)]
pub struct FallDamageGadget {
    /// Collisions without an attacker this frame: unit and engine damage.
    debris: RefCell<Vec<(i32, f32)>>,
}

impl Gadget<Global> for FallDamageGadget {
    fn name(&self) -> &'static str {
        "FallDamage"
    }

    fn game_frame(&self, _ctx: &AddonContext<'_, Global>, frame: i32) {
        // Damage raises UnitPreDamaged here: release the list first.
        let debris = core::mem::take(&mut *self.debris.borrow_mut());
        for (unit_id, damage) in debris {
            spring::add_unit_damage(
                unit_id,
                damage * falldamage::DEBRIS_DAMAGE_MULTIPLIER,
                0.0,
                falldamage::DEBRIS_DAMAGE,
                NO_ATTACKER,
                spring::types::Float3::default(),
            )
            .log_err_once("debris collision damage");
        }
        effects::kill_fallen(frame);
    }

    fn unit_pre_damaged(
        &self,
        ctx: &AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        match event.weapon_def_id {
            falldamage::OBJECT_COLLISION if event.attacker_id >= 0 => None,
            falldamage::OBJECT_COLLISION => {
                self.debris.borrow_mut().push((event.unit_id, event.damage));
                Some(spring::DamageResult::unchanged(0.0))
            }
            falldamage::GROUND_COLLISION => {
                let immune = ctx
                    .global()
                    .fall_immunity
                    .access()
                    .is_immune(event.unit_id, spring::game::current_frame());
                let damage = if immune {
                    0.0
                } else {
                    falldamage::ground_collision(event.unit_id, event.unit_def_id)
                };
                falldamage::play_bounce(event.unit_id, damage);
                Some(spring::DamageResult::unchanged(damage))
            }
            _ => None,
        }
    }

    fn unit_destroyed(&self, _ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        self.debris
            .borrow_mut()
            .retain(|(unit_id, _)| *unit_id != event.unit_id);
    }
}

const NO_ATTACKER: i32 = -1;
