use std::time::Duration;

use bevy::{ecs::relationship::RelatedSpawnerCommands, prelude::*};

use crate::{
    map::{MapItemData, MapState},
    screens::Screen,
    unit::UnitData,
};

#[derive(Debug, Component)]
pub struct UsedCooldownTimer(Timer);

#[derive(Debug, Component)]
pub struct UsedCooldownTimerText(Entity);

fn update_used_cooldown_timer_text_system(
    mut text_q: Query<(&mut Text, &UsedCooldownTimerText)>,
    used_cooldown_timer_q: Query<&UsedCooldownTimer>,
) {
    for (mut text, target) in text_q.iter_mut() {
        if let Ok(timer) = used_cooldown_timer_q.get(target.0) {
            text.0 = format!("{:.1}", timer.0.remaining().as_secs_f32())
        } else {
            text.0 = format!("0");
        }
    }
}

fn update_used_cooldown_timer_system(
    mut commands: Commands,
    mut used_cooldown_timer_q: Query<(&mut UsedCooldownTimer, &mut UnitDataButton, Entity)>,
    time: Res<Time>,
) {
    for (mut used_cooldown_timer, mut button, entity) in used_cooldown_timer_q.iter_mut() {
        used_cooldown_timer.0.tick(time.delta());

        if used_cooldown_timer.0.just_finished() {
            button.disabled = false;

            commands.entity(entity).remove::<UsedCooldownTimer>();
        }
    }
}

impl UsedCooldownTimer {
    pub fn new(secs: u64) -> UsedCooldownTimer {
        UsedCooldownTimer(Timer::new(Duration::from_secs(secs), TimerMode::Once))
    }
}

#[derive(Debug, Component)]
pub struct UnitDataButton {
    pub unit_data: UnitData,
    pub disabled: bool,
}

fn unit_data_button_system(
    mut commands: Commands,
    mut button_q: Query<(&mut UnitDataButton, &Interaction, Entity), Changed<Interaction>>,
    mut map_data: ResMut<MapState>,
) {
    for (mut button, interaction, entity) in button_q.iter_mut() {
        match *interaction {
            Interaction::Pressed => {
                if !button.disabled {
                    button.disabled = true;

                    commands.entity(entity).insert(UsedCooldownTimer::new(10));

                    map_data.selelcted_map_item_data =
                        Some(MapItemData::from_unit_data(&button.unit_data));
                }
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
            Button,
            UnitDataButton {
                unit_data: unit_data.clone(),
                disabled: false,
            },
            UsedCooldownTimer::new(10),
        ))
        .with_children(|parent| {
            let entity = parent.target_entity();

            parent.spawn((
                Node {
                    width: px(64),
                    height: px(64),
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                children![(Text::new("0"), UsedCooldownTimerText(entity)),],
            ));

            parent
                .spawn((
                    Node {
                        width: px(64),
                        height: px(64),
                        position_type: PositionType::Absolute,
                        ..default()
                    },
                    image,
                ))
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

    app.add_systems(
        Update,
        (
            unit_data_button_system,
            update_used_cooldown_timer_system,
            update_used_cooldown_timer_text_system,
        ),
    );
}
