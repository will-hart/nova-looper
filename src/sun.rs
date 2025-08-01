use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Sun>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_sun);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Sun {
    pub radius: f32,
}

impl Default for Sun {
    fn default() -> Self {
        Self { radius: 500.0 }
    }
}

fn spawn_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sun = Sun::default();
    let mesh = meshes.add(Circle::new(sun.radius));
    let color = Color::hsl(30.0, 0.95, 0.7);
    commands.spawn((Mesh2d(mesh), MeshMaterial2d(materials.add(color)), sun));
}
