use avian2d::prelude::CollisionLayers;
use bevy::{ecs::system::SystemParam, prelude::*};

use crate::{
    bullet::spawn_bullet,
    skill::{
        FromSkill, Skill, SkillEffctProcessor, SkillResponse, SkillRunContextData,
        SkillRunContextDataBuilder,
    },
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

pub struct BulletContext {
    pub direction: Vec2,
    pub layers: CollisionLayers,
    pub bullet_position: Vec2,
}

impl SkillRunContextDataBuilder for BulletContext {
    fn unique_name() -> &'static str {
        "bullet"
    }

    fn from_skill_run_context_data(data: &SkillRunContextData) -> Option<Self> {
        let Some(bullet_position) =
            data.get_value::<Vec2>(&Self::get_property_name("bullet_position"))
        else {
            tracing::error!("Bullet position not found.");

            return None;
        };

        let Some(direction) = data.get_value::<Vec2>(&Self::get_property_name("direction")) else {
            tracing::error!("Direction not found.");

            return None;
        };

        let Some(layers) = data.get_value::<CollisionLayers>(&Self::get_property_name("layers"))
        else {
            tracing::error!("Bullet position not found.");
            return None;
        };

        Some(Self {
            direction: *direction,
            layers: *layers,
            bullet_position: *bullet_position,
        })
    }

    fn update_skill_run_context_data(&self, data: &mut SkillRunContextData) {
        data.set_value(&Self::get_property_name("direction"), self.direction);
        data.set_value(&Self::get_property_name("layers"), self.layers);
        data.set_value(
            &Self::get_property_name("bullet_position"),
            self.bullet_position,
        );
    }
}

impl<'w, 's> SkillEffctProcessor for BulletSystemParam<'w, 's> {
    type Effect = BulletSkillEffect;
    type Context = BulletContext;
    type Response = ();

    fn process(
        &mut self,
        _skill_effct: &Self::Effect,
        context: &BulletContext,
        _response: &mut SkillResponse,
    ) {
        tracing::debug!("Bullet skill effect process start.");
        spawn_bullet(
            &mut self.commands,
            &self.asset_server,
            context.layers,
            context.direction,
            context.bullet_position,
        );
    }
}
