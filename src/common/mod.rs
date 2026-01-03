mod attack_distance;

pub use attack_distance::*;
use avian2d::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default, // Layer 0 - the default layer that objects are assigned to
    Unit,  // Layer 1
    Enemy, // Layer 2
}

impl GameLayer {
    pub fn enemy_layer() -> CollisionLayers {
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
}
