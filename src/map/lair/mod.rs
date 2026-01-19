use std::marker::PhantomData;

use bevy::prelude::*;

use crate::{
    enemy::{EnemySpawnerContainer, SquareEnemySpawner},
    map::MapEnvironment,
};

#[derive(Debug, Component)]
#[require(Transform)]
pub struct Lair;

pub trait LairTrait: 'static + Send + Sync {
    fn spaw_enemy(
        &self,
        _commands: &mut EntityCommands,
        _asset_server: &AssetServer,
        _position: Vec3,
        _enemy: &str,
        _map_environment: &MapEnvironment,
    );
}

///方块巢穴
#[derive(Debug, Component)]
pub struct SquareLarir {
    timer: Timer,
    enemy_spawner_container: EnemySpawnerContainer,
}

impl SquareLarir {
    pub fn new() -> Self {
        let mut larir = Self::empty();
        larir
            .enemy_spawner_container
            .register("square", SquareEnemySpawner);

        larir
    }

    pub fn empty() -> Self {
        SquareLarir {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            enemy_spawner_container: EnemySpawnerContainer::empty(),
        }
    }
}

impl Default for SquareLarir {
    fn default() -> Self {
        Self::new()
    }
}

impl LairTrait for SquareLarir {
    fn spaw_enemy(
        &self,
        commands: &mut EntityCommands,
        asset_server: &AssetServer,
        position: Vec3,
        enemy: &str,
        map_environment: &MapEnvironment,
    ) {
        let _v = map_environment.get_property("test");

        self.enemy_spawner_container
            .spawn_enemy(commands, asset_server, position, enemy);
    }
}

fn larir_process(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map: Single<(Entity, &MapEnvironment)>,
    mut lair_q: Query<(&mut SquareLarir, &Transform)>,
    time: Res<Time>,
) {
    let (map_entity, map_environment) = map.into_inner();

    let mut commands = commands.entity(map_entity);

    for (mut lair, transorm) in lair_q.iter_mut() {
        lair.timer.tick(time.delta());

        if lair.timer.just_finished() {
            lair.spaw_enemy(
                &mut commands,
                &asset_server,
                transorm.translation,
                "square",
                map_environment,
            );
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
    commands
        .spawn((
            SquareLarir::default(),
            Lair,
            Name::new("Lair"),
            Transform { ..default() },
        ))
        .id()
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((LairPlugin::<SquareLarir>::default(),));
}
