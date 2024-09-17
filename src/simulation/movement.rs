use bevy::prelude::*;
use crate::simulation::vehicle::Vehicle;
use crate::city::City;
use rand::prelude::*;

pub fn move_vehicles(
    mut vehicles: Query<(&mut Vehicle, &mut Transform)>,
    city: Res<City>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (mut vehicle, mut transform) in vehicles.iter_mut() {
        let new_position = vehicle.position + vehicle.velocity * time.delta_seconds();
        
        if city.is_on_road(new_position) {
            vehicle.position = new_position;
        } else {
            // If not on road, move to nearest road center
            vehicle.position = city.nearest_road_center(vehicle.position);
            // Choose a new valid direction
            let available_directions = city.available_directions(vehicle.position);
            if !available_directions.is_empty() {
                vehicle.direction = *available_directions.choose(&mut rng).unwrap();
                vehicle.velocity = vehicle.direction * 50.0;
            } else {
                vehicle.velocity = Vec2::ZERO; // Stop if no valid direction
            }
        }
        
        // Update transform
        transform.translation.x = vehicle.position.x;
        transform.translation.y = vehicle.position.y;
    }
}