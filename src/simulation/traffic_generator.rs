use bevy::prelude::*;
use rand::prelude::*;
use crate::city::City;
use super::vehicle::spawn_vehicle;

const MAX_VEHICLES: usize = 100;

pub fn spawn_vehicles(
    mut commands: Commands,
    city: Res<City>,
    query: Query<&super::vehicle::Vehicle>,
) {
    if query.iter().count() >= MAX_VEHICLES {
        return;
    }

    let mut rng = rand::thread_rng();

    let road_index = rng.gen_range(0..city.roads.len());
    let road = &city.roads[road_index];

    let position = Vec2::new(
        road.position.0 as f32 * city.road_width,
        road.position.1 as f32 * city.road_width,
    );

    spawn_vehicle(&mut commands, position, &city);
}