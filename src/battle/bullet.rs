use avian2d::prelude::CollisionLayers;
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    bullet::spawn_bullet,
    skill::{FromSkill, Skill, SkillEffctProcessor, SkillResponse, SkillRunContext},
};

#[derive(SystemParam)]
pub struct BulletSystemParam<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub asset_server: Res<'w, AssetServer>,
}

#[derive(Debug, Component)]
pub struct BulletSkillEffect {}

impl FromSkill for BulletSkillEffect {
    fn from_skill(_skill: &Skill) -> Self {
        BulletSkillEffect {}
    }
}

impl<'w, 's> SkillEffctProcessor for BulletSystemParam<'w, 's> {
    type Effect = BulletSkillEffect;

    fn process(
        &mut self,
        _skill_effct: &Self::Effect,
        context: &mut SkillRunContext,
        _response: &mut SkillResponse,
    ) {
        tracing::debug!("Bullet skill effect process start.");

        // let Some(bullet_position) = context.get_data::<Vec2>("bullet_position") else {
        //     tracing::error!("Bullet position not found.");

        //     return;
        // };

        let Some(direction) = context.get_data::<Vec2>("direction") else {
            tracing::error!("Direction not found.");

            return;
        };

        let Some(layers) = context.get_data::<CollisionLayers>("layers") else {
            tracing::error!("Bullet position not found.");

            return;
        };

        spawn_bullet(&mut self.commands, &self.asset_server, *layers, *direction);
    }
}
