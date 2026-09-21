use spring_addons::rules::{CommandEvent, Gadget};

use crate::state::{self, Global};

#[derive(Default)]
pub struct RobotsGadget;

impl RobotsGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for RobotsGadget {
    fn name(&self) -> &'static str {
        "Robots"
    }

    fn unit_cmd_done(
        &self,
        _ctx: &spring_addons::AddonContext<'_, Global>,
        event: &CommandEvent<'_>,
    ) {
        let _ = state::with_world(|world| world.robots.command_done(event.unit_id));
    }
}
