use std::marker::PhantomData;

use bevy::prelude::*;

use crate::map::MapEnvironment;

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Lair;

pub trait LairTrait: 'static + Send + Sync {
    fn spaw_enemy(&self, _commands: &mut EntityCommands, _map_environment: &MapEnvironment);
}

///方块巢穴
#[derive(Debug, Component)]
pub struct SquareLarir {
    timer: Timer,
}

impl Default for SquareLarir {
    fn default() -> Self {
        SquareLarir {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
        }
    }
}

impl LairTrait for SquareLarir {
    fn spaw_enemy(&self, _commands: &mut EntityCommands, _map_environment: &MapEnvironment) {}
}

fn larir_process(
    mut commands: Commands,
    map: Single<(Entity, &MapEnvironment)>,
    mut lair_q: Query<&mut SquareLarir>,
    time: Res<Time>,
) {
    let (map_entity, map_environment) = map.into_inner();

    let mut commands = commands.entity(map_entity);

    for mut lair in lair_q.iter_mut() {
        lair.timer.tick(time.delta());

        if lair.timer.just_finished() {
            lair.spaw_enemy(&mut commands, map_environment);
        }
    }
}

#[derive(Default)]
pub struct LairPlugin<T> {
    _marker: PhantomData<T>,
}

impl<T: LairTrait> Plugin for LairPlugin<T> {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, larir_process);
    }
}

pub fn spawn_lair(commands: &mut Commands) -> Entity {
    commands.spawn((Lair, Name::new("Lair"))).id()
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((LairPlugin::<SquareLarir>::default(),));
}
