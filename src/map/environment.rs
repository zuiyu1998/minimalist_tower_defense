use bevy::prelude::*;

use crate::skill::SkillAttributeSet;

///环境
#[derive(Component)]
pub struct MapEnvironment(SkillAttributeSet);

impl MapEnvironment {
    pub fn get_property(&self, name: &str) -> i32 {
        if let Some(attribute) = self.0.skill_attribute(name) {
            attribute.get_current_value()
        } else {
            tracing::warn!("{} not match.", name);
            return 0;
        }
    }
}

impl Default for MapEnvironment {
    fn default() -> Self {
        MapEnvironment(Default::default())
    }
}
