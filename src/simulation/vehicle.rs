use bevy::prelude::*;
use rand::Rng;
use crate::simulation::movement::StopAtTrafficLight;
use crate::city::City;  // Add this line to import City

#[derive(Component, Clone)]
pub struct Vehicle {
    pub position: Vec2,
    pub velocity: Vec2,
    pub direction: Vec2,
    pub color: Color,
}

impl Vehicle {
    pub fn new(position: Vec2) -> Self {
        let mut rng = rand::thread_rng();
        let direction = if rng.gen_bool(0.5) {
            Vec2::X
        } else {
            Vec2::Y
        };
        let color = Color::rgb(
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

pub fn spawn_vehicle(commands: &mut Commands, position: Vec2, city: &Res<City>) {
    let vehicle = Vehicle::new(position);
    commands.spawn((
        vehicle.clone(),
        SpriteBundle {
            sprite: Sprite {
                color: vehicle.color,
                custom_size: Some(Vec2::new(city.road_width * 0.8, city.road_width * 0.8)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 1.0),
            ..default()
        },
        StopAtTrafficLight(false),
    ));
}