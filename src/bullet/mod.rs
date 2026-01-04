use avian2d::prelude::*;
use bevy::prelude::*;

use crate::common::{GameLayer, spawn_hit};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, on_bullet_attack);
}

fn on_bullet_attack(mut collision_reader: MessageReader<CollisionStart>) {
    for event in collision_reader.read() {
        println!("event {} {}", event.collider1, event.collider2);
    }
}

#[derive(Debug, Component)]
pub struct Bullet;

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &AssetServer,
    layers: CollisionLayers,
    direction: Vec2,
    bullet_position: Vec2,
) {
    tracing::info!("bullet direction: {}", direction);
    let image = asset_server.load("images/bullet/ball.png");

    let collider = Collider::circle(3.0);

    let mut commands = commands.spawn((
        Bullet,
        Sprite { image, ..default() },
        RigidBody::Kinematic,
        collider.clone(),
        GameLayer::default_layers(),
        LinearVelocity(direction * 40.0 * 1.0),
        Transform {
            translation: Vec3::new(bullet_position.x, bullet_position.y, 0.0),
            ..default()
        },
    ));

    spawn_hit(&mut commands, collider, layers);
}
