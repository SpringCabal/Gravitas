pub mod falldamage;
use core::cell::RefCell;

use alloc::vec::Vec;

use spring::ResultExt;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent, UnitPreDamagedEvent};

use crate::effects;
use crate::state::Global;

#[derive(Default)]
pub struct FallDamageGadget {
    debris: RefCell<Vec<(i32, f32)>>,
}

impl Gadget<Global> for FallDamageGadget {
    fn name(&self) -> &'static str {
        "FallDamage"
    }

    fn game_frame(&self, _ctx: &AddonContext<'_, Global>, frame: i32) {
        let spring = _ctx.spring();
        for (unit_id, damage) in self.debris.take() {
            crate::with_addon_context(|ctx| {
                ctx.commands().add_unit_damage(
                    unit_id,
                    damage * falldamage::DEBRIS_DAMAGE_MULTIPLIER,
                    0.0,
                    falldamage::DEBRIS_DAMAGE,
                    NO_ATTACKER,
                    spring::Float3::default(),
                )
            })
            .then(|spring, damaged| {
                damaged.log_err_once(spring, "debris collision damage");
            });
        }
        effects::kill_fallen(spring, frame);
    }

    fn unit_pre_damaged(
        &self,
        ctx: &AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        let spring = ctx.spring();
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
                    .is_immune(event.unit_id, spring.game().current_frame().unwrap_or(0));
                let damage = if immune {
                    0.0
                } else {
                    falldamage::ground_collision(spring, event.unit_id, event.unit_def_id)
                };
                falldamage::play_bounce(spring, event.unit_id, damage);
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
