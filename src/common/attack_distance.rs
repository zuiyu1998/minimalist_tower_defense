use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;

#[derive(Debug, Component, Default)]
pub struct EnemyTargets(pub Vec<Entity>);

#[derive(Debug, Component)]
pub struct AttackDistance;

pub fn spawn_attack_distance(
    commmads: &mut Commands,
    circle: f32,
    collision_layers: CollisionLayers,
) -> Entity {
    commmads
        .spawn((
            Collider::circle(circle),
            Sensor,
            AttackDistance,
            collision_layers,
            CollisionEventsEnabled,
        ))
        .id()
}

//检测敌人是否存在
fn check_enemy_targets(mut enemy_targets_q: Query<&mut EnemyTargets>, enemy_q: Query<Entity>) {
    for mut enemy_targets in enemy_targets_q.iter_mut() {
        let mut new_enemy_targets = vec![];

        for enemy_target in enemy_targets.0.iter() {
            if let Ok(entity) = enemy_q.get(*enemy_target) {
                new_enemy_targets.push(entity);
            }
        }

        enemy_targets.0 = new_enemy_targets;
    }
}

fn on_ememy_enter(
    attack_distance_q: &Query<&ChildOf, With<AttackDistance>>,
    enemy_targets_q: &mut Query<&mut EnemyTargets>,
    attack_distance_entity: Entity,
    enemy_target: Entity,
) {
    if attack_distance_q.contains(attack_distance_entity) {
        let attack_distance = attack_distance_q.get(attack_distance_entity).unwrap();
        let mut enemy_targets = enemy_targets_q.get_mut(attack_distance.0).unwrap();

        tracing::debug!(
            "{} entity add {} enemy_target",
            attack_distance.0,
            enemy_target
        );

        enemy_targets.0.push(enemy_target);
    }
}

fn on_ememy_exit(
    attack_distance_q: &Query<&ChildOf, With<AttackDistance>>,
    enemy_targets_q: &mut Query<&mut EnemyTargets>,
    attack_distance_entity: Entity,
    enemy_target: Entity,
) {
    if attack_distance_q.contains(attack_distance_entity) {
        let attack_distance = attack_distance_q.get(attack_distance_entity).unwrap();
        let mut enemy_targets = enemy_targets_q.get_mut(attack_distance.0).unwrap();

        tracing::debug!(
            "{} entity remove {} enemy_target",
            attack_distance.0,
            enemy_target
        );

        enemy_targets.0 = enemy_targets
            .0
            .iter()
            .copied()
            .filter(|entity| *entity != enemy_target)
            .collect();
    }
}

//记录敌人信息
pub fn record_enemy(
    mut collision_start_reader: MessageReader<CollisionStart>,
    mut collision_end_reader: MessageReader<CollisionEnd>,
    attack_distance_q: Query<&ChildOf, With<AttackDistance>>,
    mut enemy_targets_q: Query<&mut EnemyTargets>,
) {
    //进入的敌人
    for event in collision_start_reader.read() {
        if event.body1.is_none() && event.body2.is_none() {
            continue;
        }

        on_ememy_enter(
            &attack_distance_q,
            &mut enemy_targets_q,
            event.collider1,
            event.body2.unwrap(),
        );

        on_ememy_enter(
            &attack_distance_q,
            &mut enemy_targets_q,
            event.collider2,
            event.body1.unwrap(),
        );
    }

    //退出的敌人
    for event in collision_end_reader.read() {
        if event.body1.is_none() && event.body2.is_none() {
            continue;
        }

        on_ememy_exit(
            &attack_distance_q,
            &mut enemy_targets_q,
            event.collider1,
            event.body2.unwrap(),
        );

        on_ememy_exit(
            &attack_distance_q,
            &mut enemy_targets_q,
            event.collider2,
            event.body1.unwrap(),
        );
    }
}

#[derive(Debug, SystemSet, PartialEq, Eq, Hash, Clone)]
pub enum AttackDistanceSets {
    Actions,
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (check_enemy_targets, record_enemy)
            .chain()
            .in_set(AttackDistanceSets::Actions)
            .run_if(in_state(Screen::Gameplay)),
    );
}
