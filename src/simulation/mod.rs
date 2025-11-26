//! Traffic Simulation Module
//!
//! This module implements the core traffic simulation logic following
//! Kaizen (continuous improvement) and Monozukuri (craftsmanship) principles.
//!
//! ## Components
//! - `vehicle`: Vehicle entities and their behavior
//! - `movement`: Physics and movement systems
//! - `traffic_light`: Traffic light state management
//! - `traffic_generator`: Vehicle spawning logic

pub mod movement;
pub mod traffic_generator;
pub mod traffic_light;
pub mod vehicle;

// Re-export commonly used items
pub use movement::{check_traffic_lights, cleanup_stuck_vehicles, move_vehicles};
pub use traffic_generator::{adjust_spawn_rate, spawn_vehicles, SpawnTimer};
pub use traffic_light::{
    spawn_traffic_lights, update_traffic_lights, TrafficLight, TrafficLightState,
};
pub use vehicle::{despawn_vehicles, spawn_vehicle, update_intersection_cooldowns, Vehicle};
