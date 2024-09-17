use bevy::prelude::Component;

#[derive(Component)]
pub struct Road {
    pub position: (u32, u32),
}

impl Road {
    pub fn new(position: (u32, u32)) -> Self {
        Road { position }
    }
}