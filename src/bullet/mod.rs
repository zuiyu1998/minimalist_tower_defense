use avian2d::prelude::*;
use bevy::prelude::*;

use crate::common::{GameLayer, Stas, spawn_hit};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, on_bullet_attack);
}

fn apply(
    commands: &mut Commands,
    stats_q: &mut Query<&mut Stas>,
    bullet_q: &Query<(&Bullet, Entity)>,
    bullet_entity: Entity,
    stats_entity: Entity,
) {
    if let Ok((_bullet, entity)) = bullet_q.get(bullet_entity) {
        tracing::info!("bullet attack start");

        if let Ok(mut stats) = stats_q.get_mut(stats_entity) {
            stats.update_health(-5);
            if stats.is_die() {
                commands.entity(stats_entity).despawn();
            }
        }

        commands.entity(entity).despawn();
    }
}

fn on_bullet_attack(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    bullet_q: Query<(&Bullet, Entity)>,
    mut stats_q: Query<&mut Stas>,
) {
    for event in collision_reader.read() {
        if event.body1.is_none() || event.body2.is_none() {
            continue;
        }
        let body1 = event.body1.clone().unwrap();
        let body2 = event.body2.clone().unwrap();

        apply(&mut commands, &mut stats_q, &bullet_q, body1, body2);
        apply(&mut commands, &mut stats_q, &bullet_q, body2, body1);
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
