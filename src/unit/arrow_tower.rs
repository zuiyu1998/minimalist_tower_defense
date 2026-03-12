use bevy::prelude::*;

use crate::{
    battle::{self, BulletContext},
    common::{EnemyTargets, GameLayer, spawn_attack_distance},
    enemy::Enemy,
    skill::{Skill, SkillRunContextData, SkillRunContextDataBuilder},
    unit::{CooldownTimer, Unit, UnitData, UnitFactory},
};

#[derive(Debug)]
pub struct ArrowTowerFactory;

impl UnitFactory for ArrowTowerFactory {
    fn spawn(&self, _data: &UnitData, entity_commands: &mut EntityCommands) {
        entity_commands.insert((ArrowTower, Name::new("ArrowTower")));

        let mut command = entity_commands.commands();

        let unit_attack_distance_layers = GameLayer::unit_attack_distance_layers();

        let unit_attack_distance =
            spawn_attack_distance(&mut command, 500.0, unit_attack_distance_layers);

        entity_commands.add_child(unit_attack_distance);
    }
}

#[derive(Debug, Component)]
pub struct ArrowTower;

//更新
fn process(
    mut commands: Commands,
    mut cooldown_timer_q: Query<
        (&mut Unit, &Skill, Entity, &EnemyTargets, &GlobalTransform),
        With<ArrowTower>,
    >,
    enemy_q: Query<&GlobalTransform, With<Enemy>>,
) {
    for (mut unit, skill, entity, enemy_targets, unit_position) in cooldown_timer_q.iter_mut() {
        if !unit.cooling_down {
            if enemy_targets.0.is_empty() {
                return;
            }

            let target = *enemy_targets.0.first().unwrap();
            let Ok(target_position) = enemy_q.get(target) else {
                return;
            };

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

            unit.cooling_down = true;
            commands
                .entity(entity)
                .insert(CooldownTimer::new(unit.cooldown_timer));
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, (process,).chain());
}
