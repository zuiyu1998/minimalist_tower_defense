use bevy::prelude::*;

use crate::{
    battle::{self, BulletContext},
    common::{
        EnemyTargets, GameLayer, StateChart, StateChartPlugin, StateChartSets,
        spawn_attack_distance,
    },
    enemy::Enemy,
    skill::{Skill, SkillRunContextData, SkillRunContextDataBuilder},
    unit::{CooldownTimer, EnableState, IdleState, Unit, UnitData, UnitFactory},
};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ArrowTowerStateEvent {
    Enable,
    Active,
}

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct ActiveState;

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

fn enable_2_active_on_enable_event(
    mut commands: Commands,
    idle_q: Query<
        (Entity, &StateChart<ArrowTowerStateEvent>),
        (With<EnableState>, With<ArrowTower>),
    >,
) {
    for (entity, chart) in idle_q.iter() {
        if chart.events().contains(&ArrowTowerStateEvent::Active) {
            commands
                .entity(entity)
                .remove::<EnableState>()
                .insert(ActiveState);
        }
    }
}

fn active_2_enable_on_enable_event(
    mut commands: Commands,
    idle_q: Query<
        (Entity, &StateChart<ArrowTowerStateEvent>),
        (With<ActiveState>, With<ArrowTower>),
    >,
) {
    for (entity, chart) in idle_q.iter() {
        if chart.events().contains(&ArrowTowerStateEvent::Enable) {
            commands
                .entity(entity)
                .remove::<ActiveState>()
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
fn on_actvie_update(
    mut commands: Commands,
    mut arrow_tower_q: Query<
        (
            &mut Unit,
            &Skill,
            Entity,
            &EnemyTargets,
            &GlobalTransform,
            &mut StateChart<ArrowTowerStateEvent>,
        ),
        (With<ActiveState>, With<ArrowTower>),
    >,
    enemy_q: Query<&GlobalTransform, With<Enemy>>,
) {
    for (mut _unit, skill, entity, enemy_targets, unit_position, mut start_chart) in
        arrow_tower_q.iter_mut()
    {
        if enemy_targets.0.is_empty() {
            return;
        }

        start_chart.send_event(ArrowTowerStateEvent::Enable);
        commands.entity(entity).remove::<CooldownTimer>();

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
    }
}

//更新
fn on_enable_update(
    mut commands: Commands,
    mut cooldown_timer_q: Query<
        (
            Entity,
            &CooldownTimer,
            &mut StateChart<ArrowTowerStateEvent>,
        ),
        (With<EnableState>, With<ArrowTower>),
    >,
) {
    for (entity, cooldown_timer, mut start_chart) in cooldown_timer_q.iter_mut() {
        if cooldown_timer.0.just_finished() {
            start_chart.send_event(ArrowTowerStateEvent::Active);
            commands.entity(entity).remove::<CooldownTimer>();
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(StateChartPlugin::<ArrowTowerStateEvent>::default());

    app.add_systems(
        PreUpdate,
        (
            idle_2_enable_on_enable_event,
            enable_2_active_on_enable_event,
            active_2_enable_on_enable_event,
        )
            .in_set(StateChartSets::StateTransition),
    );

    app.add_systems(
        PreUpdate,
        (on_enable_update, on_actvie_update, on_enable_enter).in_set(StateChartSets::Action),
    );
}
