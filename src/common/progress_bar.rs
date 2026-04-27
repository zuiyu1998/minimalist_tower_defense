use bevy::{prelude::*, render::render_resource::AsBindGroup, shader::ShaderRef};

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
        todo!()
    }
}

pub fn update_progress_bar(
    q_progress_bar: Query<(Entity, &ProgressBar), Changed<ProgressBar>>,
    q_children: Query<&Children>,
    q_material_node: Query<&MaterialNode<ProgressBarUiMaterial>>,
    mut r_materials: ResMut<Assets<ProgressBarUiMaterial>>,
    mut commands: Commands,
) {
    for (progress_bar_entity, progress_bar) in q_progress_bar.iter() {
        let Ok(children) = q_children.get(progress_bar_entity) else {
            continue;
        };
        let Some(inner_ent) = children.first() else {
            continue;
        };

        if let Ok(material_node) = q_material_node.get(*inner_ent) {
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
            commands.entity(*inner_ent).insert(MaterialNode(material));
        }
    }
}

pub struct ProgressBarPlugin;

impl Plugin for ProgressBarPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiMaterialPlugin::<ProgressBarUiMaterial>::default());
        app.add_systems(PostUpdate, update_progress_bar);
    }
}
