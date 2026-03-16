use avian2d::prelude::*;
use bevy::{platform::collections::HashSet, prelude::*};

use crate::common::{GameLayer, Stas, spawn_hit};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (bullet_attack, queue_free).chain());
}

#[derive(Debug, Component)]
pub struct StartPosition(Vec2);

//子弹运行一定距离后自动删除
fn queue_free(
    mut commands: Commands,
    bullet_q: Query<(&Transform, Entity, &StartPosition), With<Bullet>>,
) {
    for (transform, entity, start) in bullet_q.iter() {
        let distance = transform.translation.truncate().distance(start.0);

        if distance > 1000.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn on_bullet_attack(
    stats_q: &mut Query<&mut Stas>,
    bullet_q: &Query<(&Bullet, Entity)>,
    bullet_entity: Entity,
    stats_entity: Entity,
    die_set: &mut HashSet<Entity>,
) {
    if let Ok((_bullet, entity)) = bullet_q.get(bullet_entity) {
        tracing::info!("bullet attack start");

        if let Ok(mut stats) = stats_q.get_mut(stats_entity) {
            stats.update_health(-5);
            if stats.is_die() {
                die_set.insert(stats_entity);
            }
        }
        die_set.insert(entity);
    }
}

fn bullet_attack(
    mut commands: Commands,
    mut collision_reader: MessageReader<CollisionStart>,
    bullet_q: Query<(&Bullet, Entity)>,
    mut stats_q: Query<&mut Stas>,
) {
    let mut die_set = HashSet::new();

    for event in collision_reader.read() {
        if event.body1.is_none() || event.body2.is_none() {
            continue;
        }
        let body1 = event.body1.clone().unwrap();
        let body2 = event.body2.clone().unwrap();

        on_bullet_attack(&mut stats_q, &bullet_q, body1, body2, &mut die_set);
        on_bullet_attack(&mut stats_q, &bullet_q, body2, body1, &mut die_set);
    }

    for entity in die_set.iter() {
        commands.entity(*entity).despawn();
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
        LinearVelocity(direction * 400.0 * 1.0),
        Transform {
            translation: Vec3::new(bullet_position.x, bullet_position.y, 0.0),
            ..default()
        },
        StartPosition(bullet_position),
    ));

    spawn_hit(&mut commands, collider, layers);
}
