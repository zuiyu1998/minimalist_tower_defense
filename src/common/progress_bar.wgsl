// This shader draws the color plane in various color spaces.
#import bevy_ui::ui_vertex_output::UiVertexOutput
#import bevy_render::maths::PI

@group(1) @binding(0) var<uniform> progress: f32;
@group(1) @binding(1) var material_color_texture: texture_2d<f32>;
@group(1) @binding(2) var material_color_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let p = uv - vec2(0.5);

    var angle = atan2(p.y, p.x);
    angle += PI;

    let start = progress / 100 * 2.0 * PI;

    if (start >= angle) {
        let output_color = textureSample(material_color_texture, material_color_sampler, uv);
        return output_color;
    } else {
        return vec4(0.0);
    }
}
