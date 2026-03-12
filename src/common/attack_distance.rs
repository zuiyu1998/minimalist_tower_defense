use avian2d::prelude::*;
use bevy::prelude::*;

use crate::screens::Screen;

#[derive(Debug, Component, Default)]
pub struct EnemyTargets(pub Vec<Entity>);

#[derive(Debug, Component)]
pub struct AttackDistance;

pub fn spawn_attack_distance(
    commmads: &mut Commands,
    circle: f32,
    collision_layers: CollisionLayers,
) -> Entity {
    commmads
        .spawn((
            Collider::circle(circle),
            Sensor,
            AttackDistance,
            collision_layers,
            CollisionEventsEnabled,
        ))
        .id()
}

//获取敌人信息
pub fn find_enemy(
    mut collision_reader: MessageReader<CollisionStart>,
    attack_distance_q: Query<&ChildOf, With<AttackDistance>>,
    mut enemy_targets_q: Query<&mut EnemyTargets>,
) {
    for event in collision_reader.read() {
        if attack_distance_q.contains(event.collider1) {
            if let Some(enemy) = event.body2 {
                let attack_distance = attack_distance_q.get(event.collider1).unwrap();
                let mut enemy_targets = enemy_targets_q.get_mut(attack_distance.0).unwrap();

                enemy_targets.0.push(enemy);
            }
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (find_enemy).run_if(in_state(Screen::Gameplay)));
}
