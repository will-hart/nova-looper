use avian2d::prelude::{Collider, RigidBody, Sensor};
use bevy::{color::palettes::css::BLACK, prelude::*};
use rand::{Rng, thread_rng};

use crate::{
    consts::MAX_PLAYER_RADIUS,
    obstacle::Obstacle,
    player::{ItemPosition, Player},
    sun::Sun,
    supernova::Nova,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<WarpBarrier>();
    app.add_systems(
        OnEnter(Nova::During),
        (clear_existing_obstacles, spawn_barriers).chain(),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct WarpBarrier;

fn clear_existing_obstacles(mut commands: Commands, obstacles: Query<Entity, With<Obstacle>>) {
    for entity in &obstacles {
        commands.entity(entity).despawn();
    }
}

fn spawn_barriers(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    player: Single<&ItemPosition, With<Player>>,
    sun: Single<&Sun>,
) {
    let current_theta = player.theta;
    let color = materials.add(Color::Srgba(BLACK));

    let mut rng = thread_rng();

    for extra in 4..=14 {
        let scale = rng.gen_range(20.0..30.0);
        let mesh = meshes.add(Circle::new(scale));
        let theta = current_theta + extra as f32 * rng.gen_range(0.5..0.7);
        let radius = rng.gen_range(sun.radius..(sun.radius + 0.8 * MAX_PLAYER_RADIUS - 80.0));

        let pos = Vec3::new(
            radius * (theta + 0.1).sin(),
            radius * (theta + 0.1).cos(),
            -0.3,
        );
        commands.spawn((
            StateScoped(Nova::During),
            WarpBarrier,
            Mesh2d(mesh.clone()),
            MeshMaterial2d(color.clone()),
            RigidBody::Dynamic,
            Collider::circle(0.98 * scale),
            Sensor,
            Transform::from_translation(pos).with_rotation(Quat::from_axis_angle(
                Vec3::Z,
                pos.truncate().to_angle() + std::f32::consts::FRAC_PI_2,
            )),
        ));
    }
}
