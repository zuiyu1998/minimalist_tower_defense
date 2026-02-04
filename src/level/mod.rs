mod unit_data_collection;

pub use unit_data_collection::*;

use bevy::prelude::*;

use crate::{
    common::{Sunlight, SunlightProductProcessor}, map::{MapData, spawn_map}, navigator::spawn_nav_mesh, player::Player, product::{ProductMeta, ProductSystem}, screens::Screen, unit::UnitFactoryContainer
};

#[derive(Debug, Component)]
pub struct SunlightText;

#[derive(Debug, Default)]
pub struct LevelCollection {
    sunlight: u32,
}

pub fn update_level_collection_panel(
    level: Res<Level>,
    sunlight: Single<&mut Text, With<SunlightText>>,
) {
    let mut sunlight = sunlight.into_inner();

    tracing::info!(
        "Collected sunlight products: {:?}",
        level.collection.sunlight
    );

    sunlight.0 = format!("{}", level.collection.sunlight);
}

pub fn spawn_level_collection_panel(commands: &mut Commands, level_collection: &LevelCollection) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            align_items: AlignItems::Start,
            justify_content: JustifyContent::Start,
            ..default()
        },
        ZIndex(1),
        Name::new("LevelCollection"),
        DespawnOnExit(Screen::Gameplay),
        children![
            (
                Node {
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                children![
                    Text::new("Sunlight: "),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                ],
            ),
            (
                Node {
                    align_items: AlignItems::Start,
                    justify_content: JustifyContent::Start,
                    ..default()
                },
                children![
                    (
                        Text::new(format!("{}", level_collection.sunlight)),
                        SunlightText,
                        Name::new("SunlightText"),
                    ),
                    TextFont {
                        font_size: 32.0,
                        ..default()
                    },
                ],
            )
        ],
    ));
}

#[derive(Debug, Resource)]
pub struct Level {
    product_system: ProductSystem,
    collection: LevelCollection,
}

impl Level {
    pub fn collect_product(&mut self, reader: &mut MessageReader<ProductMeta>) {
        let mut sunlight = 0;

        for product_meta in reader.read() {
            if let Some(product) = self.product_system.create::<Sunlight>(product_meta) {
                sunlight += product.0;
            } else {
                tracing::warn!(
                    "No processor found for product: {:?}, skipping.",
                    product_meta
                );
            }
        }

        if sunlight == 0 {
            return;
        }

        self.collection.sunlight += sunlight;
    }
}

impl Default for Level {
    fn default() -> Self {
        let mut product_system = ProductSystem::empty();
        product_system.register_processor("sunlight", SunlightProductProcessor);

        Level {
            product_system,
            collection: LevelCollection::default(),
        }
    }
}

fn collect_product(mut level: ResMut<Level>, mut reader: MessageReader<ProductMeta>) {
    level.collect_product(&mut reader);
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Level>();

    app.add_plugins(unit_data_collection::plugin);
    app.add_systems(
        Update,
        (collect_product, update_level_collection_panel).run_if(in_state(Screen::Gameplay)),
    );
}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_data: Res<MapData>,
    unit_factory_container: Res<UnitFactoryContainer>,
    collection: Res<UnitDataCollection>,
    level: Res<Level>,
) {
    commands.spawn(Player);

    spawn_map(
        &mut commands,
        &asset_server,
        &map_data,
        &unit_factory_container,
    );

    spawn_nav_mesh(&mut commands);

    spawn_unit_data_collection_panel(&mut commands, &asset_server, &collection);

    spawn_level_collection_panel(&mut commands, &level.collection);
}
