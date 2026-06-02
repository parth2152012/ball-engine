use macroquad::{color, prelude::*};

#[derive(Debug, Clone)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,

    pub radius: f32,
    pub mass: f32,

    pub restitution: f32,
    pub friction: f32,

    pub color: color::Color,
    pub is_sleeping: bool,
    pub sleep_timer: f32,
}

impl Body {
    pub fn new(
        position: Vec2,
        radius: f32,
        mass: f32,
        restitution: f32,
        friction: f32,
        color: color::Color,
    ) -> Self {
        Self {
            position,

            velocity: Vec2::ZERO,
            acceleration: Vec2::ZERO,

            radius,
            mass,

            restitution,
            friction,
            color,

            is_sleeping: false,
            sleep_timer: 0.0,
        }
    }
}
