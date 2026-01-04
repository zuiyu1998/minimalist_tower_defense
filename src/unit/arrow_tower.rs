use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    battle::{self, BulletContext},
    common::{AttackDistance, GameLayer},
    enemy::Enemy,
    skill::{Skill, SkillRunContextData, SkillRunContextDataBuilder},
    unit::{CooldownTimer, EnemyTargets},
};

#[derive(Debug, Component)]
pub struct ArrowTower;

//检测敌人是否存在
fn check_enemy_targets(
    mut enemy_targets_q: Query<&mut EnemyTargets, With<ArrowTower>>,
    enemy_q: Query<Entity, With<Enemy>>,
) {
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

//更新
fn on_cooldown_timer_update(
    mut commands: Commands,
    mut cooldown_timer_q: Query<
        (
            &mut CooldownTimer,
            &Skill,
            Entity,
            &EnemyTargets,
            &GlobalTransform,
        ),
        With<ArrowTower>,
    >,
    enemy_q: Query<&GlobalTransform, With<Enemy>>,
    time: Res<Time>,
) {
    for (mut cooldown_timer, skill, entity, enemy_targets, unit_position) in
        cooldown_timer_q.iter_mut()
    {
        cooldown_timer.0.tick(time.delta());

        if cooldown_timer.0.just_finished() {
            if enemy_targets.0.is_empty() {
                return;
            }

            let target = *enemy_targets.0.first().unwrap();
            let target_position = enemy_q.get(target).unwrap();

            tracing::debug!("target_position {}", target_position.translation());
            tracing::debug!("unit_position {}", unit_position.translation());

            let direction =
                target_position.translation().truncate() - unit_position.translation().truncate();
            let direction = direction.normalize();

            let context = BulletContext {
                layers: GameLayer::unit_hitbox_layers(),
                direction: direction,
                bullet_position: unit_position.translation().truncate(),
            };

            let mut data = SkillRunContextData::default();

            context.update_skill_run_context_data(&mut data);

            tracing::debug!("Skill start.");
            battle::execute_skill(&mut commands, skill, entity, vec![target], None, data);
        }
    }
}

//获取敌人信息
fn find_enemy(
    mut collision_reader: MessageReader<CollisionStart>,
    attack_distance_q: Query<&ChildOf, With<AttackDistance>>,
    mut enemy_targets_q: Query<&mut EnemyTargets>,
) {
    for event in collision_reader.read() {
        if attack_distance_q.contains(event.collider1) {
            if let Some(enemy) = event.body2 {
                let attack_distance = attack_distance_q.get(event.collider1).unwrap();
                let mut enemy_targets = enemy_targets_q.get_mut(attack_distance.0).unwrap();

                enemy_targets.0.push(enemy);
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (check_enemy_targets, find_enemy).chain());
    app.add_systems(FixedUpdate, (on_cooldown_timer_update,).chain());
}
