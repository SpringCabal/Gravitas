pub struct DefParams {
    pub name: &'static str,
    pub params: &'static [(&'static str, &'static str)],
}

pub const DEF_PARAMS: [DefParams; 16] = [
    DefParams {
        name: "barrel",
        params: &[
            ("barrel", "true"),
            ("nonparalyzable", "1"),
            ("radius", "20"),
        ],
    },
    DefParams {
        name: "bob",
        params: &[("invulnerable", "1"), ("radius", "30"), ("robot", "true")],
    },
    DefParams {
        name: "canister",
        params: &[
            ("barrel", "true"),
            ("nonparalyzable", "1"),
            ("radius", "10"),
        ],
    },
    DefParams {
        name: "cometopapa",
        params: &[("radius", "50"), ("robot", "true")],
    },
    DefParams {
        name: "crate",
        params: &[
            ("invulnerable", "1"),
            ("nonparalyzable", "1"),
            ("plate_toggler", "true"),
            ("radius", "30"),
        ],
    },
    DefParams {
        name: "electrafi",
        params: &[
            ("effect", "true"),
            ("electric_field", "true"),
            ("invulnerable", "1"),
        ],
    },
    DefParams {
        name: "fire",
        params: &[
            ("effect", "true"),
            ("fire", "true"),
            ("invulnerable", "1"),
            ("nonparalyzable", "1"),
        ],
    },
    DefParams {
        name: "gate",
        params: &[
            ("gate", "true"),
            ("invulnerable", "1"),
            ("nonparalyzable", "1"),
            ("radius", "260"),
            ("wall", "true"),
        ],
    },
    DefParams {
        name: "gravit",
        params: &[("player", "true"), ("radius", "20")],
    },
    DefParams {
        name: "plate",
        params: &[("invulnerable", "1"), ("nonparalyzable", "1")],
    },
    DefParams {
        name: "projector",
        params: &[("radius", "15"), ("robot", "true")],
    },
    DefParams {
        name: "wall1",
        params: &[
            ("invulnerable", "1"),
            ("nonparalyzable", "1"),
            ("radius", "490"),
            ("wall", "true"),
        ],
    },
    DefParams {
        name: "wall2",
        params: &[
            ("invulnerable", "1"),
            ("nonparalyzable", "1"),
            ("radius", "260"),
            ("wall", "true"),
        ],
    },
    DefParams {
        name: "wall3",
        params: &[
            ("invulnerable", "1"),
            ("nonparalyzable", "1"),
            ("radius", "160"),
            ("wall", "true"),
        ],
    },
    DefParams {
        name: "wall4",
        params: &[
            ("invulnerable", "1"),
            ("nonparalyzable", "1"),
            ("radius", "160"),
            ("wall", "true"),
        ],
    },
    DefParams {
        name: "wallier",
        params: &[("radius", "27"), ("robot", "true")],
    },
];
