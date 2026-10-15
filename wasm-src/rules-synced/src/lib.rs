extern crate alloc;

mod defs;
mod diagnostics;
mod effects;
mod gadgets;
mod hazard;
mod state;
mod tests;
mod unit_scripts;
mod units;

spring_addons::export_rules_gadgets! {
    state: state::Global,
    setup: gadgets::setup,
    scratch: 4096,
}

spring::export_core_cus!(unit_scripts::GravitasCusModule);
