pub mod renderer;
pub mod road;

use self::road::Road;
use bevy::prelude::*;

/// Direction a vehicle can travel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    /// Convert direction to a normalized Vec2
    pub fn to_vec2(self) -> Vec2 {
        match self {
            Direction::North => Vec2::Y,
            Direction::South => Vec2::NEG_Y,
            Direction::East => Vec2::X,
            Direction::West => Vec2::NEG_X,
        }
    }

    /// Get opposite direction
    pub fn opposite(self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    /// All four cardinal directions
    pub fn all() -> [Direction; 4] {
        [
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }
}

/// Road type for better visualization and logic
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoadType {
    Intersection,
    Horizontal,
    Vertical,
}

/// City resource that manages the road network grid
///
/// Monozukuri principle: Quality built into the design
/// - Clear grid-based coordinate system
/// - Efficient spatial queries
/// - Immutable road network after creation
#[derive(Resource)]
pub struct City {
    pub size: (u32, u32),
    pub roads: Vec<Road>,
    pub road_width: f32,
    pub grid_spacing: u32, // Road grid interval (roads every N cells)
}

impl City {
    /// Create a new city with a grid-based road network
    ///
    /// Roads are placed at regular intervals creating a grid pattern:
    /// - Horizontal roads at y positions where y % grid_spacing == 0
    /// - Vertical roads at x positions where x % grid_spacing == 0
    /// - Intersections where both conditions are true
    pub fn new(size: (u32, u32), road_width: f32, grid_spacing: u32) -> Self {
        let mut roads: Vec<Road> = Vec::new();

        for x in 0..=size.0 {
            for y in 0..=size.1 {
                let is_on_h_road = y % grid_spacing == 0;
                let is_on_v_road = x % grid_spacing == 0;

                if is_on_h_road || is_on_v_road {
                    let road_type = if is_on_h_road && is_on_v_road {
                        RoadType::Intersection
                    } else if is_on_h_road {
                        RoadType::Horizontal
                    } else {
                        RoadType::Vertical
                    };
                    roads.push(Road::new((x, y), road_type));
                }
            }
        }

        City {
            size,
            roads,
            road_width,
            grid_spacing,
        }
    }

    /// Convert grid coordinates to world position
    pub fn grid_to_world(&self, grid_x: u32, grid_y: u32) -> Vec2 {
        Vec2::new(
            grid_x as f32 * self.road_width,
            grid_y as f32 * self.road_width,
        )
    }

    /// Convert world position to grid coordinates (clamped to valid range)
    pub fn world_to_grid(&self, position: Vec2) -> (u32, u32) {
        let x = (position.x / self.road_width)
            .round()
            .clamp(0.0, self.size.0 as f32) as u32;
        let y = (position.y / self.road_width)
            .round()
            .clamp(0.0, self.size.1 as f32) as u32;
        (x, y)
    }

    /// Check if a grid position has a road
    pub fn has_road_at_grid(&self, x: u32, y: u32) -> bool {
        x <= self.size.0
            && y <= self.size.1
            && (x % self.grid_spacing == 0 || y % self.grid_spacing == 0)
    }

    /// Check if a world position is on a road (with tolerance)
    pub fn is_on_road(&self, position: Vec2) -> bool {
        let (grid_x, grid_y) = self.world_to_grid(position);
        self.has_road_at_grid(grid_x, grid_y)
    }

    /// Check if a grid position is an intersection
    pub fn is_intersection_at_grid(&self, x: u32, y: u32) -> bool {
        x <= self.size.0
            && y <= self.size.1
            && x % self.grid_spacing == 0
            && y % self.grid_spacing == 0
    }

    /// Check if a world position is at an intersection
    pub fn is_intersection(&self, position: Vec2) -> bool {
        let (grid_x, grid_y) = self.world_to_grid(position);
        self.is_intersection_at_grid(grid_x, grid_y)
    }

    /// Get the road type at a grid position
    pub fn road_type_at_grid(&self, x: u32, y: u32) -> Option<RoadType> {
        if !self.has_road_at_grid(x, y) {
            return None;
        }

        let is_on_h = y % self.grid_spacing == 0;
        let is_on_v = x % self.grid_spacing == 0;

        Some(if is_on_h && is_on_v {
            RoadType::Intersection
        } else if is_on_h {
            RoadType::Horizontal
        } else {
            RoadType::Vertical
        })
    }

    /// Find the nearest road position from a world position
    /// This properly handles the grid logic to find actual roads
    pub fn nearest_road_position(&self, position: Vec2) -> Vec2 {
        let (grid_x, grid_y) = self.world_to_grid(position);

        // If already on a road, return the snapped position
        if self.has_road_at_grid(grid_x, grid_y) {
            return self.grid_to_world(grid_x, grid_y);
        }

        // Find nearest horizontal road (y coordinate)
        let nearest_h_road_y =
            ((grid_y as f32 / self.grid_spacing as f32).round() * self.grid_spacing as f32) as u32;
        let nearest_h_road_y = nearest_h_road_y.min(self.size.1);

        // Find nearest vertical road (x coordinate)
        let nearest_v_road_x =
            ((grid_x as f32 / self.grid_spacing as f32).round() * self.grid_spacing as f32) as u32;
        let nearest_v_road_x = nearest_v_road_x.min(self.size.0);

        // Calculate distances to nearest horizontal and vertical roads
        let dist_to_h = (grid_y as i32 - nearest_h_road_y as i32).abs() as f32;
        let dist_to_v = (grid_x as i32 - nearest_v_road_x as i32).abs() as f32;

        // Return the closest road position
        if dist_to_h <= dist_to_v {
            self.grid_to_world(grid_x, nearest_h_road_y)
        } else {
            self.grid_to_world(nearest_v_road_x, grid_y)
        }
    }

    /// Get available movement directions from a position
    /// Only returns directions that lead to valid road positions
    pub fn available_directions(&self, position: Vec2) -> Vec<Direction> {
        let (grid_x, grid_y) = self.world_to_grid(position);
        let mut directions = Vec::new();

        // Check each cardinal direction
        if grid_x > 0 && self.has_road_at_grid(grid_x - 1, grid_y) {
            directions.push(Direction::West);
        }
        if grid_x < self.size.0 && self.has_road_at_grid(grid_x + 1, grid_y) {
            directions.push(Direction::East);
        }
        if grid_y > 0 && self.has_road_at_grid(grid_x, grid_y - 1) {
            directions.push(Direction::South);
        }
        if grid_y < self.size.1 && self.has_road_at_grid(grid_x, grid_y + 1) {
            directions.push(Direction::North);
        }

        directions
    }

    /// Check if a world position is within city bounds
    pub fn is_within_bounds(&self, position: Vec2) -> bool {
        position.x >= 0.0
            && position.x <= self.size.0 as f32 * self.road_width
            && position.y >= 0.0
            && position.y <= self.size.1 as f32 * self.road_width
    }

    /// Clamp a position to stay within city bounds
    pub fn clamp_to_bounds(&self, position: Vec2) -> Vec2 {
        Vec2::new(
            position.x.clamp(0.0, self.size.0 as f32 * self.road_width),
            position.y.clamp(0.0, self.size.1 as f32 * self.road_width),
        )
    }

    /// Get city dimensions in world units
    pub fn world_size(&self) -> Vec2 {
        Vec2::new(
            self.size.0 as f32 * self.road_width,
            self.size.1 as f32 * self.road_width,
        )
    }

    /// Get city center in world units
    pub fn world_center(&self) -> Vec2 {
        self.world_size() / 2.0
    }

    /// Get all intersection positions (for traffic lights)
    pub fn get_intersections(&self) -> Vec<(u32, u32)> {
        let mut intersections = Vec::new();
        for x in (0..=self.size.0).step_by(self.grid_spacing as usize) {
            for y in (0..=self.size.1).step_by(self.grid_spacing as usize) {
                intersections.push((x, y));
            }
        }
        intersections
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_road_generation() {
        let city = City::new((20, 20), 20.0, 5);
        assert!(!city.roads.is_empty());
    }

    #[test]
    fn test_is_on_road() {
        let city = City::new((20, 20), 20.0, 5);
        // Position (0, 0) should be on a road
        assert!(city.is_on_road(Vec2::new(0.0, 0.0)));
        // Position (100, 0) = grid (5, 0) should be on a road
        assert!(city.is_on_road(Vec2::new(100.0, 0.0)));
    }

    #[test]
    fn test_is_intersection() {
        let city = City::new((20, 20), 20.0, 5);
        // (0, 0) is an intersection
        assert!(city.is_intersection(Vec2::new(0.0, 0.0)));
        // (5*20, 5*20) = (100, 100) is an intersection
        assert!(city.is_intersection(Vec2::new(100.0, 100.0)));
        // (20, 0) = grid (1, 0) is NOT an intersection
        assert!(!city.is_intersection(Vec2::new(20.0, 0.0)));
    }
}
