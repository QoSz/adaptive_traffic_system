use crate::city::City;
use bevy::prelude::*;
use rand::Rng;

/// Traffic light timing configuration
const RED_DURATION: f32 = 5.0;
const YELLOW_DURATION: f32 = 2.0;
const GREEN_DURATION: f32 = 5.0;
const TIMING_VARIANCE: f32 = 1.0; // Random variance in timing

/// Traffic light states
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

impl TrafficLightState {
    /// Get the color for this state
    pub fn color(&self) -> Color {
        match self {
            TrafficLightState::Red => Color::rgb(0.9, 0.1, 0.1),
            TrafficLightState::Yellow => Color::rgb(0.9, 0.9, 0.1),
            TrafficLightState::Green => Color::rgb(0.1, 0.9, 0.1),
        }
    }

    /// Get the next state in the cycle
    pub fn next(&self) -> TrafficLightState {
        match self {
            TrafficLightState::Red => TrafficLightState::Green,
            TrafficLightState::Green => TrafficLightState::Yellow,
            TrafficLightState::Yellow => TrafficLightState::Red,
        }
    }

    /// Get the base duration for this state
    pub fn base_duration(&self) -> f32 {
        match self {
            TrafficLightState::Red => RED_DURATION,
            TrafficLightState::Yellow => YELLOW_DURATION,
            TrafficLightState::Green => GREEN_DURATION,
        }
    }
}

/// Traffic light component
///
/// Monozukuri principle: Clear state management
/// - Explicit state machine
/// - Timer-based transitions
/// - Grid position for spatial queries
#[derive(Component)]
pub struct TrafficLight {
    pub state: TrafficLightState,
    pub timer: Timer,
    pub grid_position: (u32, u32), // Store grid position for reference
}

impl TrafficLight {
    pub fn new(grid_position: (u32, u32)) -> Self {
        let mut rng = rand::thread_rng();
        let initial_state = TrafficLightState::Red;

        // Add some variance to prevent all lights from syncing
        let initial_duration =
            initial_state.base_duration() + rng.gen_range(-TIMING_VARIANCE..TIMING_VARIANCE);

        TrafficLight {
            state: initial_state,
            timer: Timer::from_seconds(initial_duration, TimerMode::Once),
            grid_position,
        }
    }

    /// Create with a specific initial state (useful for coordinated timing)
    pub fn with_state(grid_position: (u32, u32), state: TrafficLightState) -> Self {
        let mut rng = rand::thread_rng();
        let initial_duration =
            state.base_duration() + rng.gen_range(-TIMING_VARIANCE..TIMING_VARIANCE);

        TrafficLight {
            state,
            timer: Timer::from_seconds(initial_duration, TimerMode::Once),
            grid_position,
        }
    }

    /// Transition to the next state
    pub fn transition(&mut self) {
        let mut rng = rand::thread_rng();
        self.state = self.state.next();
        let duration =
            self.state.base_duration() + rng.gen_range(-TIMING_VARIANCE..TIMING_VARIANCE);
        self.timer = Timer::from_seconds(duration.max(0.5), TimerMode::Once);
    }
}

/// Spawn traffic lights at intersections
///
/// Kaizen principle: Standardized placement
/// - Only at true intersections
/// - Consistent positioning
/// - Visual clarity
pub fn spawn_traffic_lights(mut commands: Commands, city: Res<City>) {
    let mut rng = rand::thread_rng();

    // Get all intersections from the city
    for (grid_x, grid_y) in city.get_intersections() {
        let world_pos = city.grid_to_world(grid_x, grid_y);

        // Alternate initial states for more interesting traffic patterns
        let initial_state = if (grid_x + grid_y) % 2 == 0 {
            TrafficLightState::Red
        } else {
            TrafficLightState::Green
        };

        // Offset position slightly for visibility (above the intersection)
        let light_offset = city.road_width * 0.3;
        let light_pos = Vec3::new(
            world_pos.x + light_offset,
            world_pos.y + light_offset,
            2.0, // Above vehicles
        );

        let traffic_light = TrafficLight::with_state((grid_x, grid_y), initial_state);

        commands.spawn((
            traffic_light,
            SpriteBundle {
                sprite: Sprite {
                    color: initial_state.color(),
                    custom_size: Some(Vec2::new(city.road_width * 0.25, city.road_width * 0.25)),
                    ..default()
                },
                transform: Transform::from_translation(light_pos),
                ..default()
            },
        ));
    }

    println!("Spawned {} traffic lights", city.get_intersections().len());
}

/// Update traffic light states based on timers
pub fn update_traffic_lights(mut query: Query<(&mut TrafficLight, &mut Sprite)>, time: Res<Time>) {
    for (mut traffic_light, mut sprite) in query.iter_mut() {
        traffic_light.timer.tick(time.delta());

        if traffic_light.timer.finished() {
            traffic_light.transition();
            sprite.color = traffic_light.state.color();
        }
    }
}

/// Debug system to show traffic light status
pub fn debug_traffic_lights(
    query: Query<&TrafficLight>,
    mut last_state: Local<Option<Vec<TrafficLightState>>>,
) {
    let states: Vec<_> = query.iter().map(|tl| tl.state).collect();

    // Only print when states change
    if last_state.as_ref() != Some(&states) {
        let red_count = states
            .iter()
            .filter(|s| **s == TrafficLightState::Red)
            .count();
        let green_count = states
            .iter()
            .filter(|s| **s == TrafficLightState::Green)
            .count();
        let yellow_count = states
            .iter()
            .filter(|s| **s == TrafficLightState::Yellow)
            .count();

        if !states.is_empty() {
            println!(
                "Traffic Lights - Red: {}, Yellow: {}, Green: {}",
                red_count, yellow_count, green_count
            );
        }
        *last_state = Some(states);
    }
}
