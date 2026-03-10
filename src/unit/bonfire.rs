use bevy::prelude::*;

use crate::{
    common::LightSource,
    product::ProductMeta,
    unit::{CooldownTimer, Unit, UnitFactory},
};

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct IdleState;

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct EnableState;

#[derive(Debug, PartialEq, Eq)]
pub enum StateChartEvent {
    Enable,
}

#[derive(Debug, Component)]
#[component(storage = "SparseSet")]
pub struct StateChart {
    events: Vec<StateChartEvent>,
}

fn idle_2_enable_on_enable_event(
    mut commands: Commands,
    idle_q: Query<(Entity, &StateChart), With<IdleState>>,
) {
    for (entity, chart) in idle_q.iter() {
        if chart.events.contains(&StateChartEvent::Enable) {
            commands
                .entity(entity)
                .remove::<IdleState>()
                .insert(EnableState);
        }
    }
}

fn on_enable_enter(mut commands: Commands, enable_q: Query<(Entity, &Unit), Added<EnableState>>) {
    for (entity, unit) in enable_q.iter() {
        commands
            .entity(entity)
            .insert(CooldownTimer::new(unit.cooldown_timer));
    }
}

fn on_cooldown_timer_finished(
    mut cooldown_timer_q: Query<(&mut CooldownTimer, &Bonfire, &mut Unit), With<EnableState>>,
    mut writer: MessageWriter<ProductMeta>,
) {
    for (mut cooldown_timer, bonfire, mut _unit) in cooldown_timer_q.iter_mut() {
        if cooldown_timer.0.just_finished() {
            tracing::info!("Products is generated.");
            writer.write_batch(bonfire.products.iter().cloned());

            cooldown_timer.0.reset();
        }
    }
}

#[derive(Debug, Component)]
pub struct Bonfire {
    products: Vec<ProductMeta>,
}

impl Default for Bonfire {
    fn default() -> Self {
        Bonfire {
            products: vec![ProductMeta {
                name: "sunlight".to_string(),
                value: 10.0,
            }],
        }
    }
}

#[derive(Debug)]
pub struct BonfireFactory;

impl UnitFactory for BonfireFactory {
    fn spawn(&self, _data: &super::UnitData, commands: &mut EntityCommands) {
        commands.insert((
            Bonfire::default(),
            LightSource,
            Name::new("Bonfire"),
            IdleState,
            StateChart {
                events: vec![StateChartEvent::Enable],
            },
        ));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        FixedUpdate,
        (
            on_cooldown_timer_finished,
            idle_2_enable_on_enable_event,
            on_enable_enter,
        )
            .chain(),
    );
}
