// This shader draws the color plane in various color spaces.
#import bevy_ui::ui_vertex_output::UiVertexOutput

@group(1) @binding(0) var<uniform> progress: f32;
@group(1) @binding(1) var texture: texture_2d_array<f32>;
@group(1) @binding(2) var texture_sampler: sampler;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv;

    let p = uv - vec2(0.5);

    var angle = atan2(p.y, p.x);
    angle += 3.141592653589793;

    let start = progress / 100 * 2.0 * 3.141592653589793;

    if (start >= angle) {
        return vec4(1.0, 0.0, 0.0, 1.0);
    } else {
        return vec4(0.0, 1.0, 0.0, 1.0);
        // return textureSample(texture, texture_sampler, uv)
    }
}
