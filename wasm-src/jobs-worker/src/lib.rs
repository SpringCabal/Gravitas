extern crate alloc;
#[allow(dead_code)]
#[path = "../../ui/src/widgets/gfx_fire/fire/geometry.rs"]
mod geometry;
#[allow(dead_code)]
#[path = "../../ui/src/widgets/gfx_fire/fire/particles.rs"]
mod particles;
use geometry::axes;
use particles::{Particle, Texture, sample_with_colors};

#[path = "../../ui/src/widgets/gfx_fire/fire/jobs/worker.rs"]
mod work;
struct Pure;
impl spring::module::Module<spring::Ui> for Pure {
    fn initialize(_: &mut spring::Spring<'_, spring::Ui>) -> spring::Result<Self> {
        Ok(Self)
    }
}
spring::export_module!(APP, spring::Ui, Pure);
spring::export_jobs!(APP, { RUN => ("fire-vertices", work::worker) });
