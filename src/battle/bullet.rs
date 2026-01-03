use bevy::{ecs::system::SystemParam, prelude::*};

use crate::skill::{FromSkill, Skill, SkillEffctProcessor, SkillResponse, SkillRunContext};

#[derive(Debug, SystemParam)]
pub struct BulletSystemParam {}

#[derive(Debug, Component)]
pub struct BulletSkillEffect {}

impl FromSkill for BulletSkillEffect {
    fn from_skill(_skill: &Skill) -> Self {
        BulletSkillEffect {}
    }
}

impl SkillEffctProcessor for BulletSystemParam {
    type Effect = BulletSkillEffect;

    fn process(
        &self,
        _skill_effct: &Self::Effect,
        _context: &mut SkillRunContext,
    ) -> SkillResponse {
        println!("bullet skill effect process");

        SkillResponse::empty()
    }
}
