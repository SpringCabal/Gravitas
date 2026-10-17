use alloc::string::String;

use gravitas_shared::Status;
use spring_addons::AddonContext;
use spring_addons::rules::Gadget;
use springcabal::message::SendOnChange;

use crate::state::Global;
use crate::units;

const STATUS_PERIOD: i32 = 3;

#[derive(Default)]
pub struct StatusGadget {
    sent: SendOnChange<String>,
}

impl Gadget<Global> for StatusGadget {
    fn name(&self) -> &'static str {
        "Status"
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        if frame % STATUS_PERIOD != 0 {
            return;
        }
        let message = status_message(spring, ctx.global(), frame);
        self.sent.send_if_changed(message, |message| {
            springcabal::message::send_to_ui(spring, message)
        });
    }
}

fn status_message(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    global: &Global,
    frame: i32,
) -> String {
    let player_unit = global.player_unit.get();
    let player = player_unit.unwrap_or(-1);
    let (health, max_health) = health(spring, player);
    Status {
        player_unit,
        health,
        max_health,
        push_ready: reload_ready(spring, player, units::WEAPON_PUSH, frame),
        pull_ready: reload_ready(spring, player, units::WEAPON_PULL, frame),
    }
    .encode()
}

fn health(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32) -> (i32, i32) {
    if unit_id < 0 {
        return (0, 1);
    }
    spring
        .units_info()
        .get_unit_health(unit_id)
        .map(|health| (health.health.max(0.0) as i32, health.max_health as i32))
        .unwrap_or((0, 1))
}

fn reload_ready(
    spring: &spring::Spring<'_, spring::RulesSynced>,
    unit_id: i32,
    weapon: i32,
    frame: i32,
) -> bool {
    if unit_id < 0 {
        return false;
    }
    spring
        .units_weapons()
        .get_unit_weapon_state(unit_id, weapon, "reloadFrame")
        .is_ok_and(|state| state.reload_frame <= frame as f32)
}
