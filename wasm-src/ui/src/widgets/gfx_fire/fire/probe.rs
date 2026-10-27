use core::cell::Cell;

use super::{Fire, emit};
use crate::state::{Clock, Global};

pub struct Probe {
    stage: Cell<i32>,
    shot: Cell<i32>,
    perf_emitters: Cell<i32>,
    count_particles: Cell<bool>,
}

impl Default for Probe {
    fn default() -> Self {
        Self {
            stage: Cell::new(-1),
            shot: Cell::new(-1),
            perf_emitters: Cell::new(-1),
            count_particles: Cell::new(false),
        }
    }
}

impl Probe {
    pub fn update(&self, spring: &spring::Spring<'_, spring::Ui>, global: &Global) {
        let _ = spring.unsynced_ctrl().warp_mouse(200, 500);
        crate::widgets::gui_camera::camera::look_at(spring, 4288.0, 600.0, Some(650.0));
        let frame = spring.game().current_frame().unwrap_or(0);
        if self.perf_emitters.get() == -1 {
            let enabled = spring
                .game()
                .get_mod_option("fire_perf")
                .is_ok_and(|option| option.value == "1");
            let count = if enabled {
                spring
                    .game()
                    .get_mod_option("fire_perf_emitters")
                    .ok()
                    .and_then(|option| option.value.parse::<i32>().ok())
                    .unwrap_or(1)
                    .max(0)
            } else {
                -2
            };
            self.perf_emitters.set(count);
            self.count_particles.set(
                spring
                    .game()
                    .get_mod_option("fire_diagnostic_counts")
                    .is_ok_and(|v| v.value == "1"),
            );
            if enabled {
                spring::log_notice!(spring, "FirePerf", "Rust emitters={count}");
            }
        }
        if self.perf_emitters.get() >= 0 {
            let previous = self.shot.replace(frame).max(0);
            let mut fire = global.fire.access_mut();
            for n in previous + 1..=frame {
                fire.advance(1.0);
                if n < 30 {
                    continue;
                }
                for _ in 0..self.perf_emitters.get() {
                    emit::raw_burn(
                        &mut fire,
                        spring::Float3 {
                            x: 4400.0,
                            y: 144.25,
                            z: 531.0,
                        },
                        25.0,
                        1.0,
                        [0.5, -0.25],
                        (n * 20) as u64,
                    );
                    if n % 15 == 0 {
                        emit::raw_flame(
                            &mut fire,
                            spring::Float3 {
                                x: 4288.0,
                                y: 144.25,
                                z: 531.0,
                            },
                            500.0,
                            0.5,
                            (n * 10) as u64,
                        );
                    }
                }
            }
            if self.count_particles.get() && previous != frame && matches!(frame, 300 | 600 | 899) {
                use super::particles::Texture;
                let live = |texture| {
                    fire.field
                        .gpu_particles(texture)
                        .iter()
                        .filter(|p| p.age > p.delay && p.age <= p.delay + p.life)
                        .count()
                };
                let normal: usize = [Texture::Flame, Texture::Electric, Texture::Smoke]
                    .into_iter()
                    .map(|texture| live(Some(texture)))
                    .sum();
                let distortion = live(None);
                spring::log_notice!(
                    spring,
                    "FireCounts",
                    "frame={frame} normal={normal} distortion={distortion}"
                );
            }
            global.clock.set(Clock { frame, elapsed: 0 });
            return;
        }
        if frame < 30 {
            return;
        }
        let stage = ((frame - 30) / 30).min(4);
        let target = [90, 105, 120, 150, 180][stage as usize];
        if self.stage.replace(stage) != stage {
            let mut fire = global.fire.access_mut();
            *fire = Fire::default();
            for n in 30..=target {
                fire.advance(1.0);
                emit::raw_burn(
                    &mut fire,
                    spring::Float3 {
                        x: 4400.0,
                        y: 144.25,
                        z: 531.0,
                    },
                    25.0,
                    1.0,
                    [0.5, -0.25],
                    (n * 20) as u64,
                );
                if n % 15 == 0 {
                    emit::raw_flame(
                        &mut fire,
                        spring::Float3 {
                            x: 4288.0,
                            y: 144.25,
                            z: 531.0,
                        },
                        500.0,
                        0.5,
                        (n * 10) as u64,
                    );
                }
            }
        }
        global.clock.set(Clock {
            frame: target,
            elapsed: 0,
        });
        if frame >= 45 + 30 * stage && self.shot.replace(stage) != stage {
            let _ = spring.messages().send_commands("screenshot png", "");
        }
        if frame >= 190 {
            let _ = spring.messages().send_commands("quitforce", "");
        }
    }
}
