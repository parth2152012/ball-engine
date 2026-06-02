use crate::engine::body::Body;

use macroquad::prelude::*;

const BOUNCINESS: f32 = 0.9;

pub fn resolve_collisions(bodies: &mut Vec<Body>) {
    for i in 0..bodies.len() {
        for j in (i + 1)..bodies.len() {
            let (left, right) = bodies.split_at_mut(j);

            let body1 = &mut left[i];
            let body2 = &mut right[0];

            let delta = body2.position - body1.position;

            let distance = delta.length();

            let min_distance = body1.radius + body2.radius;

            if distance < min_distance {
                let normal = if distance == 0.0 {
                    vec2(1.0, 0.0)
                } else {
                    delta / distance
                };

                // --------------------
                // Position correction
                // --------------------

                let overlap = min_distance - distance;

                body1.position -= normal * overlap * 0.5;

                body2.position += normal * overlap * 0.5;

                // --------------------
                // Velocity correction
                // --------------------

                let relative_velocity = body2.velocity - body1.velocity;

                let velocity_along_normal = relative_velocity.dot(normal);

                if velocity_along_normal > 0.0 {
                    continue;
                }

                let impulse_scalar = -(1.0 + BOUNCINESS) * velocity_along_normal
                    / ((1.0 / body1.mass) + (1.0 / body2.mass));

                let impulse = normal * impulse_scalar;

                body1.velocity -= impulse / body1.mass;

                body2.velocity += impulse / body2.mass;
            }
        }
    }
}
