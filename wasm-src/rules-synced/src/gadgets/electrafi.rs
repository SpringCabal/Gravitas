pub mod behavior;
use spring::ResultExt;

use spring_addons::AddonContext;
use spring_addons::rules::{Gadget, UnitDestroyedEvent};

use gravitas_shared::EffectKind;

use self::behavior::Electrafi;
use crate::effects::send_effect;
use crate::state::{Global, PerLevel};
use crate::units;

#[derive(Default)]
pub struct ElectrafiGadget {
    electrafi: PerLevel<Electrafi>,
}

impl Gadget<Global> for ElectrafiGadget {
    fn name(&self) -> &'static str {
        "Electrafi"
    }

    fn unit_created(
        &self,
        ctx: &AddonContext<'_, Global>,
        unit_id: i32,
        unit_def_id: i32,
        _team: i32,
        _builder: i32,
    ) {
        self.electrafi
            .get(ctx.global())
            .created(ctx.spring(), unit_id, unit_def_id);
    }

    fn unit_destroyed(&self, ctx: &AddonContext<'_, Global>, event: &UnitDestroyedEvent) {
        self.electrafi.get(ctx.global()).destroyed(event.unit_id);
    }

    fn game_frame(&self, ctx: &AddonContext<'_, Global>, frame: i32) {
        let spring = ctx.spring();
        let mut electrafi = self.electrafi.get(ctx.global());
        let shocks = electrafi.update(spring, frame);
        let arcs = electrafi.arcs(spring, frame);
        for shock in shocks {
            crate::with_addon_context(|ctx| {
                ctx.commands().run(move |spring| {
                    if !units::valid(spring, shock.unit_id) {
                        return;
                    }
                    if let Some(damage) = shock.damage {
                        let paralyze_time = if shock.paralyze { PARALYZING } else { 0.0 };
                        spring
                            .unit_control()
                            .add_unit_damage(
                                shock.unit_id,
                                damage,
                                paralyze_time,
                                -1,
                                shock.attacker_id,
                                spring::Float3::default(),
                            )
                            .log_err_once(spring, "electric field damage");
                    }
                    emit_spark(spring, shock.unit_id, shock.intensity);
                    crate::with_global(|global| {
                        crate::tests::note_spark(global, shock.unit_id, shock.damage.is_some());
                    });
                })
            });
        }
        crate::with_addon_context(|ctx| {
            ctx.commands().run(move |spring| {
                for arc in arcs {
                    send_effect(spring, arc.position, EffectKind::Arc { radius: arc.radius });
                }
            })
        });
    }
}

const PARALYZING: f32 = 1.0;

fn emit_spark(spring: &spring::Spring<'_, spring::RulesSynced>, unit_id: i32, intensity: f32) {
    let Some(position) = units::position(spring, unit_id) else {
        return;
    };
    let spread = spring.units_info().get_unit_radius(unit_id).unwrap_or(0.0) * 0.5;
    let velocity = spring
        .units_info()
        .get_unit_velocity(unit_id)
        .unwrap_or_default();
    send_effect(
        spring,
        position,
        EffectKind::Spark {
            intensity,
            velocity: [velocity.x, velocity.z],
            spread,
        },
    );
}
