use bevy::prelude::*;

/// 圆形进度条
#[derive(Debug, Component)]
pub struct ProgressBar {
    pub texture: Handle<Image>,
    pub value: f32,
}

pub fn progress_bar(progress_bar: ProgressBar) -> impl Bundle {
    let texture = progress_bar.texture.clone();
    (
        progress_bar,
        ImageNode::new(texture),
        Node {
            width: percent(100),
            height: percent(100),
            ..default()
        },
    )
}

pub fn update_progress_bar(
    mut progress_bar_q: Query<&ImageNode, With<ProgressBar>>,
    mut assets: ResMut<Assets<Image>>,
) {
}

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {}
}
