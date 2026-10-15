use spring::UnitId;
use spring::cus::host::{CusHost, UnitDefScript};
use spring::cus::{InitCtx, ScriptCapabilities};

use super::{GravitasUnitScript, JumpCall, capabilities};

pub type GravitasCusModule = CusHost<GravitasUnitScript>;

impl UnitDefScript for GravitasUnitScript {
    fn capabilities(def: i32) -> Option<ScriptCapabilities> {
        crate::defs::get().unit(def).script.map(capabilities)
    }

    fn for_def(def: i32, ctx: &mut InitCtx<'_>, _: &mut ()) -> Self {
        let kind = crate::defs::get().unit(def).script;
        Self::for_kind(
            kind.expect("CusHost builds scripts only for defs with one"),
            ctx,
        )
    }
}

pub fn attach(unit: i32, def: i32) {
    with_host("attaching a script", move |host| {
        host.unit_created(UnitId(unit), def)
    });
}

pub fn attach_existing() {
    with_host("attaching the scripts of existing units", |host| {
        host.attach_existing()
    });
}

pub fn game_frame(frame: i32) {
    with_host("waking the scripts", move |host| host.game_frame(frame));
}

pub fn jump(unit: i32, call: JumpCall) {
    with_host("a jump", move |host| {
        let handled = host.with_script(UnitId(unit), |ctx, script| script.jump(ctx, call));
        if handled != Some(true) {
            spring::log_warning!(
                "gravitas-wasm",
                "unit {unit} has no script that jumps ({call:?})"
            );
        }
    });
}

fn with_host(what: &str, f: impl FnOnce(&mut GravitasCusModule)) {
    if crate::with_cus_module(f).is_none() {
        spring::log_warning!("gravitas-wasm", "unit scripts busy or not loaded: {what}");
    }
}
