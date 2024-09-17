use bevy::prelude::*;
mod city;
mod simulation;

use city::City;
use city::renderer::render_city;
use simulation::traffic_generator::spawn_vehicles;
use simulation::movement::move_vehicles;
use simulation::traffic_light::{spawn_traffic_lights, update_traffic_lights};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(City::new((20, 20), 20.0))
        .add_startup_system(setup)
        .add_startup_system(render_city)
        .add_startup_system(spawn_traffic_lights)
        .add_system(spawn_vehicles)
        .add_system(move_vehicles)
        .add_system(update_traffic_lights)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}