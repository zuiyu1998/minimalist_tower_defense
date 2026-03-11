use bevy::prelude::*;

use crate::{
    common::LightSource,
    product::ProductMeta,
    unit::{CooldownTimer, EnableState, IdleState, Unit, UnitFactory},
};

use bevy_state_chart::{StateChart, StateChartPlugin, StateChartSets};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BonfireStateEvent {
    Enable,
}

fn idle_2_enable_on_enable_event(
    mut commands: Commands,
    idle_q: Query<(Entity, &StateChart<BonfireStateEvent>), (With<IdleState>, With<Bonfire>)>,
) {
    for (entity, chart) in idle_q.iter() {
        if chart.events().contains(&BonfireStateEvent::Enable) {
            commands
                .entity(entity)
                .remove::<IdleState>()
                .insert(EnableState);
        }
    }
}

fn on_enable_enter(
    mut commands: Commands,
    enable_q: Query<(Entity, &Unit), (Added<EnableState>, With<Bonfire>)>,
) {
    for (entity, unit) in enable_q.iter() {
        commands
            .entity(entity)
            .insert(CooldownTimer::new(unit.cooldown_timer));
    }
}

fn on_cooldown_timer_finished(
    mut cooldown_timer_q: Query<
        (&mut CooldownTimer, &Bonfire, &mut Unit),
        (With<EnableState>, With<Bonfire>),
    >,
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
        let mut state_chart = StateChart::<BonfireStateEvent>::default();
        state_chart.send_event(BonfireStateEvent::Enable);

        commands.insert((
            Bonfire::default(),
            LightSource,
            Name::new("Bonfire"),
            IdleState,
            state_chart,
        ));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(StateChartPlugin::<BonfireStateEvent>::default());

    app.add_systems(
        FixedUpdate,
        idle_2_enable_on_enable_event.in_set(StateChartSets::StateTransition),
    );

    app.add_systems(
        FixedUpdate,
        (on_enable_enter, on_cooldown_timer_finished).in_set(StateChartSets::Action),
    );
}
