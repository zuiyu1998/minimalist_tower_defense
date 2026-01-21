use bevy::prelude::*;

use crate::{common::LightSource, unit::UnitFactory};

#[derive(Debug, Component)]
pub struct Bonfire;

#[derive(Debug)]
pub struct BonfireFactory;

fn process(_bonfire_q: Query<Entity, With<Bonfire>>) {}

impl UnitFactory for BonfireFactory {
    fn spawn(&self, _data: &super::UnitData, commands: &mut EntityCommands) {
        commands.insert((Bonfire, LightSource, Name::new("Bonfire")));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, process);
}
