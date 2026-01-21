use bevy::prelude::*;

use crate::{common::LightSource, unit::UnitFactory};

#[derive(Debug, Component)]
pub struct Bonfire;

#[derive(Debug)]
pub struct BonfireFactory;

impl UnitFactory for BonfireFactory {
    fn spawn(&self, _data: &super::UnitData, commands: &mut EntityCommands) {
        commands.insert((Bonfire, LightSource, Name::new("Bonfire")));
    }
}

pub(super) fn plugin(_app: &mut App) {}
