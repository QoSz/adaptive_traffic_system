use crate::city::{City, Direction};
use crate::simulation::traffic_light::{TrafficLight, TrafficLightState};
use crate::simulation::vehicle::{DespawnVehicle, IntersectionCooldown, Vehicle};
use bevy::prelude::*;
use rand::prelude::*;

/// Distance at which vehicles should start stopping for red lights
const TRAFFIC_LIGHT_STOP_DISTANCE: f32 = 25.0;

/// Probability of changing direction at an intersection (0.0 to 1.0)
const TURN_PROBABILITY: f32 = 0.4;

/// Check traffic lights and update vehicle stopped state
///
/// Kaizen principle: Single responsibility - only handles traffic light logic
pub fn check_traffic_lights(
    mut vehicles: Query<(&Transform, &mut Vehicle)>,
    traffic_lights: Query<(&TrafficLight, &Transform)>,
    city: Res<City>,
) {
    for (vehicle_transform, mut vehicle) in vehicles.iter_mut() {
        let vehicle_pos = vehicle_transform.translation.truncate();
        let mut should_stop = false;

        // Check against all traffic lights
        for (traffic_light, light_transform) in traffic_lights.iter() {
            let light_pos = light_transform.translation.truncate();
            let distance = vehicle_pos.distance(light_pos);

            // Only stop if:
            // 1. Within stopping distance
            // 2. Light is red
            // 3. Vehicle is approaching the light (not past it)
            if distance < TRAFFIC_LIGHT_STOP_DISTANCE {
                if matches!(traffic_light.state, TrafficLightState::Red) {
                    // Check if vehicle is approaching the light
                    let to_light = light_pos - vehicle_pos;
                    let vehicle_dir = vehicle.direction.to_vec2();

                    // Dot product > 0 means vehicle is heading toward the light
                    if to_light.dot(vehicle_dir) > 0.0 {
                        should_stop = true;
                        break;
                    }
                }
            }
        }

        // Update vehicle stopped state
        if should_stop {
            vehicle.stop();
        } else {
            vehicle.resume();
        }
    }
}

/// Move vehicles and handle direction changes at intersections
///
/// Monozukuri principle: Quality at every step
/// - Bounds checking before movement
/// - Proper direction changes
/// - Clean despawn for out-of-bounds vehicles
pub fn move_vehicles(
    mut commands: Commands,
    mut vehicles: Query<(
        Entity,
        &mut Vehicle,
        &mut Transform,
        &mut IntersectionCooldown,
    )>,
    city: Res<City>,
    time: Res<Time>,
) {
    let mut rng = rand::thread_rng();
    let delta = time.delta_seconds();

    for (entity, mut vehicle, mut transform, mut cooldown) in vehicles.iter_mut() {
        // Skip stopped vehicles
        if vehicle.stopped {
            continue;
        }

        let current_pos = transform.translation.truncate();
        let velocity = vehicle.velocity();
        let new_pos = current_pos + velocity * delta;

        // Check bounds - if out of bounds, mark for despawn
        if !city.is_within_bounds(new_pos) {
            // Clamp to bounds and mark for despawn
            let clamped_pos = city.clamp_to_bounds(new_pos);
            transform.translation.x = clamped_pos.x;
            transform.translation.y = clamped_pos.y;
            commands.entity(entity).insert(DespawnVehicle);
            continue;
        }

        // Check if new position is on a road
        if city.is_on_road(new_pos) {
            // Valid move - update position
            transform.translation.x = new_pos.x;
            transform.translation.y = new_pos.y;

            // Check for intersection behavior (with cooldown)
            if city.is_intersection(new_pos) && cooldown.timer.finished() {
                handle_intersection_behavior(
                    &mut vehicle,
                    &mut transform,
                    &city,
                    &mut cooldown,
                    &mut rng,
                );
            }
        } else {
            // Off-road - try to correct
            let nearest_road = city.nearest_road_position(current_pos);
            let available_dirs = city.available_directions(nearest_road);

            if !available_dirs.is_empty() {
                // Snap to road and pick a valid direction
                transform.translation.x = nearest_road.x;
                transform.translation.y = nearest_road.y;

                // Pick a direction that keeps us on the road
                let new_dir = pick_valid_direction(&vehicle.direction, &available_dirs, &mut rng);
                vehicle.set_direction(new_dir);
                update_vehicle_rotation(&mut transform, new_dir);
                cooldown.timer.reset();
            } else {
                // No valid directions - mark for despawn
                commands.entity(entity).insert(DespawnVehicle);
            }
        }
    }
}

/// Handle behavior when a vehicle reaches an intersection
fn handle_intersection_behavior(
    vehicle: &mut Vehicle,
    transform: &mut Transform,
    city: &City,
    cooldown: &mut IntersectionCooldown,
    rng: &mut ThreadRng,
) {
    let position = transform.translation.truncate();
    let available_dirs = city.available_directions(position);

    if available_dirs.is_empty() {
        return;
    }

    // Randomly decide whether to turn
    if rng.gen_bool(TURN_PROBABILITY as f64) {
        // Pick a new direction (prefer not going backwards)
        let new_dir = pick_valid_direction(&vehicle.direction, &available_dirs, rng);
        vehicle.set_direction(new_dir);
        update_vehicle_rotation(transform, new_dir);
    }

    // Reset cooldown to prevent rapid direction changes
    cooldown.timer.reset();
}

/// Pick a valid direction, preferring not to reverse
fn pick_valid_direction(
    current_dir: &Direction,
    available_dirs: &[Direction],
    rng: &mut ThreadRng,
) -> Direction {
    // Filter out the opposite direction (no U-turns)
    let opposite = current_dir.opposite();
    let forward_dirs: Vec<_> = available_dirs
        .iter()
        .filter(|&&d| d != opposite)
        .copied()
        .collect();

    // If we can only go backwards, allow it
    if forward_dirs.is_empty() {
        *available_dirs.choose(rng).unwrap()
    } else {
        *forward_dirs.choose(rng).unwrap()
    }
}

/// Update vehicle sprite rotation based on direction
fn update_vehicle_rotation(transform: &mut Transform, direction: Direction) {
    let rotation = match direction {
        Direction::North => 0.0,
        Direction::South => std::f32::consts::PI,
        Direction::East => -std::f32::consts::FRAC_PI_2,
        Direction::West => std::f32::consts::FRAC_PI_2,
    };
    transform.rotation = Quat::from_rotation_z(rotation);
}

/// System to handle edge case: vehicles that somehow got stuck
/// Kaizen principle: Continuous improvement - handle edge cases gracefully
pub fn cleanup_stuck_vehicles(
    mut commands: Commands,
    vehicles: Query<(Entity, &Transform, &Vehicle)>,
    city: Res<City>,
) {
    for (entity, transform, vehicle) in vehicles.iter() {
        let pos = transform.translation.truncate();

        // Check if vehicle is completely stuck (not on road and no valid directions)
        if !city.is_on_road(pos) {
            let nearest = city.nearest_road_position(pos);
            if city.available_directions(nearest).is_empty() {
                commands.entity(entity).insert(DespawnVehicle);
            }
        }
    }
}
