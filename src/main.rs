//! Adaptive Traffic Simulation System
//!
//! A traffic simulation built with Bevy following Kaizen and Monozukuri principles:
//! - Kaizen: Continuous improvement through iterative refinement
//! - Monozukuri: Craftsmanship with attention to quality at every step
//!
//! ## Architecture
//! - City module: Road network and spatial queries
//! - Simulation module: Vehicle behavior, traffic lights, movement physics

use bevy::prelude::*;

mod city;
mod simulation;

use city::renderer::{render_city, render_city_background};
use city::City;
use simulation::{
    adjust_spawn_rate, check_traffic_lights, cleanup_stuck_vehicles, despawn_vehicles,
    move_vehicles, spawn_traffic_lights, spawn_vehicles, update_intersection_cooldowns,
    update_traffic_lights, SpawnTimer, Vehicle,
};

/// Configuration constants
const CITY_SIZE: (u32, u32) = (20, 20);
const ROAD_WIDTH: f32 = 20.0;
const GRID_SPACING: u32 = 5; // Roads every 5 grid cells

/// Debug information resource
#[derive(Resource)]
struct DebugInfo {
    last_vehicle_count: usize,
    show_debug: bool,
}

impl Default for DebugInfo {
    fn default() -> Self {
        DebugInfo {
            last_vehicle_count: 0,
            show_debug: true,
        }
    }
}

fn main() {
    println!("=================================");
    println!("  Adaptive Traffic Simulation");
    println!("=================================");
    println!("Principles: Kaizen & Monozukuri");
    println!();

    App::new()
        // Core plugins
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Adaptive Traffic Simulation".to_string(),
                resolution: (800.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        // Resources
        .insert_resource(City::new(CITY_SIZE, ROAD_WIDTH, GRID_SPACING))
        .insert_resource(SpawnTimer::default())
        .insert_resource(DebugInfo::default())
        // Startup systems (run once at beginning)
        .add_systems(
            Startup,
            (
                setup_camera,
                render_city_background,
                render_city,
                spawn_traffic_lights,
            )
                .chain(),
        )
        // Update systems (run every frame)
        .add_systems(
            Update,
            (
                // Traffic generation
                spawn_vehicles,
                adjust_spawn_rate,
                // Traffic light updates
                update_traffic_lights,
                // Vehicle updates (order matters!)
                update_intersection_cooldowns,
                check_traffic_lights,
                move_vehicles,
                // Cleanup
                despawn_vehicles,
                cleanup_stuck_vehicles,
                // Debug
                debug_info,
            )
                .chain(),
        )
        .run();
}

/// Setup the 2D camera centered on the city
fn setup_camera(mut commands: Commands, city: Res<City>) {
    let center = city.world_center();

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(center.x, center.y, 999.9),
        ..default()
    });

    println!(
        "Camera positioned at city center: ({:.1}, {:.1})",
        center.x, center.y
    );
    println!(
        "City size: {}x{} grid cells ({:.0}x{:.0} world units)",
        city.size.0,
        city.size.1,
        city.world_size().x,
        city.world_size().y
    );
    println!("Road grid spacing: {} cells", city.grid_spacing);
    println!("Total roads: {}", city.roads.len());
    println!("Intersections: {}", city.get_intersections().len());
    println!();
}

/// Debug system to display vehicle count and simulation status
fn debug_info(vehicles: Query<&Vehicle>, mut debug_info: ResMut<DebugInfo>) {
    if !debug_info.show_debug {
        return;
    }

    let current_count = vehicles.iter().count();

    // Count stopped vs moving vehicles
    let stopped_count = vehicles.iter().filter(|v| v.stopped).count();
    let moving_count = current_count - stopped_count;

    if current_count != debug_info.last_vehicle_count {
        println!(
            "Vehicles: {} total ({} moving, {} stopped)",
            current_count, moving_count, stopped_count
        );
        debug_info.last_vehicle_count = current_count;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_city_creation() {
        let city = City::new(CITY_SIZE, ROAD_WIDTH, GRID_SPACING);
        assert!(!city.roads.is_empty());
        assert!(city.size.0 == CITY_SIZE.0);
        assert!(city.size.1 == CITY_SIZE.1);
    }

    #[test]
    fn test_intersections() {
        let city = City::new(CITY_SIZE, ROAD_WIDTH, GRID_SPACING);
        let intersections = city.get_intersections();
        assert!(!intersections.is_empty());

        // All returned positions should be valid intersections
        for (x, y) in &intersections {
            assert!(city.is_intersection_at_grid(*x, *y));
        }
    }
}
