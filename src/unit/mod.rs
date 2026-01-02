use bevy::prelude::*;

pub fn spawn_unit(commands: &mut Commands, asset_server: &AssetServer) {
    let image = asset_server.load("images/unit/TemporaryArrowTower.png");

    let position = Vec3::new(-100.0, 100.0, 100.0);

    commands.spawn((
        Unit,
        Sprite {
            image,
            custom_size: Some(Vec2::splat(100.0)),
            ..default()
        },
        Transform {
            translation: position,
            ..default()
        },
    ));
}

#[derive(Debug, Component)]
pub struct Unit;

pub(super) fn plugin(_app: &mut App) {}
