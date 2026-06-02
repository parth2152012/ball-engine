use crate::engine::body::Body;

const SLEEP_THRESHOLD: f32 = 2.0;
const SLEEP_TIME: f32 = 1.0;

pub fn update(body: &mut Body, dt: f32) {
    if body.is_sleeping {
        return;
    }

    body.velocity += body.acceleration * dt;

    body.position += body.velocity * dt;

    // Sleep logic

    if body.velocity.length() < SLEEP_THRESHOLD {
        body.sleep_timer += dt;

        if body.sleep_timer > SLEEP_TIME {
            body.is_sleeping = true;

            body.velocity = macroquad::prelude::Vec2::ZERO;
        }
    } else {
        body.sleep_timer = 0.0;
    }
}
