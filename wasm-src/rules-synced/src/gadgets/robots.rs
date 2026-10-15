pub mod behavior;
use alloc::rc::Rc;

use spring_addons::AddonContext;
use spring_addons::rules::{CommandEvent, Gadget, UnitDestroyedEvent, UnitPreDamagedEvent};

use self::behavior::Robots;
use crate::state::{Global, PerLevel};

#[derive(Default)]
pub struct RobotsGadget {
    robots: Rc<PerLevel<Robots>>,
}

impl Gadget<Global> for RobotsGadget {
    fn name(&self) -> &'static str {
        "Robots"
    }

    fn unit_pre_damaged(
        &self,
        ctx: &AddonContext<'_, Global>,
        event: &UnitPreDamagedEvent,
    ) -> Option<spring::DamageResult> {
        let avoidance = self.robots.get(ctx.global()).flee(
            event.unit_id,
            event.unit_def_id,
            event.attacker_id,
            event.attacker_def_id,
            spring::game::current_frame(),
        );
        if let Some(avoidance) = avoidance {
            let (unit_id, frame) = (avoidance.unit_id, spring::game::current_frame());
            self.robots.get(ctx.global()).mark(unit_id, frame);
            let robots = Rc::clone(&self.robots);
            behavior::move_away(avoidance).then(move |()| {
                crate::with_global(|global| robots.get(global).mark(unit_id, frame));
            });
        }
        let same_team = event.unit_team == event.attacker_team;
        behavior::is_friendly_fire(event.unit_def_id, event.attacker_def_id, same_team)
            .then(|| spring::DamageResult::unchanged(0.0))
    }

    fn unit_cmd_done(&self, ctx: &AddonContext<'_, Global>, event: &CommandEvent<'_>) {
        self.robots.get(ctx.global()).command_done(event.unit_id);
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        self.robots.get(ctx.global()).command_done(event.unit_id);
    }
}
