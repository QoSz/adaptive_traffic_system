pub mod road;
pub mod renderer;

use bevy::prelude::*;
use self::road::Road;

#[derive(Resource)]
pub struct City {
    pub size: (u32, u32),
    pub roads: Vec<Road>,
    pub road_width: f32,
}

impl City {
    pub fn new(size: (u32, u32), road_width: f32) -> Self {
        let mut roads: Vec<Road> = Vec::new();
        for x in 0..=size.0 {
            for y in 0..=size.1 {
                if x % 5 == 0 || y % 5 == 0 {
                    roads.push(Road::new((x, y)));
                }
            }
        }
        City { size, roads, road_width }
    }

    pub fn is_on_road(&self, position: Vec2) -> bool {
        let (x, y): (u32, u32) = (
            (position.x / self.road_width).round() as u32,
            (position.y / self.road_width).round() as u32
        );
        self.roads.iter().any(|road| road.position == (x, y))
    }

    pub fn is_intersection(&self, position: Vec2) -> bool {
        let (x, y): (u32, u32) = (
            (position.x / self.road_width).round() as u32,
            (position.y / self.road_width).round() as u32
        );
        x % 5 == 0 && y % 5 == 0
    }

    pub fn nearest_road_center(&self, position: Vec2) -> Vec2 {
        let x: f32 = (position.x / self.road_width).round().clamp(0.0, self.size.0 as f32);
        let y: f32 = (position.y / self.road_width).round().clamp(0.0, self.size.1 as f32);
        
        let x: f32 = if x % 5.0 != 0.0 { (x / 5.0).round() * 5.0 } else { x };
        let y: f32 = if y % 5.0 != 0.0 { (y / 5.0).round() * 5.0 } else { y };
        
        Vec2::new(x * self.road_width, y * self.road_width)
    }

    pub fn available_directions(&self, position: Vec2) -> Vec<Vec2> {
        let (x, y): (u32, u32) = (
            (position.x / self.road_width).round() as u32,
            (position.y / self.road_width).round() as u32
        );
        let mut directions: Vec<Vec2> = Vec::new();
        
        if x > 0 && self.is_on_road(Vec2::new((x - 1) as f32 * self.road_width, y as f32 * self.road_width)) {
            directions.push(Vec2::NEG_X);
        }
        if x < self.size.0 && self.is_on_road(Vec2::new((x + 1) as f32 * self.road_width, y as f32 * self.road_width)) {
            directions.push(Vec2::X);
        }
        if y > 0 && self.is_on_road(Vec2::new(x as f32 * self.road_width, (y - 1) as f32 * self.road_width)) {
            directions.push(Vec2::NEG_Y);
        }
        if y < self.size.1 && self.is_on_road(Vec2::new(x as f32 * self.road_width, (y + 1) as f32 * self.road_width)) {
            directions.push(Vec2::Y);
        }
        
        directions
    }

    pub fn is_valid_position(&self, position: Vec2) -> bool {
        position.x >= 0.0 && position.x <= self.size.0 as f32 * self.road_width &&
        position.y >= 0.0 && position.y <= self.size.1 as f32 * self.road_width
    }
}