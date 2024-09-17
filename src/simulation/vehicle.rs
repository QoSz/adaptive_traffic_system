use bevy::prelude::*;
use rand::Rng;

#[derive(Component, Clone)]
pub struct Vehicle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: Vec2,
    pub color: Color,
}

impl Vehicle {
    pub fn new(position: Vec2) -> Self {
        let mut rng: rand::rngs::ThreadRng = rand::thread_rng();
        let direction: Vec2 = if rng.gen_bool(0.5) {
            Vec2::X
        } else {
            Vec2::Y
        };
        let color: Color = Color::rgb(
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
            rng.gen_range(0.0..1.0),
        );
        Vehicle {
            position,
            velocity: direction * 50.0,
            direction,
            color,
        }
    }
}