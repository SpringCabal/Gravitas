use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitPreDamagedEvent};

use crate::state::Global;
use crate::{effects, falldamage, units};

/// Units bounce off the ground and take damage from hard landings; objects
/// don't hurt each other on collision; units that fall off the map die.
pub struct FallDamageGadget;

impl Gadget<Global> for FallDamageGadget {
    fn name(&self) -> &'static str {
        "FallDamage"
    }

    fn game_frame(&self, _ctx: &AddonContext<'_, Global>, frame: i32) {
        effects::kill_fallen(frame);
    }

    fn unit_pre_damaged(
        &self,
        ctx: &AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        match event.weapon_def_id {
            falldamage::OBJECT_COLLISION => Some(spring::DamageResult::unchanged(0.0)),
            falldamage::GROUND_COLLISION => {
                let immune = ctx
                    .global()
                    .fall_immunity
                    .access()
                    .is_immune(event.unit_id, units::game_frame());
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
}
