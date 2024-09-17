use bevy::prelude::*;
use rand::Rng;
use crate::city::City;

#[derive(Component)]
pub struct TrafficLight {
    pub state: TrafficLightState,
    pub timer: Timer,
}

#[derive(Clone, Copy)]
pub enum TrafficLightState {
    Red,
    Yellow,
    Green,
}

impl TrafficLight {
    pub fn new() -> Self {
        TrafficLight {
            state: TrafficLightState::Red,
            timer: Timer::from_seconds(5.0 + rand::thread_rng().gen_range(0.0..5.0), TimerMode::Repeating),
        }
    }
}

pub fn spawn_traffic_lights(
    mut commands: Commands,
    city: Res<City>,
) {
    for x in (0..=city.size.0).step_by(10) {
        for y in (0..=city.size.1).step_by(10) {
            if city.is_intersection(Vec2::new(x as f32 * city.road_width, y as f32 * city.road_width)) {
                let position = Vec3::new(
                    x as f32 * city.road_width,
                    y as f32 * city.road_width,
                    1.0,
                );
                commands.spawn((
                    TrafficLight::new(),
                    SpriteBundle {
                        sprite: Sprite {
                            color: Color::RED,
                            custom_size: Some(Vec2::new(city.road_width * 0.2, city.road_width * 0.2)),
                            ..default()
                        },
                        transform: Transform::from_translation(position),
                        ..default()
                    },
                ));
            }
        }
    }
}

pub fn update_traffic_lights(
    mut query: Query<(&mut TrafficLight, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut traffic_light, mut sprite) in query.iter_mut() {
        traffic_light.timer.tick(time.delta());

        if traffic_light.timer.just_finished() {
            traffic_light.state = match traffic_light.state {
                TrafficLightState::Red => TrafficLightState::Green,
                TrafficLightState::Yellow => TrafficLightState::Red,
                TrafficLightState::Green => TrafficLightState::Yellow,
            };

            sprite.color = match traffic_light.state {
                TrafficLightState::Red => Color::RED,
                TrafficLightState::Yellow => Color::YELLOW,
                TrafficLightState::Green => Color::GREEN,
            };
            
            // Reset timer with a slight randomness
            traffic_light.timer.set_duration(std::time::Duration::from_secs_f32(5.0 + rand::thread_rng().gen_range(0.0..2.0)));
        }
    }
}