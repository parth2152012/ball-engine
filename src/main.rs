mod engine;

use engine::body::Body;
use engine::collision;
use engine::physics;
use engine::world::World;

use macroquad::prelude::*;
use macroquad::rand::gen_range;

const GRAVITY: f32 = 500.0;

fn random_color() -> Color {
    Color::new(
        gen_range(0.0, 1.0),
        gen_range(0.0, 1.0),
        gen_range(0.0, 1.0),
        1.0,
    )
}

fn random_radius() -> f32 {
    gen_range(20.0, 40.0)
}

fn random_mass() -> f32 {
    let rad = random_radius();
    let mass = rad / 10.0;
    return mass;
}

#[macroquad::main("ball Engine")]
async fn main() {
    let mut world = World::new(GRAVITY);

    world.add_body(Body::new(vec2(200.0, 100.0), 20.0, 1.0, 0.95, 0.8, RED));

    world.add_body(Body::new(vec2(500.0, 100.0), 30.0, 2.0, 0.90, 0.7, BLUE));

    // Give initial velocities
    world.bodies[0].velocity.x = 250.0;
    world.bodies[1].velocity.x = -150.0;

    // Apply gravity
    for body in &mut world.bodies {
        body.acceleration.y = world.gravity;
    }

    loop {
        let dt = get_frame_time();

        clear_background(BLACK);

        // Spawn ball with left click
        if is_mouse_button_pressed(MouseButton::Left) {
            let (mx, my) = mouse_position();

            let mut body = Body::new(
                vec2(mx, my),
                random_radius(),
                random_mass(),
                0.95,
                0.8,
                random_color(),
            );

            body.acceleration.y = world.gravity;
            body.is_sleeping = false;
            body.sleep_timer = 0.0;
            body.velocity.x = gen_range(-200.0, 200.0);

            world.add_body(body);
        }

        // Physics update
        for body in &mut world.bodies {
            physics::update(body, dt);

            // Floor collision
            let ground = screen_height() - body.radius;

            if body.position.y > ground {
                body.position.y = ground;

                body.velocity.y *= -body.restitution;
                body.velocity.x *= body.friction;

                if body.velocity.y.abs() < 5.0 {
                    body.velocity.y = 0.0;

                    if body.velocity.length() < 2.0 {
                        body.is_sleeping = true;
                    }
                }

                if body.velocity.x.abs() < 5.0 {
                    body.velocity.x = 0.0;
                }
            }

            // Ceiling collision
            if body.position.y - body.radius <= 0.0 {
                body.position.y = body.radius;
                body.velocity.y *= -body.restitution;
            }

            // Left wall collision
            if body.position.x - body.radius <= 0.0 {
                body.position.x = body.radius;
                body.velocity.x *= -body.friction;
            }

            // Right wall collision
            if body.position.x + body.radius >= screen_width() {
                body.position.x = screen_width() - body.radius;
                body.velocity.x *= -body.friction;
            }
        }

        // Ball-ball collisions
        for _ in 0..150 {
            collision::resolve_collisions(&mut world.bodies);
        }

        // Render all bodies
        for body in &world.bodies {
            draw_circle(body.position.x, body.position.y, body.radius, body.color);
        }

        // Debug info
        draw_text(
            &format!("Bodies: {}", world.bodies.len()),
            20.0,
            30.0,
            30.0,
            GREEN,
        );

        draw_text("Left Click = Spawn Ball", 20.0, 60.0, 24.0, WHITE);
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 90.0, 24.0, YELLOW);

        next_frame().await;
    }
}
