use crate::engine::body::Body;

pub fn update(body: &mut Body, dt: f32) {
    body.velocity += body.acceleration * dt;

    body.position += body.velocity * dt;
}
