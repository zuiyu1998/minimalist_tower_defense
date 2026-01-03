use crate::common::GameLayer;
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn spawn_enemy(commands: &mut Commands, asset_server: &AssetServer) {
    let image = asset_server.load("images/enemy/square.png");

    let position = Vec3::new(200.0, 0.0, 0.0);

    commands.spawn((
        Enemy,
        Square,
        Sprite {
            image,
            custom_size: Some(Vec2::splat(80.0)),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::rectangle(80.0, 80.0),
        LinearVelocity(Vec2::new(-10.0, 10.0)),
        GameLayer::enemy_layer(),
        Transform {
            translation: position,
            ..default()
        },
    ));
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Component)]
pub struct Square;

pub(super) fn plugin(_app: &mut App) {}
