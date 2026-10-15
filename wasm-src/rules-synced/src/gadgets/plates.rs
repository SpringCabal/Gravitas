pub(crate) mod behavior;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use crate::gadgets::plates::behavior as plate;
use crate::state::Global;
use crate::tests;

pub struct PlatesGadget;

impl Gadget<Global> for PlatesGadget {
    fn name(&self) -> &'static str {
        "Plates"
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        let is_plate = ctx
            .global()
            .plates
            .access_mut()
            .created(unit_id, unit_def_id);
        if is_plate {
            plate::disable_collision(unit_id);
        }
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        ctx.global().plates.access_mut().destroyed(event.unit_id);
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let global = ctx.global();
        let changes = global.plates.access_mut().pending(frame);
        for (unit_id, active) in changes {
            tests::note_toggled(global, unit_id, active);
            plate::set_state(unit_id, active);
            let Some(unit_def_id) = crate::units::def_of(unit_id) else {
                continue;
            };
            if crate::defs::get().gate != Some(unit_def_id) {
                continue;
            }
            let mut terrain = global.terrain.access_mut();
            if active {
                terrain.defer(unit_id, unit_def_id, 0.0);
            } else {
                terrain.defer_until(unit_id, unit_def_id, 1.0, frame + GATE_CLOSE_FRAMES);
            }
        }
    }
}

const GATE_CLOSE_FRAMES: i32 = 27;
