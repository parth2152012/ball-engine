use crate::engine::body::Body;

pub struct World {
    pub bodies: Vec<Body>,

    pub gravity: f32,
}

impl World {
    pub fn new(gravity: f32) -> Self {
        Self {
            bodies: Vec::new(),
            gravity,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }
}
