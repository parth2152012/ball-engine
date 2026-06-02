# Physics Engine

A simple 2D physics engine built in Rust using Macroquad.

This project was created as a learning project to understand:

- Rust fundamentals
- Physics simulation
- Collision detection
- Collision resolution
- Game engine architecture

## Features

- Gravity
- Ball-to-ball collisions
- Wall collisions
- Floor collisions
- Restitution (bounciness)
- Friction
- Sleeping bodies
- Random ball spawning
- Random colors
- Random masses
- Random radii

## Controls

### Left Mouse Button

Spawn a new ball at the cursor position.

## Physics

The engine currently supports:

- Circle rigid bodies
- Impulse-based collision response
- Position correction
- Gravity simulation
- Sleeping bodies for performance

## Project Structure

```text
src/
├── main.rs
└── engine/
    ├── body.rs
    ├── collision.rs
    ├── physics.rs
    ├── world.rs
    └── mod.rs
