use bevy::prelude::*;

pub fn spawn_enemy(commands: &mut Commands, asset_server: &AssetServer) {
    let image = asset_server.load("images/enemy/square.png");

    commands.spawn((Enemy, Square, Sprite { image, ..default() }));
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Component)]
pub struct Square;

pub(super) fn plugin(_app: &mut App) {}
