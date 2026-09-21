pub const BAR_HEIGHT: f32 = 2.3;
pub const BAR_WIDTH: f32 = 11.5;
pub const BAR_ALPHA: f32 = 0.85;
pub const BAR_OUTLINE_ALPHA: f32 = 0.8;
pub const BAR_INNER_ALPHA: f32 = 0.5;
pub const BAR_VALUE_ALPHA: f32 = 0.9;

const CHOPPED_CORNER_SIZE: f32 = 0.44;
const OUTLINE_SIZE: f32 = 0.7;
const OUTLINE_BOTTOM: [f32; 4] = [0.25, 0.25, 0.25, BAR_OUTLINE_ALPHA];
const OUTLINE_TOP: [f32; 4] = [0.10, 0.10, 0.10, BAR_OUTLINE_ALPHA];
const INNER_BOTTOM: [f32; 4] = [0.06, 0.06, 0.06, BAR_INNER_ALPHA];
const INNER_TOP: [f32; 4] = [0.33, 0.33, 0.33, BAR_INNER_ALPHA * 0.5];

pub fn emit_bar() {
    let cs = CHOPPED_CORNER_SIZE;
    let outline = OUTLINE_SIZE;
    let span = BAR_WIDTH * 2.0 - cs * 2.0;

    color(OUTLINE_BOTTOM);
    vertex(BAR_WIDTH - cs, -outline, 0.0, 1.0);
    vertex(BAR_WIDTH - cs, -outline, span, 1.0);
    color(OUTLINE_TOP);
    vertex(BAR_WIDTH - cs, BAR_HEIGHT + outline, span, 1.0);
    vertex(BAR_WIDTH - cs, BAR_HEIGHT + outline, 0.0, 1.0);

    color(INNER_BOTTOM);
    vertex(BAR_WIDTH - cs - cs, 0.0, 0.0, 2.0);
    vertex(BAR_WIDTH - cs, 0.0, span, 2.0);
    color(INNER_TOP);
    vertex(BAR_WIDTH - cs, BAR_HEIGHT, span, 2.0);
    vertex(BAR_WIDTH - cs - cs, BAR_HEIGHT, 0.0, 2.0);

    color(INNER_BOTTOM);
    vertex(BAR_WIDTH - cs - cs + cs, cs, 0.0, 2.0);
    vertex(BAR_WIDTH - cs - cs, 0.0, 0.0, 2.0);
    color(INNER_TOP);
    vertex(BAR_WIDTH - cs - cs, BAR_HEIGHT, 0.0, 2.0);
    vertex(BAR_WIDTH - cs - cs + cs, BAR_HEIGHT - cs, 0.0, 2.0);

    color(OUTLINE_BOTTOM);
    vertex(BAR_WIDTH - cs + outline, 0.0, 0.0, 1.0);
    vertex(BAR_WIDTH - cs, -outline, 0.0, 1.0);
    color(OUTLINE_TOP);
    vertex(BAR_WIDTH - cs, BAR_HEIGHT + outline, 0.0, 1.0);
    vertex(
        BAR_WIDTH - cs + outline,
        BAR_HEIGHT + outline - outline,
        0.0,
        1.0,
    );

    color(OUTLINE_BOTTOM);
    vertex(-BAR_WIDTH - outline, 0.0, 0.0, 1.0);
    vertex(-BAR_WIDTH, -outline, 0.0, 1.0);
    color(OUTLINE_TOP);
    vertex(-BAR_WIDTH, BAR_HEIGHT + outline, 0.0, 1.0);
    vertex(
        -BAR_WIDTH - outline,
        BAR_HEIGHT + outline - outline,
        0.0,
        1.0,
    );

    let blend = outline / (BAR_HEIGHT + outline + outline);
    let top_shade = OUTLINE_TOP[0] + (OUTLINE_BOTTOM[0] - OUTLINE_TOP[0]) * blend;
    color([top_shade, top_shade, top_shade, OUTLINE_TOP[3]]);
    vertex(-BAR_WIDTH, BAR_HEIGHT, 0.0, 1.0);
    vertex(BAR_WIDTH - cs, BAR_HEIGHT, span, 1.0);
    color(OUTLINE_TOP);
    vertex(BAR_WIDTH - cs, BAR_HEIGHT + outline, span, 1.0);
    vertex(-BAR_WIDTH, BAR_HEIGHT + outline, 0.0, 1.0);

    let bottom_shade = OUTLINE_BOTTOM[0] - (OUTLINE_BOTTOM[0] - OUTLINE_TOP[0]) * blend;
    color([bottom_shade, bottom_shade, bottom_shade, OUTLINE_TOP[3]]);
    vertex(-BAR_WIDTH, 0.0, 0.0, 1.0);
    vertex(BAR_WIDTH - cs, 0.0, span, 1.0);
    color(OUTLINE_BOTTOM);
    vertex(BAR_WIDTH - cs, -outline, span, 1.0);
    vertex(-BAR_WIDTH, -outline, 0.0, 1.0);

    vertex(-BAR_WIDTH + cs, 0.0, 0.0, 0.0);
    vertex(-BAR_WIDTH, 0.0, span, 0.0);
    vertex(-BAR_WIDTH, BAR_HEIGHT, span, 0.0);
    vertex(-BAR_WIDTH + cs, BAR_HEIGHT, 0.0, 0.0);

    vertex(-BAR_WIDTH, cs, 0.0, 0.0);
    vertex(-BAR_WIDTH + cs, 0.0, 0.0, 0.0);
    vertex(-BAR_WIDTH + cs, BAR_HEIGHT, 0.0, 0.0);
    vertex(-BAR_WIDTH, BAR_HEIGHT - cs, 0.0, 0.0);

    vertex(-BAR_WIDTH + cs, cs, span, 0.0);
    vertex(-BAR_WIDTH, 0.0, span, 0.0);
    vertex(-BAR_WIDTH, BAR_HEIGHT, span, 0.0);
    vertex(-BAR_WIDTH + cs, BAR_HEIGHT - cs, span, 0.0);
}

fn color(value: [f32; 4]) {
    let _ = spring::gfx::color(value[0], value[1], value[2], value[3]);
}

fn vertex(x: f32, y: f32, z: f32, w: f32) {
    let _ = spring::gfx::vertex(x, y, z, w, 1);
}
