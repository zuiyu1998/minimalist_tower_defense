use avian2d::prelude::*;
use bevy::prelude::*;

use crate::common::{GameLayer, spawn_hit};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, on_bullet_attack);
}

fn on_bullet_attack(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    bullet_q: Query<(&Bullet, Entity)>,
) {
    for event in collision_reader.read() {
        if event.body1.is_none() {
            continue;
        }
        let body1 = event.body1.clone().unwrap();

        if let Ok((_bullet, entity)) = bullet_q.get(body1) {
            tracing::info!("bullet attack start");

            commands.entity(entity).despawn();
        }
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
    let image = asset_server.load("images/bullet/ball.png");

    let collider = Collider::circle(3.0);

    let mut commands = commands.spawn((
        Bullet,
        Sprite { image, ..default() },
        RigidBody::Kinematic,
        collider.clone(),
        GameLayer::default_layers(),
        LinearVelocity(direction * 100.0 * 1.0),
        Transform {
            translation: Vec3::new(bullet_position.x, bullet_position.y, 0.0),
            ..default()
        },
    ));

    spawn_hit(&mut commands, collider, layers);
}
