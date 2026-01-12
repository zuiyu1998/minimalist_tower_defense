use bevy::prelude::*;

use crate::{
    map::{MapData, MapItemData, MapItemFactoryContainer, MapState, spawn_map},
    player::Player,
    screens::Screen,
    unit::{UnitData, UnitFactoryContainer},
};

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

fn on_unit_data_button(event: On<Pointer<Over>>) {
    println!("I am being hovered over");
}

fn button(asset_server: &AssetServer, unit_data: &UnitData) -> impl Bundle {
    let image: ImageNode = unit_data.get_unit_image(asset_server).into();

    (
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
        Pickable::IGNORE,
        children![(
            Node {
                width: px(64),
                height: px(64),
                ..default()
            },
            Observer::new(on_unit_data_button),
            Pickable::default(),
        )],
    )
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
            Pickable::IGNORE,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    Node {
                        width: px(150),
                        height: percent(100),
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    Pickable::IGNORE,
                ))
                .with_children(|parent| {
                    for data in collection.items.iter() {
                        parent.spawn(button(asset_server, data));
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

    app.add_systems(
        Update,
        unit_data_button_system.run_if(in_state(Screen::Gameplay)),
    );
}

pub fn spawn_level(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    map_data: Res<MapData>,
    container: Res<MapItemFactoryContainer>,
    unit_factory_container: Res<UnitFactoryContainer>,
    collection: Res<UnitDataCollection>,
) {
    commands.spawn(Player);

    spawn_map(
        &mut commands,
        &asset_server,
        &map_data,
        &container,
        &unit_factory_container,
    );

    spawn_unit_data_collection_panel(&mut commands, &asset_server, &collection);
}
