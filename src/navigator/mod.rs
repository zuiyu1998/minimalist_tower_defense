use avian2d::prelude::*;
use bevy::prelude::*;
use vleue_navigator::{
    Triangulation, VleueNavigatorPlugin,
    prelude::{NavMeshSettings, NavMeshUpdateMode, NavmeshUpdaterPlugin},
};

use crate::consts::MAP_TIEM_SIZE;

#[derive(Component)]
pub enum Obstacle {
    Wall,
}

#[derive(Component, Default)]
pub struct NavigatorPath {
    pub current: Vec3,
    pub next: Vec<Vec3>,
}

pub fn spawn_nav_mesh(commands: &mut Commands) {
    commands.spawn((
        NavMeshSettings {
            // Define the outer borders of the navmesh.
            fixed: Triangulation::from_outer_edges(&[
                vec2(-500.0, -500.0),
                vec2(500.0, -500.0),
                vec2(500.0, 500.0),
                vec2(-500.0, 500.0),
            ]),
            agent_radius: MAP_TIEM_SIZE / 2.0,
            simplify: 10.0,
            merge_steps: 1,
            ..default()
        },
        NavMeshUpdateMode::Direct,
    ));
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        VleueNavigatorPlugin,
        NavmeshUpdaterPlugin::<Collider, Obstacle>::default(),
    ));
}
