use crate::common::{GameLayer, Stas, spawn_hurt};
use avian2d::prelude::*;
use bevy::prelude::*;

pub fn spawn_enemy(
    commands: &mut EntityCommands,
    asset_server: &AssetServer,
    position: Vec3,
    name: Name,
) {
    let image = asset_server.load("images/enemy/square.png");

    let collider = Collider::rectangle(80.0, 80.0);

    let parent = commands.id();

    let mut commands = commands.commands();

    let mut entity_commands = commands.spawn((
        Enemy,
        Square,
        Sprite {
            image,
            custom_size: Some(Vec2::splat(80.0)),
            ..default()
        },
        RigidBody::Kinematic,
        collider.clone(),
        LinearVelocity(Vec2::new(-10.0, 10.0)),
        GameLayer::enemy_layers(),
        Transform {
            translation: position,
            ..default()
        },
        Stas::default(),
        name,
    ));

    let enemy = entity_commands.id();

    spawn_hurt(
        &mut entity_commands,
        collider,
        GameLayer::enemy_hurtbox_layers(),
    );

    commands.entity(parent).add_child(enemy);
}

#[derive(Debug, Component)]
pub struct Enemy;

#[derive(Debug, Component)]
pub struct Square;

pub(super) fn plugin(_app: &mut App) {}
