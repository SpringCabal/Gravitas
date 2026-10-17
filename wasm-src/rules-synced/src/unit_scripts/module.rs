use spring::cus::host::{CusHost, UnitDefScript};
use spring::cus::{InitCtx, ScriptCapabilities};
use spring::{Result, ResultExt, RulesSynced, Spring, UnitId};

use super::{GravitasUnitScript, JumpCall, capabilities};

pub type GravitasCusModule = CusHost<GravitasUnitScript>;

impl UnitDefScript for GravitasUnitScript {
    fn capabilities(spring: &Spring<'_, RulesSynced>, def: i32) -> Option<ScriptCapabilities> {
        crate::defs::get(spring).unit(def).script.map(capabilities)
    }
    fn for_def(
        spring: &Spring<'_, RulesSynced>,
        def: i32,
        ctx: &mut InitCtx<'_>,
        _: &mut (),
    ) -> Self {
        let kind = crate::defs::get(spring).unit(def).script;
        Self::for_kind(
            kind.expect("CusHost builds scripts only for defs with one"),
            ctx,
        )
    }
}

pub fn attach(spring: &Spring<'_, RulesSynced>, unit: i32, def: i32) {
    with_host(spring, "attaching a script", move |host, spring| {
        host.unit_created(spring, UnitId(unit), def)
    });
}

pub fn attach_existing(spring: &Spring<'_, RulesSynced>) {
    with_host(
        spring,
        "attaching the scripts of existing units",
        |host, spring| host.attach_existing(spring),
    );
}

pub fn game_frame(spring: &Spring<'_, RulesSynced>, frame: i32) {
    with_host(spring, "waking the scripts", move |host, _| {
        host.game_frame(frame);
        Ok(())
    });
}

pub fn jump(spring: &Spring<'_, RulesSynced>, unit: i32, call: JumpCall) {
    with_host(spring, "a jump", move |host, spring| {
        let handled =
            host.with_script(spring, UnitId(unit), |ctx, script| script.jump(ctx, call))?;
        if handled != Some(true) {
            spring::log_warning!(
                spring,
                "gravitas-wasm",
                "unit {unit} has no script that jumps ({call:?})"
            );
        }
        Ok(())
    });
}

fn with_host(
    spring: &Spring<'_, RulesSynced>,
    what: &str,
    f: impl FnOnce(&GravitasCusModule, &Spring<'_, RulesSynced>) -> Result<()>,
) {
    crate::with_cus_module(|host, spring| f(host, spring)).log_err_once(spring, what);
}
