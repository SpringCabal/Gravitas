use spring::gl;

use core::cell::Cell;

const GRID_TEXTURE: &str = "LuaUI/Images/vr_grid.png";
const GRID_RESOLUTION: f32 = 128.0;
const GRID_RANGE: f32 = 512.0;

/// The grid drawn beyond the map edge, mirroring the terrain and fading out.
#[derive(Default)]
pub struct MapEdge {
    list: Cell<u32>,
    setup_failed: Cell<bool>,
}

impl MapEdge {
    pub fn draw(&self) {
        let Some(list_id) = self.list() else {
            return;
        };
        let _ = spring::gfx::push_attrib(gl::ALL_ATTRIB_BITS);
        let _ = spring::gfx::depth_test(spring::gfx::GfxDepthTestOptions {
            enable: true,
            set_func: true,
            func: gl::LEQUAL,
        });
        let _ = spring::gfx::depth_mask(true);
        let _ = spring::gfx::bind_texture(GRID_TEXTURE, 0, true);
        let _ = spring::gfx::call_list(list_id);
        let _ = spring::gfx::bind_texture(GRID_TEXTURE, 0, false);
        let _ = spring::gfx::color(1.0, 1.0, 1.0, 1.0);
        let _ = spring::gfx::pop_attrib(0);
    }

    /// The display list of the grid, built on first use.
    fn list(&self) -> Option<u32> {
        if self.list.get() != 0 {
            return Some(self.list.get());
        }
        if self.setup_failed.get() {
            return None;
        }
        let (width, height) = map_size();
        if width <= 0.0 || height <= 0.0 {
            return None;
        }
        let Ok(list_id) = spring::gfx::create_list(emit_tiles) else {
            self.setup_failed.set(true);
            return None;
        };
        self.list.set(list_id);
        Some(list_id)
    }
}

fn emit_tiles() {
    let (width, height) = map_size();
    if width <= 0.0 || height <= 0.0 {
        return;
    }
    let range = (GRID_RANGE / GRID_RESOLUTION) as i32;
    let max_x = (width / GRID_RESOLUTION).ceil() as i32 + 1;
    let max_z = (height / GRID_RESOLUTION).ceil() as i32 + 1;
    let _ = spring::gfx::begin_end(gl::QUADS, || {
        for tile_x in -range..=max_x + range {
            for tile_z in -range..=max_z + range {
                if tile_x > 0 && tile_z > 0 && tile_x < max_x && tile_z < max_z {
                    continue;
                }
                let x0 = GRID_RESOLUTION * (tile_x - 1) as f32;
                let x1 = GRID_RESOLUTION * tile_x as f32;
                let z0 = GRID_RESOLUTION * (tile_z - 1) as f32;
                let z1 = GRID_RESOLUTION * tile_z as f32;
                vertex(0.0, 0.0, x0, z1, width, height);
                vertex(0.0, 1.0, x1, z1, width, height);
                vertex(1.0, 1.0, x1, z0, width, height);
                vertex(1.0, 0.0, x0, z0, width, height);
            }
        }
    });
}

fn vertex(u: f32, v: f32, x: f32, z: f32, width: f32, height: f32) {
    let _ = spring::gfx::tex_coord(u, v, 0.0, 0.0, 2);
    let _ = spring::gfx::vertex(x, ground(x, z, width, height), z, 1.0, 1);
}

fn ground(x: f32, z: f32, width: f32, height: f32) -> f32 {
    let mirrored_x = if x < 0.0 {
        -x
    } else if x > width {
        2.0 * width - x
    } else {
        x
    };
    let mirrored_z = if z < 0.0 {
        -z
    } else if z > height {
        2.0 * height - z
    } else {
        z
    };
    let distance = distance_from_edge(x, z, width, height);
    let fade = (distance / GRID_RANGE).min(1.0);
    spring::get_ground_height(mirrored_x, mirrored_z).unwrap_or(0.0) * (1.0 - fade * fade)
}

fn distance_from_edge(x: f32, z: f32, width: f32, height: f32) -> f32 {
    let dx = if x < 0.0 {
        -x
    } else if x > width {
        x - width
    } else {
        0.0
    };
    let dz = if z < 0.0 {
        -z
    } else if z > height {
        z - height
    } else {
        0.0
    };
    dx.max(dz)
}

fn map_size() -> (f32, f32) {
    spring::game::get_game_map_info(0)
        .map(|map| (map.map_size_x as f32, map.map_size_z as f32))
        .unwrap_or((0.0, 0.0))
}
