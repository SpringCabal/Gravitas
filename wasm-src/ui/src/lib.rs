extern crate alloc;

mod camera;
mod defs;
mod diagnostics;
mod fire;
mod fx;
mod health;
mod healthbars;
mod hud;
mod input;
mod mapedge;
mod selection;
mod shots;
mod state;
mod widgets;

spring_addons::export_ui_widgets! {
    state: state::Global,
    setup: widgets::setup,
    scratch: 4096,
}
