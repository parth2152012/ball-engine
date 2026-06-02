use crate::engine::body::Body;
use macroquad::prelude::*;

const BOUNCINESS: f32 = 0.7;

// Position correction constants
const POSITION_CORRECTION_PERCENT: f32 = 0.8;
const POSITION_CORRECTION_SLOP: f32 = 0.01;

pub fn resolve_collisions(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let (left, right) = bodies.split_at_mut(j);

            let body1 = &mut left[i];
            let body2 = &mut right[0];

            let delta = body2.position - body1.position;

            let distance = delta.length();

            let min_distance = body1.radius + body2.radius;

            if distance >= min_distance {
                continue;
            }

            // -------------------------
            // Collision normal
            // -------------------------

            let normal = if distance <= f32::EPSILON {
                vec2(1.0, 0.0)
            } else {
                delta / distance
            };

            body1.is_sleeping = false;
            body2.is_sleeping = false;

            body1.sleep_timer = 0.0;
            body2.sleep_timer = 0.0;

            // -------------------------
            // Position correction
            // -------------------------

            let overlap = min_distance - distance;

            let correction = normal
                * ((overlap - POSITION_CORRECTION_SLOP).max(0.0))
                * POSITION_CORRECTION_PERCENT
                * 0.5;

            body1.position -= correction;
            body2.position += correction;

            // -------------------------
            // Relative velocity
            // -------------------------

            let relative_velocity = body2.velocity - body1.velocity;

            let velocity_along_normal = relative_velocity.dot(normal);

            // Already separating
            if velocity_along_normal > 0.0 {
                continue;
            }

            // Ignore tiny collision velocities
            // Prevents resting jitter
            if velocity_along_normal.abs() < 1.0 {
                continue;
            }

            // -------------------------
            // Impulse calculation
            // -------------------------

            let impulse_scalar = -(1.0 + BOUNCINESS) * velocity_along_normal
                / ((1.0 / body1.mass) + (1.0 / body2.mass));

            let impulse = normal * impulse_scalar;

            body1.velocity -= impulse / body1.mass;

            body2.velocity += impulse / body2.mass;
        }
    }
}
