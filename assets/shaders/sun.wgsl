#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Material {
    color: vec4f,
    // can use this to blur
    blur_start: f32,
}

@group(2) @binding(0) var<uniform> material: Material;

@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    let pos : vec2<f32> = mesh.uv * 2.0 - vec2<f32>(1.0,1.0);
    let dist = length(pos);

    if dist > 1.0 { discard; }
    return vec4<f32>(material.color.rgb, 1.0 - smoothstep(material.blur_start, 1.0, dist));
}
