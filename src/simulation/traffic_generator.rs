use super::vehicle::{spawn_vehicle, Vehicle};
use crate::city::City;
use bevy::prelude::*;
use rand::prelude::*;

/// Maximum number of vehicles in the simulation
const MAX_VEHICLES: usize = 50;

/// Spawn rate: vehicles per second
const SPAWN_RATE: f32 = 2.0;

/// Resource to track spawn timing
#[derive(Resource)]
pub struct SpawnTimer {
    pub timer: Timer,
}

impl Default for SpawnTimer {
    fn default() -> Self {
        SpawnTimer {
            timer: Timer::from_seconds(1.0 / SPAWN_RATE, TimerMode::Repeating),
        }
    }
}

/// Spawn vehicles at valid road positions
///
/// Kaizen principle: Eliminate waste (muda)
/// - Only spawn at positions with valid directions
/// - Rate-limited spawning for smooth simulation
/// - Prefer edge spawns for natural traffic flow
pub fn spawn_vehicles(
    mut commands: Commands,
    city: Res<City>,
    query: Query<&Vehicle>,
    mut spawn_timer: ResMut<SpawnTimer>,
    time: Res<Time>,
) {
    // Tick the spawn timer
    spawn_timer.timer.tick(time.delta());

    // Check if we should spawn
    if !spawn_timer.timer.just_finished() {
        return;
    }

    // Check vehicle count
    let current_count = query.iter().count();
    if current_count >= MAX_VEHICLES {
        return;
    }

    let mut rng = rand::thread_rng();

    // Try to find a valid spawn position (prefer edges for natural traffic flow)
    if let Some(spawn_pos) = find_spawn_position(&city, &mut rng) {
        spawn_vehicle(&mut commands, spawn_pos, &city);
    }
}

/// Find a valid spawn position for a new vehicle
///
/// Monozukuri principle: Quality from the source
/// - Validates position has available directions
/// - Prefers edge positions for realistic traffic entry
fn find_spawn_position(city: &City, rng: &mut ThreadRng) -> Option<Vec2> {
    // Collect edge road positions (where vehicles would naturally enter)
    let mut edge_positions: Vec<Vec2> = Vec::new();
    let mut interior_positions: Vec<Vec2> = Vec::new();

    for road in &city.roads {
        let (x, y) = road.position;
        let world_pos = city.grid_to_world(x, y);

        // Check if this position has available directions
        let directions = city.available_directions(world_pos);
        if directions.is_empty() {
            continue;
        }

        // Classify as edge or interior
        let is_edge = x == 0 || x == city.size.0 || y == 0 || y == city.size.1;

        if is_edge {
            edge_positions.push(world_pos);
        } else {
            interior_positions.push(world_pos);
        }
    }

    // Prefer edge positions (80% chance if available)
    if !edge_positions.is_empty() && rng.gen_bool(0.8) {
        edge_positions.choose(rng).copied()
    } else if !interior_positions.is_empty() {
        interior_positions.choose(rng).copied()
    } else {
        edge_positions.choose(rng).copied()
    }
}

/// System to gradually increase traffic over time
/// Kaizen principle: Continuous improvement
pub fn adjust_spawn_rate(
    mut spawn_timer: ResMut<SpawnTimer>,
    vehicles: Query<&Vehicle>,
    time: Res<Time>,
) {
    let vehicle_count = vehicles.iter().count();

    // Adjust spawn rate based on current traffic
    // More vehicles = slower spawn rate
    let rate_factor = if vehicle_count < 20 {
        1.5 // Faster spawning when few vehicles
    } else if vehicle_count < 40 {
        1.0 // Normal rate
    } else {
        0.5 // Slower when approaching max
    };

    let target_duration = 1.0 / (SPAWN_RATE * rate_factor);

    // Smoothly adjust the timer duration
    let current_duration = spawn_timer.timer.duration().as_secs_f32();
    if (current_duration - target_duration).abs() > 0.1 {
        spawn_timer
            .timer
            .set_duration(std::time::Duration::from_secs_f32(target_duration));
    }
}
