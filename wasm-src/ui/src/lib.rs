extern crate alloc;

mod camera;
mod diagnostics;
mod fire;
mod fx;
mod healthbars;
mod hud;
mod input;
mod mapedge;
mod music;
mod selection;
mod shots;
mod state;
mod widgets;

spring_addons::export_ui_widgets! {
    state: state::Global,
    setup: widgets::setup,
    scratch: 4096,
}
