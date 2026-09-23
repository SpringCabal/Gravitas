//! Per-def customParams the draw and unit callins need, read once.

use alloc::boxed::Box;
use alloc::vec::Vec;
use core::cell::OnceCell;

#[derive(Clone, Copy, Default)]
pub struct UnitDef {
    /// Players, robots and barrels get healthbars; map machinery does not.
    pub healthbar: bool,
    /// Fire and electric fields: drawn as particles, not as models.
    pub effect: bool,
}

thread_local! {
    static DEFS: OnceCell<&'static [UnitDef]> = const { OnceCell::new() };
}

pub fn unit(unit_def_id: i32) -> UnitDef {
    let defs = DEFS.with(|defs| *defs.get_or_init(load));
    usize::try_from(unit_def_id)
        .ok()
        .and_then(|index| defs.get(index))
        .copied()
        .unwrap_or_default()
}

fn load() -> &'static [UnitDef] {
    let count = spring::unit_defs::get_unit_def_count(0).unwrap_or(0) as i32;
    // Unit def ids start at 1; index 0 stays empty.
    let defs: Vec<UnitDef> = (0..=count)
        .map(|unit_def_id| {
            let keys =
                spring::unit_defs::get_unit_def_custom_param_keys(unit_def_id).unwrap_or_default();
            let has = |key: &str| keys.iter().any(|candidate| candidate == key);
            UnitDef {
                healthbar: has("player") || has("robot") || has("barrel"),
                effect: has("effect"),
            }
        })
        .collect();
    Box::leak(defs.into_boxed_slice())
}
