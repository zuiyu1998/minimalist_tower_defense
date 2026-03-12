use bevy::prelude::*;
use bevy_state_chart::{StateChart, StateChartPlugin, StateChartSets};

use crate::{
    battle::{self, BulletContext},
    common::{EnemyTargets, GameLayer, spawn_attack_distance},
    enemy::Enemy,
    skill::{Skill, SkillRunContextData, SkillRunContextDataBuilder},
    unit::{CooldownTimer, EnableState, IdleState, Unit, UnitData, UnitFactory},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArrowTowerStateEvent {
    Enable,
}

#[derive(Debug, Component)]
pub struct CooldownTimerFinished;

fn idle_2_enable_on_enable_event(
    mut commands: Commands,
    idle_q: Query<(Entity, &StateChart<ArrowTowerStateEvent>), (With<IdleState>, With<ArrowTower>)>,
) {
    for (entity, chart) in idle_q.iter() {
        if chart.events().contains(&ArrowTowerStateEvent::Enable) {
            commands
                .entity(entity)
                .remove::<IdleState>()
                .insert(EnableState);
        }
    }
}

#[derive(Debug)]
pub struct ArrowTowerFactory;

impl UnitFactory for ArrowTowerFactory {
    fn spawn(&self, _data: &UnitData, entity_commands: &mut EntityCommands) {
        let mut state_chart = StateChart::<ArrowTowerStateEvent>::default();
        state_chart.send_event(ArrowTowerStateEvent::Enable);

        entity_commands.insert((state_chart, IdleState, ArrowTower, Name::new("ArrowTower")));

        let mut command = entity_commands.commands();

        let unit_attack_distance_layers = GameLayer::unit_attack_distance_layers();

        let unit_attack_distance =
            spawn_attack_distance(&mut command, 500.0, unit_attack_distance_layers);

        entity_commands.add_child(unit_attack_distance);
    }
}

#[derive(Debug, Component)]
pub struct ArrowTower;

fn on_enable_enter(
    mut commands: Commands,
    enable_q: Query<(Entity, &Unit), (Added<EnableState>, With<ArrowTower>)>,
) {
    for (entity, unit) in enable_q.iter() {
        commands
            .entity(entity)
            .insert(CooldownTimer::new(unit.cooldown_timer));
    }
}

//更新
fn process(
    mut commands: Commands,
    mut cooldown_timer_q: Query<
        (
            &mut Unit,
            &Skill,
            Entity,
            &EnemyTargets,
            &GlobalTransform,
            &mut CooldownTimer,
        ),
        (
            With<EnableState>,
            With<ArrowTower>,
            With<CooldownTimerFinished>,
        ),
    >,
    enemy_q: Query<&GlobalTransform, With<Enemy>>,
) {
    for (mut _unit, skill, entity, enemy_targets, unit_position, mut cooldown_timer) in
        cooldown_timer_q.iter_mut()
    {
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

        cooldown_timer.0.reset();
        commands.entity(entity).remove::<CooldownTimerFinished>();
    }
}

//更新
fn on_cooldown_timer_finished(
    mut commands: Commands,
    mut cooldown_timer_q: Query<
        (Entity, &CooldownTimer),
        (
            With<EnableState>,
            With<ArrowTower>,
            Without<CooldownTimerFinished>,
        ),
    >,
) {
    for (entity, cooldown_timer) in cooldown_timer_q.iter_mut() {
        if cooldown_timer.0.just_finished() {
            commands.entity(entity).insert(CooldownTimerFinished);
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(StateChartPlugin::<ArrowTowerStateEvent>::default());

    app.add_systems(
        FixedUpdate,
        idle_2_enable_on_enable_event.in_set(StateChartSets::StateTransition),
    );

    app.add_systems(
        FixedUpdate,
        (
            (on_cooldown_timer_finished, process).chain(),
            on_enable_enter,
        )
            .in_set(StateChartSets::Action),
    );
}
