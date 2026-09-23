extern crate alloc;

mod diagnostics;
mod effects;
mod electrafi;
mod falldamage;
mod fire;
mod gadgets;
mod hazard;
mod impulse;
mod jump;
mod level;
mod lifecycle;
mod mission;
mod orchestration;
mod plate;
mod player;
mod probe;
mod respawn;
mod restart;
mod robots;
mod setup;
mod shapes;
mod state;
mod status;
mod terrain;
mod tests;
mod unit_scripts;
mod units;
mod walls;

spring_addons::export_rules_gadgets! {
    state: state::Global,
    setup: gadgets::setup,
    scratch: 4096,
}

spring::export_core_cus!(unit_scripts::GravitasCusModule);
