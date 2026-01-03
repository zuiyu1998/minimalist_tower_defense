mod attribute;

use std::{fmt::Debug, ops::Deref};

pub use attribute::*;

use bevy::{platform::collections::HashMap, prelude::*};
use downcast_rs::{Downcast, impl_downcast};

///技能运行过程数据
#[derive(Debug, Component)]
pub struct SkillRunContext {
    pub source: Option<Entity>,
    pub caster: Entity,
    pub target: Entity,
    pub data: SkillRunContextData,
}

#[derive(Debug, Default)]
pub struct SkillRunContextData(HashMap<String, Box<dyn SkillRunData>>);

impl SkillRunContextData {
    pub fn set_data<T: SkillRunData>(&mut self, name: &str, value: T) {
        self.0.insert(name.to_string(), Box::new(value));
    }
}

impl Clone for SkillRunContextData {
    fn clone(&self) -> Self {
        let mut map = HashMap::default();

        for (key, value) in self.0.iter() {
            map.insert(key.clone(), value.clone());
        }

        SkillRunContextData(map)
    }
}

impl SkillRunContext {
    pub fn get_data<T: SkillRunData>(&self, name: &str) -> Option<&T> {
        self.data.0.get(name).and_then(|data| data.downcast_ref())
    }
}

pub trait SkillRunData: 'static + Sync + Send + Debug + Downcast {
    fn clone_boxed(&self) -> Box<dyn SkillRunData>;
}

impl_downcast!(SkillRunData);

impl Clone for Box<dyn SkillRunData> {
    fn clone(&self) -> Self {
        self.deref().clone_boxed()
    }
}

impl<T: Clone + 'static + Sync + Send + Debug> SkillRunData for T {
    fn clone_boxed(&self) -> Box<dyn SkillRunData> {
        Box::new(self.clone())
    }
}

#[derive(Debug, Component)]
pub struct Skill {}

pub trait FromSkill {
    fn from_skill(skill: &Skill) -> Self;
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
}

pub trait SkillEffctProcessor {
    type Effect: FromSkill + Component;

    fn process(
        &mut self,
        _skill_effct: &Self::Effect,
        _context: &mut SkillRunContext,
        response: &mut SkillResponse,
    );
}

#[derive(SystemSet, Debug, Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub enum SkillSystems {
    Update,
    Execute,
    Free,
}

pub fn process_skill_effct<T: SkillEffctProcessor>(
    processor: &mut T,
    mut skill_effct_q: Query<(&T::Effect, &mut SkillRunContext, &mut SkillResponse)>,
) {
    for (skill_effct, mut context, mut res) in skill_effct_q.iter_mut() {
        processor.process(skill_effct, &mut context, &mut res);
    }
}

fn free(mut commands: Commands, skill_effct_q: Query<Entity, With<SkillRunContext>>) {
    for entity in skill_effct_q.iter() {
        commands.entity(entity).despawn();
    }
}

pub(super) fn plugin(app: &mut App) {
    app.configure_sets(
        Last,
        (
            SkillSystems::Update,
            SkillSystems::Execute,
            SkillSystems::Free,
        )
            .chain(),
    );

    app.add_systems(Last, free.in_set(SkillSystems::Free));
}
