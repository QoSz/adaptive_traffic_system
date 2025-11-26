use crate::city::{City, RoadType};
use bevy::prelude::*;

/// Road rendering colors
/// Monozukuri principle: Visual clarity through consistent design language
const ROAD_COLOR: Color = Color::rgb(0.3, 0.3, 0.35);
const INTERSECTION_COLOR: Color = Color::rgb(0.35, 0.35, 0.4);
const ROAD_MARKING_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);

/// Render the city road network
///
/// Kaizen principle: Visual management - roads should be clearly distinguishable
pub fn render_city(mut commands: Commands, city: Res<City>, asset_server: Res<AssetServer>) {
    // Try to load road texture, fallback to colored sprites
    let road_texture: Handle<Image> = asset_server.load("road.png");

    for road in &city.roads {
        let world_pos = city.grid_to_world(road.position.0, road.position.1);
        let position = Vec3::new(world_pos.x, world_pos.y, 0.0);

        // Choose color based on road type
        let color = match road.road_type {
            RoadType::Intersection => INTERSECTION_COLOR,
            RoadType::Horizontal | RoadType::Vertical => ROAD_COLOR,
        };

        // Spawn road tile
        commands.spawn(SpriteBundle {
            texture: road_texture.clone(),
            transform: Transform::from_translation(position),
            sprite: Sprite {
                color,
                custom_size: Some(Vec2::new(city.road_width, city.road_width)),
                ..default()
            },
            ..default()
        });

        // Add road markings for non-intersection roads
        if road.road_type != RoadType::Intersection {
            spawn_road_markings(&mut commands, position, road.road_type, city.road_width);
        }
    }
}

/// Spawn center line markings on roads
fn spawn_road_markings(
    commands: &mut Commands,
    road_position: Vec3,
    road_type: RoadType,
    road_width: f32,
) {
    let marking_size = match road_type {
        RoadType::Horizontal => Vec2::new(road_width * 0.6, road_width * 0.03),
        RoadType::Vertical => Vec2::new(road_width * 0.03, road_width * 0.6),
        RoadType::Intersection => return,
    };

    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: ROAD_MARKING_COLOR,
            custom_size: Some(marking_size),
            ..default()
        },
        transform: Transform::from_xyz(
            road_position.x,
            road_position.y,
            0.1, // Slightly above road
        ),
        ..default()
    });
}

/// Spawn a background for the city area
pub fn render_city_background(mut commands: Commands, city: Res<City>) {
    let world_size = city.world_size();
    let center = city.world_center();

    // Background grass/ground color
    commands.spawn(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.2, 0.4, 0.2), // Dark green
            custom_size: Some(Vec2::new(
                world_size.x + city.road_width * 2.0,
                world_size.y + city.road_width * 2.0,
            )),
            ..default()
        },
        transform: Transform::from_xyz(center.x, center.y, -1.0),
        ..default()
    });
}
