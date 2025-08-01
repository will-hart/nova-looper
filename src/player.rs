use bevy::prelude::*;

use crate::{screens::Screen, sun::Sun};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);
    app.add_systems(
        Update,
        set_player_position.run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        PostUpdate,
        camera_follow_player.run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(OnExit(Screen::Gameplay), reset_camera_position);
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Player;

#[derive(Component, Debug, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct PlayerPosition {
    // The distance from the sun edge
    pub radius: f32,
    pub theta: f32,
    pub speed: f32,
    pub center: Vec2,
}

impl Default for PlayerPosition {
    fn default() -> Self {
        Self {
            radius: 50.0,
            speed: 1.2,
            theta: 0.0,
            center: Vec2::ZERO,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let player_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 10.0,
        Vec2::new(-5.0, -5.0),
        Vec2::new(5.0, -5.0),
    ));

    let color = Color::hsl(142.0, 0.95, 0.97);

    commands.spawn((
        Mesh2d(player_mesh),
        MeshMaterial2d(materials.add(color)),
        Player,
        PlayerPosition::default(),
    ));
}

fn set_player_position(
    time: Res<Time>,
    sun: Single<&Sun>,
    mut player: Single<(&mut Transform, &PlayerPosition)>,
) {
    let radius = sun.radius + player.1.radius;

    player.0.translation = Vec3::new(
        radius * (player.1.speed * time.elapsed_secs()).sin(),
        radius * (player.1.speed * time.elapsed_secs()).cos(),
        0.0,
    );

    player.0.rotation = Quat::from_axis_angle(
        Vec3::Z,
        player.0.translation.truncate().to_angle() + std::f32::consts::PI,
    );
}

fn camera_follow_player(
    mut cameras: Query<&mut Transform, (Without<Player>, With<Camera2d>)>,
    player: Single<Ref<Transform>, With<Player>>,
) {
    for mut camera in &mut cameras {
        camera.translation = player.translation;
    }
}

fn reset_camera_position(mut cameras: Query<&mut Transform, With<Camera2d>>) {
    for mut tx in &mut cameras {
        tx.translation = Vec3::ZERO;
    }
}
