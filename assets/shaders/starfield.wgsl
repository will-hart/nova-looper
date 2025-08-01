#import bevy_sprite::{
    mesh2d_vertex_output::VertexOutput,
    mesh2d_view_bindings::globals,
}

struct Material {
    coords: vec2<f32>,
    seeds: vec2<f32>,
    background: vec4f,
    foreground: vec4f,
}

@group(2) @binding(0)
var<uniform> material: Material;

// Returns a single f32 for a position
fn rand(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(54.90898, 18.233))) * 4337.5453);
}

fn stars(position: vec2<f32>, density: f32, size: f32, brightness: f32) -> f32 {
    let n = position * density;
    let f = floor(n);

    var d = 1.0e10;
    for (var i = -1; i <= 1; i = i + 1) {
        for (var j = -1; j <= 1; j = j + 1) {
            var g = f + vec2<f32>(f32(i), f32(j));
            g = n - g - rand2(g % density) + rand(g);
            g = g / (density * size);
            d = min(d, dot(g, g));
        }
    }

    return brightness * (smoothstep(.95, 1., (1. - sqrt(d))));
}

// Returns two f32 for a position
fn rand2(p: vec2<f32>) -> vec2<f32> {
    let p2 = vec2<f32>(dot(p, vec2<f32>(12.9898, 78.233)), dot(p, vec2<f32>(26.65125, 83.054543)));
    return fract(sin(p2) * 43758.5453);
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    var result = material.background.rgb;
    // result = result + stars(in.uv - material.coords / (1000.0 * 1.2), 3.0, 0.025, 2.0);
    // result = result + stars(in.uv - material.coords / (1000.0 * 1.4), 10.0, 0.018, 1.0);
    // result = result + stars(in.uv - material.coords / (1000.0 * 2.0), 30.0, 0.015, 0.5);
    return vec4<f32>(result, 1.0);
}
