//! The gadgets, in the order they run. Each owns its state, or shares it
//! through `Global` where another gadget needs it.

mod barrel;
mod electrafi;
mod fall;
mod fire;
mod impulse;
mod invulnerable;
mod jump;
mod level;
mod mission;
mod plates;
mod player;
mod respawn;
mod robots;
mod status;
mod terrain;
mod tests;
mod unit_setup;

use spring_addons::register_gadgets;
use spring_addons::rules::GadgetHandler;

use crate::state::Global;

pub fn setup(handler: &mut GadgetHandler<Global>) {
    register_gadgets!(
        handler,
        [
            // First, so the rules params it sets on a new unit are there for
            // every other gadget's UnitCreated.
            unit_setup::UnitSetupGadget,
            level::LevelGadget,
            terrain::TerrainGadget,
            player::PlayerGadget::default(),
            jump::JumpGadget,
            // The UnitPreDamaged gadgets run in this order, each seeing the
            // damage the one before returned.
            impulse::ImpulseGadget,
            robots::RobotsGadget::default(),
            fall::FallDamageGadget,
            invulnerable::InvulnerableGadget,
            electrafi::ElectrafiGadget::default(),
            fire::FireGadget::default(),
            respawn::RespawnGadget::default(),
            mission::MissionGadget::default(),
            plates::PlatesGadget,
            status::StatusGadget,
            tests::TestsGadget,
            barrel::BarrelGadget,
        ]
    );
}
