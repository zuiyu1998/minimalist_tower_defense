use bevy::{
    asset::{load_internal_asset, uuid_handle},
    prelude::*,
    render::render_resource::AsBindGroup,
    shader::ShaderRef,
};

const PROGRESS_BAR_SHADER_HANDLE: Handle<Shader> =
    uuid_handle!("3b23a76a-7eaf-49f6-8718-cbe88ad20310");

/// 圆形进度条
#[derive(Debug, Component)]
pub struct ProgressBar {
    pub texture: Handle<Image>,
    pub value: f32,
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct ProgressBarUiMaterial {
    #[uniform(0)]
    pub value: f32,
    #[texture(1)]
    #[sampler(2)]
    pub texture: Handle<Image>,
}

impl UiMaterial for ProgressBarUiMaterial {
    fn fragment_shader() -> ShaderRef {
        PROGRESS_BAR_SHADER_HANDLE.into()
    }
}

pub fn update_progress_bar(
    q_progress_bar: Query<(Entity, &ProgressBar), Changed<ProgressBar>>,
    q_material_node: Query<&MaterialNode<ProgressBarUiMaterial>>,
    mut r_materials: ResMut<Assets<ProgressBarUiMaterial>>,
    mut commands: Commands,
) {
    for (progress_bar_entity, progress_bar) in q_progress_bar.iter() {
        if let Ok(material_node) = q_material_node.get(progress_bar_entity) {
            // Node component exists, update it
            if let Some(material) = r_materials.get_mut(material_node.id()) {
                // Update properties
                material.texture = progress_bar.texture.clone();
                material.value = progress_bar.value;
            }
        } else {
            // Insert new node component
            let material = r_materials.add(ProgressBarUiMaterial {
                texture: progress_bar.texture.clone(),
                value: progress_bar.value,
            });
            commands
                .entity(progress_bar_entity)
                .insert(MaterialNode(material));
        }
    }
}

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PROGRESS_BAR_SHADER_HANDLE,
            "progress_bar.wgsl",
            Shader::from_wgsl
        );

        app.add_plugins(UiMaterialPlugin::<ProgressBarUiMaterial>::default());
        app.add_systems(PostUpdate, update_progress_bar);
    }
}
