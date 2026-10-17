use super::*;

use spring_addons::register_gadgets;
use spring_addons::rules::GadgetHandler;

use crate::state::Global;

pub fn setup(
    handler: &mut GadgetHandler<Global>,
    _spring: &spring::Spring<'_, spring::RulesSynced>,
) {
    register_gadgets!(
        handler,
        [
            unit_setup::UnitSetupGadget,
            level::LevelGadget,
            terrain::TerrainGadget,
            player::PlayerGadget::default(),
            jump::JumpGadget,
            impulse::ImpulseGadget,
            robots::RobotsGadget::default(),
            fall::FallDamageGadget::default(),
            invulnerable::InvulnerableGadget,
            electrafi::ElectrafiGadget::default(),
            fire::FireGadget::default(),
            respawn::RespawnGadget::default(),
            mission::MissionGadget::default(),
            plates::PlatesGadget,
            status::StatusGadget::default(),
            tests::TestsGadget::default(),
            barrel::BarrelGadget,
        ]
    );
}
