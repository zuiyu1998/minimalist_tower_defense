use bevy::prelude::*;
use vleue_navigator::VleueNavigatorPlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(VleueNavigatorPlugin);
}
