use bevy::prelude::*;

use crate::{
    common::LightSource,
    product::ProductMeta,
    unit::{Unit, UnitFactory},
};

#[derive(Debug, Component)]
pub struct Bonfire {
    products: Vec<ProductMeta>,
}

impl Default for Bonfire {
    fn default() -> Self {
        Bonfire {
            products: vec![ProductMeta {
                name: "sunlight".to_string(),
                value: 10.0,
            }],
        }
    }
}

#[derive(Debug)]
pub struct BonfireFactory;

fn process(mut bonfire_q: Query<(&Bonfire, &mut Unit)>, mut writer: MessageWriter<ProductMeta>) {
    for (bonfire, mut unit) in bonfire_q.iter_mut() {
        if unit.cooling_down {
            unit.cooling_down = false;

            tracing::info!("Products is generated.");
            writer.write_batch(bonfire.products.iter().cloned());
        }
    }
}

impl UnitFactory for BonfireFactory {
    fn spawn(&self, _data: &super::UnitData, commands: &mut EntityCommands) {
        commands.insert((Bonfire::default(), LightSource, Name::new("Bonfire")));
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_systems(FixedUpdate, process);
}
