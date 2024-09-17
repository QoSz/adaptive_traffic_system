use bevy::prelude::*;
use rand::prelude::*;
use crate::city::City;
use super::vehicle::Vehicle;

const MAX_VEHICLES: usize = 100;

pub fn spawn_vehicles(
    mut commands: Commands,
    city: Res<City>,
    vehicle_query: Query<&Vehicle>,
) {
    if vehicle_query.iter().count() >= MAX_VEHICLES {
        return;
    }

    let mut rng: rand::rngs::ThreadRng = rand::thread_rng();

    let road_index: usize = rng.gen_range(0..city.roads.len());
    let road = &city.roads[road_index];

    let position: Vec2 = Vec2::new(
        road.position.0 as f32 * city.road_width,
        road.position.1 as f32 * city.road_width,
    );

    let vehicle: Vehicle = Vehicle::new(position);
    let vehicle_color: Color = vehicle.color;
    commands.spawn((
        vehicle,
        SpriteBundle {
            sprite: Sprite {
                color: vehicle_color,
                custom_size: Some(Vec2::new(city.road_width * 0.8, city.road_width * 0.8)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 1.0),
            ..default()
        },
    ));
}