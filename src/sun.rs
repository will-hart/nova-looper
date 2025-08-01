use bevy::prelude::*;

use crate::{consts::SUN_STARTING_RADIUS, materials::SunMaterial, screens::Screen};

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
        Self {
            radius: SUN_STARTING_RADIUS,
        }
    }
}

fn spawn_sun(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SunMaterial>>,
) {
    let sun = Sun::default();
    let mesh = meshes.add(Rectangle::new(
        2.0 * sun.radius + 500.0,
        2.0 * sun.radius + 500.0,
    ));
    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(SunMaterial::default())),
        StateScoped(Screen::Gameplay),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)),
        sun,
    ));
}
