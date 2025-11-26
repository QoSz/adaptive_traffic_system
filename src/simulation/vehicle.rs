use crate::city::{City, Direction};
use bevy::prelude::*;
use rand::Rng;

/// Base speed for vehicles in world units per second
pub const VEHICLE_SPEED: f32 = 60.0;

/// Vehicle component representing a car in the simulation
///
/// Monozukuri principle: Clear, single-purpose components
/// - No redundant state (position comes from Transform)
/// - Direction as enum for type safety
/// - Speed as scalar for easy modification
#[derive(Component, Clone)]
pub struct Vehicle {
    pub direction: Direction,
    pub speed: f32,
    pub color: Color,
    pub stopped: bool, // Track if vehicle is stopped (for velocity restoration)
}

impl Vehicle {
    /// Create a new vehicle with a random direction from available options
    pub fn new(available_directions: &[Direction]) -> Self {
        let mut rng = rand::thread_rng();

        // Choose a random direction from available options
        let direction = if available_directions.is_empty() {
            // Fallback to random direction if none available
            *Direction::all().get(rng.gen_range(0..4)).unwrap()
        } else {
            *available_directions
                .get(rng.gen_range(0..available_directions.len()))
                .unwrap()
        };

        // Generate a nice random color
        let color = Color::rgb(
            rng.gen_range(0.3..1.0),
            rng.gen_range(0.3..1.0),
            rng.gen_range(0.3..1.0),
        );

        Vehicle {
            direction,
            speed: VEHICLE_SPEED,
            color,
            stopped: false,
        }
    }

    /// Create a vehicle with a specific direction
    pub fn with_direction(direction: Direction) -> Self {
        let mut rng = rand::thread_rng();
        let color = Color::rgb(
            rng.gen_range(0.3..1.0),
            rng.gen_range(0.3..1.0),
            rng.gen_range(0.3..1.0),
        );

        Vehicle {
            direction,
            speed: VEHICLE_SPEED,
            color,
            stopped: false,
        }
    }

    /// Get the velocity vector based on direction and speed
    pub fn velocity(&self) -> Vec2 {
        if self.stopped {
            Vec2::ZERO
        } else {
            self.direction.to_vec2() * self.speed
        }
    }

    /// Stop the vehicle
    pub fn stop(&mut self) {
        self.stopped = true;
    }

    /// Resume the vehicle
    pub fn resume(&mut self) {
        self.stopped = false;
    }

    /// Change direction
    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }
}

/// Marker component for vehicles that should be despawned
#[derive(Component)]
pub struct DespawnVehicle;

/// Component to track if vehicle passed through an intersection recently
/// Prevents rapid direction changes
#[derive(Component)]
pub struct IntersectionCooldown {
    pub timer: Timer,
}

impl Default for IntersectionCooldown {
    fn default() -> Self {
        IntersectionCooldown {
            timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }
}

/// Spawn a vehicle entity at a specific position with proper components
///
/// Kaizen principle: Standardized work - consistent vehicle creation
pub fn spawn_vehicle(commands: &mut Commands, position: Vec2, city: &Res<City>) {
    // Get available directions at spawn position
    let available_directions = city.available_directions(position);

    // Don't spawn if no directions available
    if available_directions.is_empty() {
        return;
    }

    let vehicle = Vehicle::new(&available_directions);
    let vehicle_size = city.road_width * 0.5;

    // Calculate rotation based on direction
    let rotation = match vehicle.direction {
        Direction::North => 0.0,
        Direction::South => std::f32::consts::PI,
        Direction::East => -std::f32::consts::FRAC_PI_2,
        Direction::West => std::f32::consts::FRAC_PI_2,
    };

    commands.spawn((
        vehicle.clone(),
        SpriteBundle {
            sprite: Sprite {
                color: vehicle.color,
                custom_size: Some(Vec2::new(vehicle_size * 0.6, vehicle_size)),
                ..default()
            },
            transform: Transform::from_xyz(position.x, position.y, 1.0)
                .with_rotation(Quat::from_rotation_z(rotation)),
            ..default()
        },
        IntersectionCooldown::default(),
    ));
}

/// Update the intersection cooldown timers
pub fn update_intersection_cooldowns(mut query: Query<&mut IntersectionCooldown>, time: Res<Time>) {
    for mut cooldown in query.iter_mut() {
        cooldown.timer.tick(time.delta());
    }
}

/// Despawn vehicles marked for removal
pub fn despawn_vehicles(mut commands: Commands, query: Query<Entity, With<DespawnVehicle>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
