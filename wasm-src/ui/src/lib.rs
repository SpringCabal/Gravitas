extern crate alloc;

mod defs;
mod diagnostics;
mod health;
mod state;
mod widgets;

spring_addons::export_ui_widgets! {
    state: state::Global,
    setup: widgets::setup,
    scratch: 4096,
}

spring::export_jobs!(HANDLER, { LOCAL_FIRE_VERTICES => ("fire-vertices", widgets::gfx_fire::fire::jobs::worker) });
pub const FIRE_VERTICES: spring::jobs::Worker = LOCAL_FIRE_VERTICES.in_wasm_module("gravitas-jobs");
