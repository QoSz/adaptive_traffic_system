use super::RoadType;
use bevy::prelude::Component;

/// Road component representing a single road tile in the city grid
///
/// Monozukuri principle: Each part serves a clear purpose
/// - Position: Grid coordinates (not world coordinates)
/// - Type: Determines rendering and vehicle behavior
#[derive(Component, Debug, Clone)]
pub struct Road {
    pub position: (u32, u32),
    pub road_type: RoadType,
}

impl Road {
    pub fn new(position: (u32, u32), road_type: RoadType) -> Self {
        Road {
            position,
            road_type,
        }
    }

    /// Check if this road is an intersection
    pub fn is_intersection(&self) -> bool {
        matches!(self.road_type, RoadType::Intersection)
    }

    /// Check if this road is horizontal
    pub fn is_horizontal(&self) -> bool {
        matches!(self.road_type, RoadType::Horizontal)
    }

    /// Check if this road is vertical
    pub fn is_vertical(&self) -> bool {
        matches!(self.road_type, RoadType::Vertical)
    }
}
