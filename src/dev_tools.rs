//! Development tools for the game. This plugin is only enabled in dev builds.

use avian2d::prelude::*;
use bevy::{
    color::palettes,
    dev_tools::{picking_debug::DebugPickingPlugin, states::log_transitions},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use bevy_inspector_egui::{bevy_egui::EguiPlugin, quick::WorldInspectorPlugin};

use crate::{navigator::NavigatorPath, screens::Screen};

pub fn display_navigator_path(navigator: Query<(&Transform, &NavigatorPath)>, mut gizmos: Gizmos) {
    for (transform, path) in &navigator {
        let mut to_display = path.next.iter().map(|v| v.xy()).collect::<Vec<_>>();
        to_display.push(path.current.xy());
        to_display.push(transform.translation.xy());
        to_display.reverse();
        if !to_display.is_empty() {
            gizmos.linestrip_2d(to_display, palettes::tailwind::YELLOW_400);
        }
    }
}

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        PhysicsDebugPlugin,
        EguiPlugin::default(),
        WorldInspectorPlugin::new(),
        DebugPickingPlugin,
    ));

    app.add_systems(PreUpdate, display_navigator_path);

    // Log `Screen` state transitions.
    app.add_systems(Update, log_transitions::<Screen>);

    // Toggle the debug overlay for UI.
    app.add_systems(
        Update,
        toggle_debug_ui.run_if(input_just_pressed(TOGGLE_KEY)),
    );
}

const TOGGLE_KEY: KeyCode = KeyCode::Backquote;

fn toggle_debug_ui(mut options: ResMut<UiDebugOptions>) {
    options.toggle();
}
