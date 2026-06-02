mod engine;

use engine::body::Body;
use engine::collision;
use engine::physics;

use macroquad::prelude::*;

const GRAVITY: f32 = 980.0;

#[macroquad::main("Parth Engine")]
async fn main() {
    let mut bodies = vec![
        Body::new(vec2(200.0, 100.0), 20.0, 1.0, 0.95, 0.8),
        Body::new(vec2(500.0, 100.0), 30.0, 2.0, 0.90, 0.7),
    ];

    bodies[0].velocity.x = 250.0;
    bodies[1].velocity.x = -150.0;

    for body in &mut bodies {
        body.acceleration.y = GRAVITY;
    }

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);

        // Spawn new ball
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();

            let mut body = Body::new(vec2(mx, my), 20.0, 1.0, 0.95, 0.8);

            body.acceleration.y = GRAVITY;

            bodies.push(body);
        }

        // Physics update
        for body in &mut bodies {
            physics::update(body, dt);

            // Floor collision
            let ground = screen_height() - body.radius;

            if body.position.y > ground {
                body.position.y = ground;

                body.velocity.y *= -body.restitution;

                body.velocity.x *= body.friction;

                if body.velocity.y.abs() < 5.0 {
                    body.velocity.y = 0.0;
                }
            }

            // Ceiling collision
            if body.position.y - body.radius <= 0.0 {
                body.position.y = body.radius;

                body.velocity.y *= -body.restitution;
            }

            // Left wall
            if body.position.x - body.radius <= 0.0 {
                body.position.x = body.radius;

                body.velocity.x *= -body.friction;
            }

            // Right wall
            if body.position.x + body.radius >= screen_width() {
                body.position.x = screen_width() - body.radius;

                body.velocity.x *= -body.friction;
            }
        }

        collision::resolve_collisions(&mut bodies);

        for body in &bodies {
            draw_circle(body.position.x, body.position.y, body.radius, WHITE);
        }

        draw_text(
            &format!("Bodies: {}", bodies.len()),
            20.0,
            30.0,
            30.0,
            GREEN,
        );

        next_frame().await;
    }
}
