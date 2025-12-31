mod attribute;

use std::fmt::Debug;

pub use attribute::*;

use bevy::{ecs::system::SystemParam, platform::collections::HashMap, prelude::*};

#[derive(Debug, SystemParam)]
pub struct SkillSystemParam {}

///技能运行过程数据
#[derive(Debug, Component)]
pub struct SkillRunContext {
    pub source: Option<Entity>,
    pub caster: Entity,
    pub target: Entity,
    pub payload: HashMap<String, Box<dyn SkillData>>,
}

pub trait SkillData: 'static + Sync + Send + Debug {}

pub struct Skill {}

pub trait FromSkill {
    fn from_skill(skill: &Skill) -> Self;
}

#[derive(Debug, SystemParam)]
pub struct MySystemParam {}

#[derive(Debug, Component)]
pub struct MySkillEffect {}

impl FromSkill for MySkillEffect {
    fn from_skill(_skill: &Skill) -> Self {
        MySkillEffect {}
    }
}

impl SkillEffctProcessor for MySystemParam {
    type Effect = MySkillEffect;
}

pub struct SkillResponse {}

impl SkillResponse {
    pub fn empty() -> SkillResponse {
        SkillResponse {}
    }
}

pub trait SkillEffctProcessor {
    type Effect: FromSkill + Component;

    fn process(
        &self,
        _skill_effct: &Self::Effect,
        _context: &mut SkillRunContext,
    ) -> SkillResponse {
        SkillResponse::empty()
    }
}

pub fn process_skill_effct<T: SystemParam + SkillEffctProcessor>(
    processor: T,
    mut skill_effct_q: Query<(&T::Effect, &mut SkillRunContext)>,
) {
    for (skill_effct, mut context) in skill_effct_q.iter_mut() {
        processor.process(skill_effct, &mut context);
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Last, process_skill_effct::<MySystemParam>);
}
