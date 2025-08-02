use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef, ShaderType},
    sprite::{Material2d, Material2dPlugin},
    window::WindowResized,
};
use rand::Rng;

use crate::{player::Player, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(Material2dPlugin::<StarfieldMaterial>::default())
        .add_systems(OnEnter(Screen::Gameplay), spawn_starfield)
        .add_systems(
            PostUpdate,
            update_starfield.run_if(in_state(Screen::Gameplay)),
        );
}

#[derive(Asset, TypePath, AsBindGroup, ShaderType, Debug, Clone)]
#[uniform(0, StarfieldMaterial)]
pub struct StarfieldMaterial {
    position: Vec2,
    seeds: Vec2,
    pub background: LinearRgba,
    pub foreground: LinearRgba,
}

impl<'a> From<&'a StarfieldMaterial> for StarfieldMaterial {
    fn from(material: &'a StarfieldMaterial) -> Self {
        material.clone()
    }
}

impl Material2d for StarfieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/starfield.wgsl".into()
    }
}

impl Default for StarfieldMaterial {
    fn default() -> Self {
        Self {
            position: Vec2::ZERO,
            seeds: Vec2::new(
                rand::thread_rng().gen_range(0.0..1000.0),
                rand::thread_rng().gen_range(0.0..1000.0),
            ),
            background: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
            foreground: LinearRgba::new(0.0, 0.0, 0.0, 1.0),
        }
    }
}

fn spawn_starfield(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StarfieldMaterial>>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let size = window.width().max(window.height());

    commands.spawn((
        // Apply the material to a square
        Mesh2d(meshes.add(Rectangle::default())),
        MeshMaterial2d(materials.add(StarfieldMaterial::default())),
        // Scale up the material so that it covers the whole screen
        Transform::from_scale(Vec3::new(size, size, 1.0)),
        StateScoped(Screen::Gameplay),
    ));
}

fn update_starfield(
    mut starfield: Query<(&mut Transform, &MeshMaterial2d<StarfieldMaterial>), Without<Player>>,
    player: Query<Ref<Transform>, With<Player>>,
    // mut materials: ResMut<Assets<StarfieldMaterial>>,
    mut resized: EventReader<WindowResized>,
) {
    // As the camera follows the player, take the camera transform
    let player_transforme = player.single().unwrap();
    if player_transforme.is_changed() {
        let (mut starfield_transform, _material) = starfield.single_mut().unwrap();

        // Change the starfield transform so that it stays in sync with the camera
        starfield_transform.translation = player_transforme.translation.with_z(-2.0);

        // // Update the position in the material
        // let material = materials.get_mut(&material.0).unwrap();
        // material.position = player_transforme.translation.xy();
    }

    if let Some(resized) = resized.read().last() {
        let (mut starfield_transform, _) = starfield.single_mut().unwrap();

        // Window size changed, update the size of the mesh showing the material
        starfield_transform.scale.x = resized.width.max(resized.height);
        starfield_transform.scale.y = resized.width.max(resized.height);
    }
}
