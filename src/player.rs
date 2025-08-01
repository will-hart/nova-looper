use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enoki::{
    NoAutoAabb, ParticleEffectHandle, ParticleSpawner, prelude::ParticleSpawnerState,
};

use crate::{
    consts::{MAX_PLAYER_RADIUS, PLAYER_STARTING_SPEED},
    input::PlayerInputAngle,
    materials::BarDataSource,
    screens::Screen,
    sun::Sun,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Player>();
    app.register_type::<PlayerPower>();

    app.add_systems(OnEnter(Screen::Gameplay), spawn_player);
    app.add_systems(
        PreUpdate,
        (update_player_item_position, set_player_position)
            .chain()
            .run_if(in_state(Screen::Gameplay)),
    );
    app.add_systems(
        Update,
        (shield_decay, power_generation, shield_monitor).run_if(in_state(Screen::Gameplay)),
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

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct PlayerPower(pub f32);

impl BarDataSource for PlayerPower {
    fn current_frac(&self) -> f32 {
        (self.0 / 100.0).clamp(0.0, 100.0)
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PlayerShield(pub f32);

impl Default for PlayerShield {
    fn default() -> Self {
        Self(100.0)
    }
}

impl BarDataSource for PlayerShield {
    fn current_frac(&self) -> f32 {
        (self.0 / 100.0).clamp(0.0, 100.0)
    }
}

#[derive(Component, Debug, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct ItemPosition {
    // The distance from the sun edge
    pub radius: f32,
    pub theta: f32,
    pub speed: f32,
    pub center: Vec2,
}

impl Default for ItemPosition {
    fn default() -> Self {
        Self {
            radius: 250.0,
            speed: PLAYER_STARTING_SPEED,
            theta: 0.0,
            center: Vec2::ZERO,
        }
    }
}

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
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
        Transform::from_xyz(0.0, 0.0, 0.1),
        StateScoped(Screen::Gameplay),
        Player,
        PlayerPower::default(),
        PlayerShield::default(),
        ItemPosition::default(),
        RigidBody::Kinematic,
        Collider::capsule(4.5, 9.0),
        Sensor,
        CollidingEntities::default(),
        children![(
            Transform::from_translation(Vec3::new(0.0, 0.0, 0.1)),
            ParticleSpawner::default(),
            ParticleEffectHandle(asset_server.load("particles/rocket_trail.ron")),
            ParticleSpawnerState::default(),
            NoAutoAabb,
        )],
    ));
}

fn update_player_item_position(
    time: Res<Time>,
    mut player: Single<&mut ItemPosition, With<Player>>,
) {
    player.theta = player.speed * time.elapsed_secs();
}

fn set_player_position(
    player_angle: Res<PlayerInputAngle>,
    sun: Single<&Sun>,
    mut player: Single<(&mut Transform, &ItemPosition), With<Player>>,
) {
    let radius = sun.radius + player.1.radius;

    player.0.translation = Vec3::new(
        radius * player.1.theta.sin(),
        radius * player.1.theta.cos(),
        0.1,
    );

    let extra_angle = if player.1.radius <= 1.0 || player.1.radius >= MAX_PLAYER_RADIUS {
        0.0
    } else {
        player_angle.0 * 0.3
    };

    player.0.rotation = Quat::from_axis_angle(
        Vec3::Z,
        player.0.translation.truncate().to_angle() + std::f32::consts::PI + extra_angle,
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

fn power_generation(time: Res<Time>, mut player: Single<(&ItemPosition, &mut PlayerPower)>) {
    let distance = player.0.radius;
    // linear decay
    let power = -0.05 * distance + 10.0;
    player.1.0 += time.delta_secs() * power.clamp(-3.0, 10.0);
}

fn shield_decay(time: Res<Time>, mut player: Single<(&ItemPosition, &mut PlayerShield)>) {
    let distance = player.0.radius;
    let rate = if distance > 55.0 {
        0.3 * distance + 23.0 // linear after intersection
    } else {
        -2.0 / (0.005 * distance)
    };
    player.1.0 = (player.1.0 + time.delta_secs() * rate.clamp(-10.0, 10.0)).clamp(0.0, 100.0);
}

fn shield_monitor(mut next_state: ResMut<NextState<Screen>>, shield: Single<&PlayerShield>) {
    if shield.0 < 0.1 {
        next_state.set(Screen::GameOver);
    }
}
