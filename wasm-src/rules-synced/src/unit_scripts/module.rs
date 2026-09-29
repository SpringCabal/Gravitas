//! The Core-CUS module, and how rules code in the same module attaches and
//! calls its scripts.

use spring::UnitId;
use spring::cus::host::{CusHost, UnitDefScript};
use spring::cus::{InitCtx, ScriptCapabilities};

use super::{GravitasUnitScript, JumpCall, capabilities};

/// The module exported to the engine: `CusHost` attaches, ticks and
/// dispatches the scripts.
pub type GravitasCusModule = CusHost<GravitasUnitScript>;

impl UnitDefScript for GravitasUnitScript {
    fn capabilities(def: i32) -> Option<ScriptCapabilities> {
        crate::defs::get().unit(def).script.map(capabilities)
    }

    /// Only called for defs `capabilities` gave a script.
    fn for_def(def: i32, ctx: &mut InitCtx<'_>, _: &mut ()) -> Self {
        let kind = crate::defs::get().unit(def).script;
        Self::for_kind(
            kind.expect("CusHost builds scripts only for defs with one"),
            ctx,
        )
    }
}

/// Attach the script of a unit that was just created.
pub fn attach(unit: i32, def: i32) {
    with_host("attaching a script", move |host| {
        host.unit_created(UnitId(unit), def)
    });
}

/// Attach the scripts of units that exist when the rules load (after a rules
/// reload).
pub fn attach_existing() {
    with_host("attaching the scripts of existing units", |host| {
        host.attach_existing()
    });
}

/// Wake the scripts' sleeping tasks for `frame`.
pub fn game_frame(frame: i32) {
    with_host("waking the scripts", move |host| host.game_frame(frame));
}

/// Run one of the jump functions of a unit's script, as `unit_jumpjets.lua`
/// does through `env`.
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

/// Run `f` on the scripts. Rules code never runs inside a script: the engine
/// calls that raise callins are commands, which run once the scripts have
/// returned.
fn with_host(what: &str, f: impl FnOnce(&mut GravitasCusModule)) {
    if crate::with_cus_module(f).is_none() {
        spring::log_warning!("gravitas-wasm", "unit scripts busy or not loaded: {what}");
    }
}
