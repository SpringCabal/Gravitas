use spring_addons::rules::Gadget;

use crate::effects;
use crate::gadgets::setup::frame_consumed;
use crate::orchestration;
use crate::state::Global;

#[derive(Default)]
pub struct ElectrafiGadget;

impl ElectrafiGadget {
    pub fn new() -> Self {
        Self
    }
}

impl Gadget<Global> for ElectrafiGadget {
    fn name(&self) -> &'static str {
        "Electrafi"
    }

    fn game_frame(&self, _ctx: &spring_addons::AddonContext<'_, Global>, frame: i32) {
        if frame_consumed() {
            return;
        }
        orchestration::apply_electrafi(frame);
        orchestration::apply_fire(frame);
        orchestration::apply_respawns(frame);
        effects::kill_fallen(frame);
    }
}
