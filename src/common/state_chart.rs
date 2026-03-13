use std::marker::PhantomData;

use bevy::prelude::*;

pub trait StateEvent: Clone + Send + Sync + 'static + PartialEq {}

impl<T: Clone + Send + Sync + 'static + PartialEq> StateEvent for T {}

#[derive(Debug, Component)]
pub struct StateChart<E: StateEvent> {
    a: Vec<E>,
    b: Vec<E>,
    flag: bool,
}

impl<E: StateEvent> Default for StateChart<E> {
    fn default() -> Self {
        Self {
            a: vec![],
            b: vec![],
            flag: true,
        }
    }
}

impl<E: StateEvent> StateChart<E> {
    pub fn state_transition_finished(mut chart_q: Query<&mut StateChart<E>>) {
        for mut chart in chart_q.iter_mut() {
            chart.swap();
        }
    }

    pub fn events(&self) -> &[E] {
        if self.flag {
            return &self.a;
        } else {
            return &self.b;
        }
    }

    pub fn swap(&mut self) {
        if self.flag {
            self.a = vec![];
        } else {
            self.b = vec![];
        }

        self.flag = !self.flag;
    }

    pub fn send_event(&mut self, e: E) {
        if self.flag {
            self.a.push(e);
        } else {
            self.b.push(e);
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, SystemSet)]
pub enum StateChartSets {
    StateTransition,
    StateTransitionFinished,
    Action,
}

pub struct StateChartPlugin<E> {
    _marker: PhantomData<E>,
}

impl<E> Default for StateChartPlugin<E> {
    fn default() -> Self {
        StateChartPlugin {
            _marker: PhantomData,
        }
    }
}

impl<E: StateEvent> Plugin for StateChartPlugin<E> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PreUpdate,
            StateChart::<E>::state_transition_finished
                .in_set(StateChartSets::StateTransitionFinished),
        );
    }
}

pub struct StateChartConfigPlugin;

impl Plugin for StateChartConfigPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            PreUpdate,
            (
                StateChartSets::StateTransition,
                StateChartSets::StateTransitionFinished,
                StateChartSets::Action,
            )
                .chain(),
        );
    }
}
