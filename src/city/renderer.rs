use bevy::prelude::*;
use crate::city::City;

pub fn render_city(
    mut commands: Commands,
    city: Res<City>,
    asset_server: Res<AssetServer>,
) {
    let road_texture: Handle<Image> = asset_server.load("road.png");
    
    for road in &city.roads {
        let position: Vec3 = Vec3::new(
            road.position.0 as f32 * city.road_width, 
            road.position.1 as f32 * city.road_width, 
            0.0
        );
        commands.spawn(SpriteBundle {
            texture: road_texture.clone(),
            transform: Transform::from_translation(position),
            sprite: Sprite {
                custom_size: Some(Vec2::new(city.road_width, city.road_width)),
                ..default()
            },
            ..default()
        });
    }
}