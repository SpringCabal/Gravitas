//! What the rules and ui modules must agree on: the messages between them.
//! Rules to ui: [`Status`], [`Effect`] and [`game_over`]. Ui to rules:
//! [`Input`] and [`RESTART`]. The e2e scenario's messages are in [`e2e`].

#![no_std]

extern crate alloc;

pub mod e2e;

use alloc::string::String;

use springcabal::message::{Encoder, fields, key_values};

/// Ui to rules: start the level again (the end panel's restart button).
pub const RESTART: &str = "gravitas_restart";

const STATUS: &str = "GRAVITAS_STATUS";
const EFFECT: &str = "GRAVITAS_FX";
const GAME_OVER: &str = "GRAVITAS_OVER";
const INPUT: &str = "gravitas";

/// Rules to ui: what the HUD shows. Sent when it changes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status {
    /// The unit the player controls, if there is one.
    pub player_unit: Option<i32>,
    pub health: i32,
    pub max_health: i32,
    /// Whether the push and pull guns are loaded.
    pub push_ready: bool,
    pub pull_ready: bool,
}

impl Status {
    pub fn encode(&self) -> String {
        Encoder::new(STATUS)
            .kv("playerUnit", self.player_unit.unwrap_or(-1))
            .kv("health", self.health)
            .kv("maxHealth", self.max_health)
            .kv_flag("push", self.push_ready)
            .kv_flag("pull", self.pull_ready)
            .finish()
    }

    /// `None` unless `message` is a status with every field.
    pub fn parse(message: &str) -> Option<Self> {
        let (mut player_unit, mut health, mut max_health) = (None, None, None);
        let (mut push_ready, mut pull_ready) = (None, None);
        for (key, value) in key_values(message, STATUS)? {
            match key {
                "playerUnit" => player_unit = value.parse::<i32>().ok(),
                "health" => health = value.parse().ok(),
                "maxHealth" => max_health = value.parse().ok(),
                "push" => push_ready = Some(value != "0"),
                "pull" => pull_ready = Some(value != "0"),
                _ => {}
            }
        }
        Some(Self {
            player_unit: Some(player_unit?).filter(|unit| *unit >= 0),
            health: health?,
            max_health: max_health?,
            push_ready: push_ready?,
            pull_ready: pull_ready?,
        })
    }
}

/// Rules to ui: the game is over, won or lost.
pub fn game_over(victory: bool) -> String {
    Encoder::new(GAME_OVER).flag(victory).finish()
}

/// Whether `message` ends the game, and with a victory.
pub fn parse_game_over(message: &str) -> Option<bool> {
    fields(message, GAME_OVER)?.flag()
}

/// Rules to ui: a fire or electric effect to draw at `position`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Effect {
    pub position: [f32; 3],
    pub kind: EffectKind,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EffectKind {
    /// A burning unit of this radius; `strength` fades from 1 as the burn
    /// runs out; the unit's horizontal velocity drifts the flames.
    Burn {
        radius: f32,
        strength: f32,
        velocity: [f32; 2],
    },
    /// A fire (or a barrel's blast) of this radius.
    Flame { radius: f32 },
    /// An electric field of this radius.
    Arc { radius: f32 },
    /// A spark on a shocked unit, scattered over `spread` around it.
    Spark {
        intensity: f32,
        velocity: [f32; 2],
        spread: f32,
    },
}

impl Effect {
    /// `GRAVITAS_FX|kind|x|y|z|values...`
    pub fn encode(&self) -> String {
        let [x, y, z] = self.position;
        let message = Encoder::new(EFFECT);
        let message = match self.kind {
            EffectKind::Burn { .. } => message.field("burn"),
            EffectKind::Flame { .. } => message.field("flame"),
            EffectKind::Arc { .. } => message.field("arc"),
            EffectKind::Spark { .. } => message.field("spark"),
        };
        let message = message.field(x).field(y).field(z);
        match self.kind {
            EffectKind::Burn {
                radius,
                strength,
                velocity,
            } => message.fields([radius, strength, velocity[0], velocity[1]]),
            EffectKind::Flame { radius } | EffectKind::Arc { radius } => message.field(radius),
            EffectKind::Spark {
                intensity,
                velocity,
                spread,
            } => message.fields([intensity, velocity[0], velocity[1], spread]),
        }
        .finish()
    }

    pub fn parse(message: &str) -> Option<Self> {
        let mut fields = fields(message, EFFECT)?;
        let kind = fields.next()?;
        let (x, y, z) = fields.xyz()?;
        let kind = match kind {
            "burn" => EffectKind::Burn {
                radius: fields.f32()?,
                strength: fields.f32()?,
                velocity: [fields.f32()?, fields.f32()?],
            },
            "flame" => EffectKind::Flame {
                radius: fields.f32()?,
            },
            "arc" => EffectKind::Arc {
                radius: fields.f32()?,
            },
            "spark" => EffectKind::Spark {
                intensity: fields.f32()?,
                velocity: [fields.f32()?, fields.f32()?],
                spread: fields.f32()?,
            },
            _ => return None,
        };
        Some(Self {
            position: [x, y, z],
            kind,
        })
    }

    /// How far the effect reaches, for the ui's visibility check.
    pub fn magnitude(&self) -> f32 {
        match self.kind {
            EffectKind::Burn { radius, .. }
            | EffectKind::Flame { radius }
            | EffectKind::Arc { radius } => radius,
            EffectKind::Spark { intensity, .. } => intensity,
        }
    }
}

/// Ui to rules: the player's input. Sent when it changes.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Input {
    /// The held movement keys, each -1, 0 or 1.
    pub move_x: f32,
    pub move_z: f32,
    /// Where the cursor points, and the unit under it (-1 for none).
    pub target: [f32; 3],
    pub target_unit: i32,
    pub push: bool,
    pub pull: bool,
    /// Jump was pressed: set only on the message sent when the key went down.
    pub jump: bool,
}

impl Input {
    /// `gravitas|move_x|move_z|target_x|target_y|target_z|target_unit|push|pull|jump`
    pub fn encode(&self) -> String {
        Encoder::new(INPUT)
            .fields([self.move_x, self.move_z])
            .fields(self.target)
            .field(self.target_unit)
            .flag(self.push)
            .flag(self.pull)
            .flag(self.jump)
            .finish()
    }

    pub fn parse(message: &str) -> Option<Self> {
        let mut fields = fields(message, INPUT)?;
        let input = Self {
            move_x: fields.f32()?,
            move_z: fields.f32()?,
            target: {
                let (x, y, z) = fields.xyz()?;
                [x, y, z]
            },
            target_unit: fields.i32()?,
            push: fields.flag()?,
            pull: fields.flag()?,
            jump: fields.flag()?,
        };
        fields.is_done().then_some(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_round_trips() {
        let status = Status {
            player_unit: Some(42),
            health: 1200,
            max_health: 1600,
            push_ready: true,
            pull_ready: false,
        };
        assert_eq!(
            status.encode(),
            "GRAVITAS_STATUS|playerUnit=42|health=1200|maxHealth=1600|push=1|pull=0"
        );
        assert_eq!(Status::parse(&status.encode()), Some(status));
        let unitless = Status {
            player_unit: None,
            ..status
        };
        assert_eq!(Status::parse(&unitless.encode()), Some(unitless));
    }

    #[test]
    fn status_needs_every_field() {
        assert_eq!(Status::parse("GRAVITAS_STATUS|playerUnit=1|health=3"), None);
        assert_eq!(Status::parse("GRAVITAS_FX|flame|1|2|3|4"), None);
    }

    #[test]
    fn game_over_round_trips() {
        assert_eq!(parse_game_over(&game_over(true)), Some(true));
        assert_eq!(parse_game_over(&game_over(false)), Some(false));
        assert_eq!(parse_game_over("GRAVITAS_STATUS|1"), None);
    }

    #[test]
    fn effects_round_trip() {
        let kinds = [
            EffectKind::Burn {
                radius: 20.0,
                strength: 0.5,
                velocity: [1.5, -2.0],
            },
            EffectKind::Flame { radius: 250.0 },
            EffectKind::Arc { radius: 100.0 },
            EffectKind::Spark {
                intensity: 0.25,
                velocity: [0.0, 3.0],
                spread: 12.5,
            },
        ];
        for kind in kinds {
            let effect = Effect {
                position: [10.0, 144.25, -3.5],
                kind,
            };
            assert_eq!(Effect::parse(&effect.encode()), Some(effect));
        }
        assert_eq!(
            Effect {
                position: [1.0, 2.0, 3.0],
                kind: EffectKind::Flame { radius: 4.0 },
            }
            .encode(),
            "GRAVITAS_FX|flame|1|2|3|4"
        );
    }

    #[test]
    fn effects_reject_bad_fields() {
        assert_eq!(Effect::parse("GRAVITAS_FX|flame|1|2|3"), None);
        assert_eq!(Effect::parse("GRAVITAS_FX|smoke|1|2|3|4"), None);
        assert_eq!(Effect::parse("GRAVITAS_FX|flame|1|NaN|3|4"), None);
    }

    #[test]
    fn input_round_trips() {
        let input = Input {
            move_x: -1.0,
            move_z: 1.0,
            target: [100.5, 20.0, -7.25],
            target_unit: -1,
            push: true,
            pull: false,
            jump: true,
        };
        assert_eq!(input.encode(), "gravitas|-1|1|100.5|20|-7.25|-1|1|0|1");
        assert_eq!(Input::parse(&input.encode()), Some(input));
    }

    #[test]
    fn input_is_not_the_restart_or_a_short_message() {
        assert_eq!(Input::parse(RESTART), None);
        assert_eq!(Input::parse("gravitas|0|0|1|2|3|-1|0|0"), None);
        assert_eq!(Input::parse("gravitas|0|0|1|2|3|-1|0|0|0|9"), None);
        assert_eq!(Input::parse("gravitas|inf|0|1|2|3|-1|0|0|0"), None);
    }
}
