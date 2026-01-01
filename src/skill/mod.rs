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
    pub payload: HashMap<String, Box<dyn SkillRunData>>,
}

pub trait SkillRunData: 'static + Sync + Send + Debug {}

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

pub trait SkillCommand: 'static + Send + Sync + Debug {
    fn execute(&self) {}
}

#[derive(Debug, Component)]
pub struct SkillResponse {
    commands: Vec<Box<dyn SkillCommand>>,
}

impl SkillResponse {
    pub fn empty() -> SkillResponse {
        SkillResponse {
            commands: Default::default(),
        }
    }

    pub fn execute(&self) {
        for command in self.commands.iter() {
            command.execute();
        }
    }

    pub fn merge(&mut self, mut other: SkillResponse) {
        self.commands.append(&mut other.commands);
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

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum SkillSystems {
    Update,
    Execute,
}

pub fn process_skill_effct<T: SystemParam + SkillEffctProcessor>(
    processor: T,
    mut skill_effct_q: Query<(&T::Effect, &mut SkillRunContext, &mut SkillResponse)>,
) {
    for (skill_effct, mut context, mut res) in skill_effct_q.iter_mut() {
        res.merge(processor.process(skill_effct, &mut context));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(Last, (SkillSystems::Update, SkillSystems::Execute).chain());

    app.add_systems(
        Last,
        process_skill_effct::<MySystemParam>.in_set(SkillSystems::Update),
    );
}
