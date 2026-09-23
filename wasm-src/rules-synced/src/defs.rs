//! Unit and weapon defs, resolved once: ids for the defs the rules refer to by
//! name, and the per-def customParams and values the hot paths need.

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use core::cell::OnceCell;

use crate::unit_scripts::ScriptKind;

pub struct Defs {
    pub gravit: Option<i32>,
    pub plate: Option<i32>,
    pub gate: Option<i32>,
    pub fire: Option<i32>,
    pub electrafi: Option<i32>,
    units: Vec<UnitDef>,
    /// The `impulse` customParam, by weapon def id.
    weapon_impulses: Vec<Option<f32>>,
}

/// What the rules need to know about one unit def.
#[derive(Default)]
pub struct UnitDef {
    pub name: String,
    pub player: bool,
    pub robot: bool,
    /// Scenery drawn as particles (fire, electric fields): never an obstacle.
    pub effect: bool,
    pub barrel: bool,
    pub wall: bool,
    pub gate: bool,
    pub plate_toggler: bool,
    pub respawnable: bool,
    /// customParams `invulnerable == 1`.
    pub invulnerable: bool,
    /// customParams `nonparalyzable == 1`.
    pub nonparalyzable: bool,
    pub radius: Option<f32>,
    pub elasticity: Option<f32>,
    pub friction: Option<f32>,
    pub mass: f32,
    pub health: f32,
    pub speed: f32,
    pub script: Option<ScriptKind>,
}

static NO_DEF: UnitDef = UnitDef {
    name: String::new(),
    player: false,
    robot: false,
    effect: false,
    barrel: false,
    wall: false,
    gate: false,
    plate_toggler: false,
    respawnable: false,
    invulnerable: false,
    nonparalyzable: false,
    radius: None,
    elasticity: None,
    friction: None,
    mass: 0.0,
    health: 0.0,
    speed: 0.0,
    script: None,
};

thread_local! {
    static DEFS: OnceCell<&'static Defs> = const { OnceCell::new() };
}

/// The resolved defs. Loaded on first use; defs never change during a game.
pub fn get() -> &'static Defs {
    DEFS.with(|defs| *defs.get_or_init(|| Box::leak(Box::new(Defs::load()))))
}

impl Defs {
    /// A unit def by id; an empty def for an unknown id.
    pub fn unit(&self, unit_def_id: i32) -> &UnitDef {
        usize::try_from(unit_def_id)
            .ok()
            .and_then(|index| self.units.get(index))
            .unwrap_or(&NO_DEF)
    }

    /// The id of the unit def called `name`.
    pub fn id(&self, name: &str) -> Option<i32> {
        self.units
            .iter()
            .position(|def| def.name == name)
            .map(|index| index as i32)
    }

    pub fn weapon_impulse(&self, weapon_def_id: i32) -> Option<f32> {
        usize::try_from(weapon_def_id)
            .ok()
            .and_then(|index| self.weapon_impulses.get(index))
            .copied()
            .flatten()
    }

    fn load() -> Self {
        let count = spring::unit_defs::get_unit_def_count(0).unwrap_or(0) as i32;
        // Unit def ids start at 1; index 0 stays empty.
        let units: Vec<UnitDef> = (0..=count).map(UnitDef::load).collect();
        let weapons = spring::weapon_defs::get_weapon_def_count(0).unwrap_or(0) as i32;
        let weapon_impulses = (0..weapons)
            .map(|weapon_def_id| {
                spring::weapon_defs::get_weapon_def_custom_param(weapon_def_id, "impulse")
                    .ok()
                    .and_then(|value| value.parse().ok())
            })
            .collect();
        let mut defs = Self {
            gravit: None,
            plate: None,
            gate: None,
            fire: None,
            electrafi: None,
            units,
            weapon_impulses,
        };
        defs.gravit = defs.id("gravit");
        defs.plate = defs.id("plate");
        defs.gate = defs.id("gate");
        defs.fire = defs.id("fire");
        defs.electrafi = defs.id("electrafi");
        defs
    }
}

impl UnitDef {
    fn load(unit_def_id: i32) -> Self {
        let Some(name) = spring::unit_defs::get_unit_def_name_bytes(unit_def_id)
            .ok()
            .and_then(|bytes| String::from_utf8(bytes).ok())
        else {
            return Self::default();
        };
        let keys =
            spring::unit_defs::get_unit_def_custom_param_keys(unit_def_id).unwrap_or_default();
        let has = |key: &str| keys.iter().any(|candidate| candidate == key);
        let number = |key: &str| -> Option<f32> {
            if !has(key) {
                return None;
            }
            spring::unit_defs::get_unit_def_custom_param(unit_def_id, key)
                .ok()
                .and_then(|value| value.parse().ok())
        };
        Self {
            player: has("player"),
            robot: has("robot"),
            effect: has("effect"),
            barrel: has("barrel"),
            wall: has("wall"),
            gate: has("gate"),
            plate_toggler: has("plate_toggler"),
            respawnable: has("respawnable"),
            invulnerable: number("invulnerable") == Some(1.0),
            nonparalyzable: number("nonparalyzable") == Some(1.0),
            radius: number("radius"),
            elasticity: number("elasticity"),
            friction: number("friction"),
            mass: spring::get_unit_def_param_float(unit_def_id, "mass").unwrap_or(0.0),
            health: spring::get_unit_def_health(unit_def_id).unwrap_or(0.0),
            speed: spring::get_unit_def_speed(unit_def_id).unwrap_or(0.0),
            script: ScriptKind::for_def_name(&name),
            name,
        }
    }
}
