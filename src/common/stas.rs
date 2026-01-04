use bevy::ecs::component::Component;

#[derive(Debug, Component)]
pub struct Stas {
    pub health: i32,
    pub health_max: i32,
}

impl Default for Stas {
    fn default() -> Self {
        Self {
            health: 10,
            health_max: 10,
        }
    }
}
