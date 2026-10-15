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
