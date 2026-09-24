//! The e2e scenario's messages. Only the e2e builds send them: the rules'
//! cases ask the ui module for camera moves, screenshots and scripted input,
//! and the ui module reports each screenshot taken.

use alloc::string::String;

use springcabal::message::{Encoder, command, fields};

const SHOT: &str = "GRAVITAS_SHOT";
const INPUT: &str = "GRAVITAS_E2E";
const TAKEN: &str = "gravitas_e2e";

/// Rules to ui.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Request<'a> {
    /// Point the camera straight at `x`, `z`, from `height` if given.
    Look { x: f32, z: f32, height: Option<f32> },
    /// Take a screenshot and report it as `name`.
    Shot(&'a str),
    /// The scenario is over: quit.
    Done,
    /// Point the scripted cursor at the ground there; `jump` holds the jump
    /// button.
    Pointer { target: [f32; 3], jump: bool },
    /// Give the mouse back.
    PointerOff,
    /// Press the HUD's restart button.
    Restart,
}

impl Request<'_> {
    pub fn encode(&self) -> String {
        match *self {
            Request::Look { x, z, height } => {
                let message = Encoder::new(SHOT).field("look").field(x).field(z);
                match height {
                    Some(height) => message.field(height),
                    None => message,
                }
                .finish()
            }
            Request::Shot(name) => Encoder::new(SHOT).field("shot").field(name).finish(),
            Request::Done => Encoder::new(SHOT).field("done").finish(),
            Request::Pointer { target, jump } => Encoder::new(INPUT)
                .field("pointer")
                .fields(target)
                .flag(jump)
                .finish(),
            Request::PointerOff => Encoder::new(INPUT).field("pointer").field("off").finish(),
            Request::Restart => Encoder::new(INPUT).field("restart").finish(),
        }
    }

    pub fn parse(message: &str) -> Option<Request<'_>> {
        if let Some(mut fields) = fields(message, SHOT) {
            return match fields.next()? {
                "look" => {
                    let (x, z) = fields.xz()?;
                    let height = if fields.is_done() {
                        None
                    } else {
                        Some(fields.f32()?)
                    };
                    Some(Request::Look { x, z, height })
                }
                "shot" => Some(Request::Shot(fields.rest()?)),
                "done" => Some(Request::Done),
                _ => None,
            };
        }
        let mut fields = fields(message, INPUT)?;
        match fields.next()? {
            "restart" => Some(Request::Restart),
            "pointer" if fields.clone().next() == Some("off") => Some(Request::PointerOff),
            "pointer" => {
                let (x, y, z) = fields.xyz()?;
                Some(Request::Pointer {
                    target: [x, y, z],
                    jump: fields.flag()?,
                })
            }
            _ => None,
        }
    }
}

/// Ui to rules: the screenshot `name` was taken.
pub fn shot_taken(name: &str) -> String {
    Encoder::new(TAKEN).field("shot").field(name).finish()
}

pub fn parse_shot_taken(message: &str) -> Option<&str> {
    let (prefix, mut fields) = command(message)?;
    if prefix != TAKEN || fields.next()? != "shot" {
        return None;
    }
    fields.rest()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn requests_round_trip() {
        let requests = [
            Request::Look {
                x: 10.0,
                z: 20.5,
                height: Some(1500.0),
            },
            Request::Look {
                x: 10.0,
                z: 20.5,
                height: None,
            },
            Request::Shot("arena"),
            Request::Done,
            Request::Pointer {
                target: [1.0, 2.0, 3.0],
                jump: true,
            },
            Request::PointerOff,
            Request::Restart,
        ];
        for request in requests {
            assert_eq!(Request::parse(&request.encode()), Some(request));
        }
        assert_eq!(Request::Shot("arena").encode(), "GRAVITAS_SHOT|shot|arena");
        assert_eq!(Request::parse("GRAVITAS_SHOT|look|1"), None);
        assert_eq!(Request::parse("GRAVITAS_FX|flame|1|2|3|4"), None);
    }

    #[test]
    fn shot_taken_round_trips() {
        assert_eq!(shot_taken("arena"), "gravitas_e2e|shot|arena");
        assert_eq!(parse_shot_taken(&shot_taken("arena")), Some("arena"));
        assert_eq!(parse_shot_taken("gravitas|shot|arena"), None);
    }
}
