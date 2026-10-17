use alloc::vec::Vec;
use springcabal::defs::DefCache;

#[derive(Clone, Copy, Default)]
pub struct UnitDef {
    pub healthbar: bool,
    pub effect: bool,
}

static DEFS: DefCache<Vec<UnitDef>> = DefCache::new();

pub fn unit(spring: &spring::Spring<'_, spring::Ui>, unit_def_id: i32) -> UnitDef {
    let defs = DEFS.get_or_init(|| load(spring));
    usize::try_from(unit_def_id)
        .ok()
        .and_then(|index| defs.get(index))
        .copied()
        .unwrap_or_default()
}

fn load(spring: &spring::Spring<'_, spring::Ui>) -> Vec<UnitDef> {
    let count = spring.unit_defs().get_unit_def_count().unwrap_or(0) as i32;
    (0..=count)
        .map(|unit_def_id| {
            let keys = spring
                .unit_defs()
                .get_unit_def_custom_param_keys(unit_def_id)
                .unwrap_or_default();
            let has = |key: &str| keys.iter().any(|candidate| candidate == key);
            UnitDef {
                healthbar: has("player") || has("robot") || has("barrel"),
                effect: has("effect"),
            }
        })
        .collect()
}
