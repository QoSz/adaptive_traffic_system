use bevy::prelude::*;
mod city;
mod simulation;

use city::City;
use city::renderer::render_city;
use simulation::traffic_generator::spawn_vehicles;
use simulation::movement::{move_vehicles, check_traffic_lights};
use simulation::traffic_light::{spawn_traffic_lights, update_traffic_lights};

#[derive(Resource)]
struct DebugInfo {
    last_vehicle_count: usize,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(City::new((20, 20), 20.0))
        .insert_resource(DebugInfo { last_vehicle_count: 0 })
        .add_systems(Startup, (setup, render_city, spawn_traffic_lights))
        .add_systems(Update, (
            spawn_vehicles,
            update_traffic_lights,
            check_traffic_lights,
            move_vehicles,
            debug_info
        ).chain())
        .run();
}

fn setup(mut commands: Commands, city: Res<City>) {
    let city_width = city.size.0 as f32 * city.road_width;
    let city_height = city.size.1 as f32 * city.road_width;
    let center_x = city_width / 2.0;
    let center_y = city_height / 2.0;

    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(center_x, center_y, 999.9),
        ..default()
    });
}

fn debug_info(
    vehicles: Query<&simulation::vehicle::Vehicle>,
    mut debug_info: ResMut<DebugInfo>
) {
    let current_count = vehicles.iter().count();
    
    if current_count == 0 && debug_info.last_vehicle_count == 0 {
        println!("No vehicles spawned yet.");
    } else if current_count != debug_info.last_vehicle_count {
        println!("Number of vehicles: {}", current_count);
        debug_info.last_vehicle_count = current_count;
    }
}