use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{AlphaMode2d, Material2d, Material2dPlugin},
};

use crate::consts::{INNER_SUN_COLOUR, SUN_COLOUR};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<SunMaterial>::default());
}

#[derive(Asset, TypePath, AsBindGroup, ShaderType, Debug, Clone)]
#[uniform(0, SunMaterial)]
pub struct SunMaterial {
    pub inner_color: Vec4,
    pub color: Vec4,
    /// the fraction of the UV coordinate where the sun starts to fade
    /// out. Defaults to 1.0 for now because its doesn't look great.
    blur_start: f32,
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

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

impl Default for SunMaterial {
    fn default() -> Self {
        Self {
            inner_color: INNER_SUN_COLOUR.to_srgba().to_vec4(),
            color: SUN_COLOUR.to_srgba().to_vec4(),
            blur_start: 1.0,
        }
    }
}
