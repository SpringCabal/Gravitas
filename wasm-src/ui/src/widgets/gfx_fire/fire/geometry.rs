use super::particles::Sample;
use alloc::vec::Vec;

pub(super) fn quad(
    vertices: &mut Vec<f32>,
    particle: &Sample,
    up: spring::Float3,
    right: spring::Float3,
) {
    let (across, along) = axes(particle, up, right);
    quad_axes(vertices, particle, across, along);
}

pub(super) fn axes(
    particle: &Sample,
    up: spring::Float3,
    right: spring::Float3,
) -> (spring::Float3, spring::Float3) {
    let radians = -particle.rotation * core::f32::consts::PI / 180.0;
    let (sine, cosine) = (radians.sin(), radians.cos());
    let half = particle.size * 0.5;
    let across = axis(right, cosine, up, sine, half);
    let along = axis(right, -sine, up, cosine, half);
    (across, along)
}

pub(super) fn quad_axes(
    vertices: &mut Vec<f32>,
    particle: &Sample,
    across: spring::Float3,
    along: spring::Float3,
) {
    for (u, v, a, b) in [
        (0.0, 0.0, -1.0, -1.0),
        (1.0, 0.0, 1.0, -1.0),
        (1.0, 1.0, 1.0, 1.0),
        (0.0, 0.0, -1.0, -1.0),
        (1.0, 1.0, 1.0, 1.0),
        (0.0, 1.0, -1.0, 1.0),
    ] {
        corner(vertices, particle, u, v, a, b, across, along);
    }
}

fn axis(
    right: spring::Float3,
    right_scale: f32,
    up: spring::Float3,
    up_scale: f32,
    half: f32,
) -> spring::Float3 {
    spring::Float3 {
        x: (right.x * right_scale + up.x * up_scale) * half,
        y: (right.y * right_scale + up.y * up_scale) * half,
        z: (right.z * right_scale + up.z * up_scale) * half,
    }
}

fn corner(
    vertices: &mut Vec<f32>,
    particle: &Sample,
    u: f32,
    v: f32,
    across_sign: f32,
    along_sign: f32,
    across: spring::Float3,
    along: spring::Float3,
) {
    vertices.extend_from_slice(&[
        particle.x + across.x * across_sign + along.x * along_sign,
        particle.y + across.y * across_sign + along.y * along_sign,
        particle.z + across.z * across_sign + along.z * along_sign,
    ]);
    vertices.extend_from_slice(&particle.color);
    vertices.extend_from_slice(&[u, v]);
}
