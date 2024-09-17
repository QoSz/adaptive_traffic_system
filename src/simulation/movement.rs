use bevy::prelude::*;
use rand::prelude::*;
use crate::simulation::vehicle::Vehicle;
use crate::simulation::traffic_light::{TrafficLight, TrafficLightState};
use crate::city::City;

#[derive(Component)]
pub struct StopAtTrafficLight(pub bool);

pub fn check_traffic_lights(
    mut commands: Commands,
    vehicles: Query<(Entity, &Transform), With<Vehicle>>,
    traffic_lights: Query<(&TrafficLight, &Transform)>,
    city: Res<City>,
) {
    for (entity, vehicle_transform) in vehicles.iter() {
        let mut should_stop = false;
        for (traffic_light, light_transform) in traffic_lights.iter() {
            let distance = light_transform.translation.truncate().distance(vehicle_transform.translation.truncate());
            if distance < city.road_width * 0.5 && matches!(traffic_light.state, TrafficLightState::Red) {
                should_stop = true;
                break;
            }
        }
        commands.entity(entity).insert(StopAtTrafficLight(should_stop));
    }
}

pub fn move_vehicles(
    mut vehicles: Query<(&mut Vehicle, &mut Transform, &StopAtTrafficLight)>,
    city: Res<City>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();

    for (mut vehicle, mut transform, stop_at_light) in vehicles.iter_mut() {
        if stop_at_light.0 {
            vehicle.velocity = Vec2::ZERO;
            continue;
        }

        let new_position = vehicle.position + vehicle.velocity * time.delta_seconds();
        
        if city.is_valid_position(new_position) && city.is_on_road(new_position) {
            vehicle.position = new_position;

            if city.is_intersection(vehicle.position) {
                let available_directions = city.available_directions(vehicle.position);
                if !available_directions.is_empty() && rng.gen_bool(0.3) {
                    vehicle.direction = *available_directions.choose(&mut rng).unwrap();
                    vehicle.velocity = vehicle.direction * 50.0;
                }
            }
        } else {
            vehicle.position = city.nearest_road_center(vehicle.position);
            let available_directions = city.available_directions(vehicle.position);
            if !available_directions.is_empty() {
                vehicle.direction = *available_directions.choose(&mut rng).unwrap();
                vehicle.velocity = vehicle.direction * 50.0;
            } else {
                vehicle.velocity = Vec2::ZERO;
            }
        }
        
        transform.translation.x = vehicle.position.x;
        transform.translation.y = vehicle.position.y;
    }
}