use bevy::ecs::component::Component;

#[derive(Debug, Component)]
pub struct Stas {
    pub health: i32,
    pub health_max: i32,
}

impl Stas {
    pub fn update_health(&mut self, value: i32) {
        self.health += value;
        self.health = self.health.clamp(0, self.health_max);
    }

    pub fn is_die(&self) -> bool {
        self.health == 0
    }
}

impl Default for Stas {
    fn default() -> Self {
        Self {
            health: 10,
            health_max: 10,
        }
    }
}
