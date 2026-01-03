mod attack_distance;
mod hit_hurt;

pub use attack_distance::*;
pub use hit_hurt::*;

use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Unit,         // Layer 1
    Enemy,        // Layer 2
    UnitHitbox,   // Layer 3 攻击框
    EnemyHurtbox, // Layer 4 受击框
}

impl GameLayer {
    pub fn default_layers() -> CollisionLayers {
        CollisionLayers::new(
            GameLayer::Default,
            [GameLayer::Default, GameLayer::Enemy, GameLayer::Unit],
        )
    }

    pub fn enemy_layers() -> CollisionLayers {
        CollisionLayers::new(
            GameLayer::Enemy,
            [GameLayer::Default, GameLayer::Enemy, GameLayer::Unit],
        )
    }

    pub fn unit_layers() -> CollisionLayers {
        CollisionLayers::new(
            GameLayer::Unit,
            [GameLayer::Default, GameLayer::Unit, GameLayer::Enemy],
        )
    }

    pub fn unit_attack_distance_layers() -> CollisionLayers {
        CollisionLayers::new(GameLayer::Unit, [GameLayer::Enemy])
    }

    pub fn unit_hitbox_layers() -> CollisionLayers {
        CollisionLayers::new(GameLayer::UnitHitbox, [GameLayer::EnemyHurtbox])
    }
}
