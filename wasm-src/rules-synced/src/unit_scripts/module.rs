//! The Core-CUS module, and how rules code in the same module attaches and
//! calls its scripts.

use spring::UnitId;
use spring::cus::host::{CusHost, UnitDefScript};
use spring::cus::{InitCtx, ScriptCapabilities};

use super::{GravitasUnitScript, capabilities};

/// The module exported to the engine: `CusHost` attaches, ticks and
/// dispatches the scripts.
pub type GravitasCusModule = CusHost<GravitasUnitScript>;

impl UnitDefScript for GravitasUnitScript {
    fn capabilities(def: i32) -> Option<ScriptCapabilities> {
        crate::defs::get().unit(def).script.map(capabilities)
    }

    fn for_def(def: i32, ctx: &mut InitCtx<'_>) -> Self {
        Self::for_kind(crate::defs::get().unit(def).script, ctx)
    }
}

/// Attach the script of a unit that was just created, or that existed before
/// the game started. Deferred while the CUS state is in use further up the
/// stack (a unit created from inside a unit script).
pub fn attach(unit: i32, def: i32) {
    crate::with_cus_module_or_defer(move |host| host.unit_created(UnitId(unit), def));
}

/// Attach the scripts of every unit that exists now.
pub fn attach_existing() {
    crate::with_cus_module_or_defer(|host| host.attach_existing());
}

/// Run a named unit-script function, like Lua's `env.Function()`. Deferred in
/// the same cases as [`attach`].
pub fn call(unit: i32, function_name: &'static str) {
    crate::with_cus_module_or_defer(move |host| {
        host.call(UnitId(unit), function_name, &[], &mut []);
    });
}
