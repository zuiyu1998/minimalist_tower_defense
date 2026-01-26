use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{
    common::{Sunlight, SunlightProductProcessor},
    map::{MapData, MapItemData, MapState, spawn_map},
    player::Player,
    product::{ProductMeta, ProductSystem},
    screens::Screen,
    unit::{UnitData, UnitFactoryContainer},
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

#[derive(Debug, Component)]
pub struct UnitDataButton {
    pub unit_data: UnitData,
}

fn unit_data_button_system(
    button_q: Query<(&UnitDataButton, &Interaction), Changed<Interaction>>,
    mut map_data: ResMut<MapState>,
) {
    for (button, interaction) in button_q.iter() {
        match *interaction {
            Interaction::Pressed => {
                map_data.selelcted_map_item_data =
                    Some(MapItemData::from_unit_data(&button.unit_data));
            }
            _ => {}
        }
    }
}

fn on_unit_data_button_out(_event: On<Pointer<Out>>, mut map_state: ResMut<MapState>) {
    map_state.enable = true;
}

fn on_unit_data_button_over(_event: On<Pointer<Over>>, mut map_state: ResMut<MapState>) {
    map_state.enable = false;
}

fn unit_data_button(
    commands: &mut RelatedSpawnerCommands<ChildOf>,
    asset_server: &AssetServer,
    unit_data: &UnitData,
) {
    let image: ImageNode = unit_data.get_unit_image(asset_server).into();

    commands
        .spawn((
            Node {
                width: px(64),
                height: px(64),
                ..default()
            },
            image,
            Button,
            UnitDataButton {
                unit_data: unit_data.clone(),
            },
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: px(64),
                    height: px(64),
                    ..default()
                })
                .observe(on_unit_data_button_out)
                .observe(on_unit_data_button_over);
        });
}

pub fn spawn_unit_data_collection_panel(
    commands: &mut Commands,
    asset_server: &AssetServer,
    collection: &UnitDataCollection,
) {
    commands
        .spawn((
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Start,
                ..default()
            },
            Name::new("UnitDataCollectionPanel"),
            DespawnOnExit(Screen::Gameplay),
            ZIndex(10),
        ))
        .with_children(|parent| {
            parent
                .spawn((Node {
                    width: px(150),
                    height: percent(100),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },))
                .with_children(|parent| {
                    for data in collection.items.iter() {
                        unit_data_button(parent, asset_server, data);
                    }
                });
        });
}

#[derive(Debug, Resource)]
pub struct UnitDataCollection {
    items: Vec<UnitData>,
}

impl Default for UnitDataCollection {
    fn default() -> Self {
        let mut items = vec![];

        items.push(UnitData {
            item_name: "arrow_tower".to_string(),
            image: "TemporaryArrowTower".to_string(),
        });

        UnitDataCollection { items }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<UnitDataCollection>();
    app.init_resource::<Level>();

    app.add_systems(
        Update,
        (
            unit_data_button_system,
            collect_product,
            update_level_collection_panel,
        )
            .run_if(in_state(Screen::Gameplay)),
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

    spawn_unit_data_collection_panel(&mut commands, &asset_server, &collection);

    spawn_level_collection_panel(&mut commands, &level.collection);
}
