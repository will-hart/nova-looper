use avian2d::prelude::{Collider, RigidBody, Sensor};
use bevy::{color::palettes::css::BLACK, prelude::*};
use rand::{Rng, thread_rng};

use crate::{
    consts::MAX_PLAYER_RADIUS,
    obstacle::Obstacle,
    player::{ItemPosition, Player},
    screens::Screen,
    sun::Sun,
    supernova::{Nova, NovaTimer},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<BlackHole>();
    app.add_systems(
        OnEnter(Nova::During),
        (clear_existing_obstacles, spawn_barriers).chain(),
    );
    app.add_systems(
        Update,
        scale_down_black_holes.run_if(in_state(Nova::After).and(resource_exists::<NovaTimer>)),
    );
    app.add_systems(OnEnter(Nova::After), deterrify_black_holes);
    app.add_systems(OnExit(Nova::After), despawn_black_holes);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub(super) struct BlackHole;

fn clear_existing_obstacles(mut commands: Commands, obstacles: Query<Entity, With<Obstacle>>) {
    for entity in &obstacles {
        commands.entity(entity).despawn();
    }
}

// make black holes less scary?
fn deterrify_black_holes(mut commands: Commands, black_holes: Query<Entity, With<BlackHole>>) {
    for entity in &black_holes {
        commands
            .entity(entity)
            .remove::<Sensor>()
            .remove::<Collider>();
    }
}
// make black holes less scary?
fn despawn_black_holes(mut commands: Commands, black_holes: Query<Entity, With<BlackHole>>) {
    for entity in &black_holes {
        commands.entity(entity).despawn();
    }
}

fn scale_down_black_holes(
    timer: Res<NovaTimer>,
    mut black_holes: Query<&mut Transform, With<BlackHole>>,
) {
    let remaining = timer.0.fraction_remaining();

    for mut bh_tx in &mut black_holes {
        bh_tx.scale = Vec3::splat(remaining);
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
            BlackHole,
            StateScoped(Screen::Gameplay),
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
