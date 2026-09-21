pub struct ImpulseWeapon {
    pub name: &'static str,
    pub impulse: f32,
}

pub const IMPULSE_WEAPONS: [ImpulseWeapon; 2] = [
    ImpulseWeapon {
        name: "gravity_neg",
        impulse: -100.0,
    },
    ImpulseWeapon {
        name: "gravity_pos",
        impulse: 100.0,
    },
];
