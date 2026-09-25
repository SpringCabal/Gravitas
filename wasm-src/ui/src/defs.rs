//! Per-def customParams the draw and unit callins need, read once.

use alloc::vec::Vec;
use springcabal::defs::DefCache;

#[derive(Clone, Copy, Default)]
pub struct UnitDef {
    /// Players, robots and barrels get healthbars; map machinery does not.
    pub healthbar: bool,
    /// Fire and electric fields: drawn as particles, not as models.
    pub effect: bool,
}

static DEFS: DefCache<Vec<UnitDef>> = DefCache::new(load);

pub fn unit(unit_def_id: i32) -> UnitDef {
    let defs = DEFS.get();
    usize::try_from(unit_def_id)
        .ok()
        .and_then(|index| defs.get(index))
        .copied()
        .unwrap_or_default()
}

fn load() -> Vec<UnitDef> {
    let count = spring::unit_defs::get_unit_def_count(0).unwrap_or(0) as i32;
    // Unit def ids start at 1; index 0 stays empty.
    (0..=count)
        .map(|unit_def_id| {
            let keys =
                spring::unit_defs::get_unit_def_custom_param_keys(unit_def_id).unwrap_or_default();
            let has = |key: &str| keys.iter().any(|candidate| candidate == key);
            UnitDef {
                healthbar: has("player") || has("robot") || has("barrel"),
                effect: has("effect"),
            }
        })
        .collect()
}
