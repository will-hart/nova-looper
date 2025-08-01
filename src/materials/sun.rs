use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<SunMaterial>::default());
}

#[derive(Asset, TypePath, AsBindGroup, ShaderType, Debug, Clone)]
#[uniform(0, SunMaterial)]
pub struct SunMaterial {
    color: LinearRgba,
    thickness: f32,
    fill: u32,
}

impl<'a> From<&'a SunMaterial> for SunMaterial {
    fn from(material: &'a SunMaterial) -> Self {
        material.clone()
    }
}

impl Material2d for SunMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sun.wgsl".into()
    }
}

impl Default for SunMaterial {
    fn default() -> Self {
        Self {
            color: LinearRgba::new(0.97, 0.65, 0.05, 1.0),
            thickness: 25.0,
            fill: 0,
        }
    }
}
