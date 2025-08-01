#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Material {
    color: vec4f,
    width: f32,
    fill: u32,
}

@group(2) @binding(0) var<uniform> material: Material;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pos : vec2<f32> = mesh.uv * 2.0 - vec2<f32>(1.0,1.0);
    let dist = 1.0 - length(pos);

    if material.fill > 0 {
        if dist < 0.0 { discard; }
        return vec4<f32>(material.color.rgb, 1.0);
    }

    let pp = 1.0 / dpdx(pos.x);
    let pixels = dist * pp;
    let edge = min(pixels, material.width - pixels) * 0.5;
    if edge < 0.0 { discard; }
    return vec4<f32>(material.color.rgb, min(1.0, edge));
}
