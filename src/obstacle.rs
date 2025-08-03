use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enoki::{
    NoAutoAabb, ParticleEffectHandle, ParticleSpawner,
    prelude::{OneShot, ParticleEffectInstance, ParticleSpawnerState, Rval},
};
use bevy_seedling::sample::SamplePlayer;
use rand::{Rng, thread_rng};

/// nova obstacles
mod nova;

use crate::{
    PlayerAssets,
    consts::{MAX_PLAYER_RADIUS, OBSTACLE_COLOR, SHIELD_COST_ON_OBSTACLE_HIT},
    obstacle::nova::WarpBarrier,
    player::{ItemPosition, Player, PlayerPower, PlayerShield},
    screens::Screen,
    sun::Sun,
    supernova::Nova,
    utils::DestroyAt,
};

const BURNED_UP: &str = "You burned up in the sun.";
const BLACK_HOLE: &str = "You flew into a black hole.";

pub(super) fn plugin(app: &mut App) {
    app.register_type::<DeathReason>();
    app.register_type::<Obstacle>();
    app.register_type::<AsteroidDebris>();

    app.init_resource::<DeathReason>();

    app.add_plugins(nova::plugin);

    app.add_systems(OnEnter(Screen::Gameplay), reset_death_reason);

    app.add_systems(
        Update,
        (
            (periodically_spawn_obstacles, collide_obstacles).run_if(in_state(Nova::Idle)),
            collide_obstacles.run_if(in_state(Nova::During)),
            // this doesn't seem to work :shrug
            update_debris_gravity_direction,
        ),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Obstacle;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct DeathReason(pub String);

impl Default for DeathReason {
    fn default() -> Self {
        Self(BURNED_UP.into())
    }
}

fn reset_death_reason(mut death_reason: ResMut<DeathReason>) {
    death_reason.0 = BURNED_UP.into();
}

fn periodically_spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: Local<f32>,
    player: Single<&ItemPosition, With<Player>>,
) {
    if *timer > 0.0 {
        *timer -= time.delta_secs();
        return;
    }

    let mut rng = thread_rng();

    *timer = rng.gen_range(0.1..0.4);
    let num_obstacles = rng.gen_range(1..=3);
    let radius = rng.gen_range(-75.0..(MAX_PLAYER_RADIUS * 0.5));

    for _ in 0..num_obstacles {
        let radius = radius + rng.gen_range(-60.0..20.0);
        let theta = player.theta + std::f32::consts::PI + rng.gen_range(-0.05..=0.05);
        commands.queue(SpawnObstacle {
            theta,
            radius,
            speed: rng.gen_range(-30.0..-15.0),
            // destroy after one player revolution
            destroy_at: std::f32::consts::TAU / player.speed + time.elapsed_secs(),
        });
    }
}

#[derive(Debug, Clone, Copy)]
struct SpawnObstacle {
    theta: f32,
    radius: f32,
    destroy_at: f32,
    speed: f32,
}

impl Command for SpawnObstacle {
    fn apply(self, world: &mut World) {
        let _ = world.run_system_cached_with(spawn_obstacle, self);
    }
}

fn spawn_obstacle(
    config: In<SpawnObstacle>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    sun: Single<&Sun>,
) {
    // info!("Spawning obstacle");
    let mesh = meshes.add(Rectangle::new(1.0, 15.0));

    let radius = config.radius + sun.radius;
    let theta = config.theta;
    let translation = Vec3::new(radius * theta.sin(), radius * theta.cos(), -1.0);

    commands.spawn((
        Obstacle,
        Transform::from_translation(translation).with_rotation(Quat::from_axis_angle(
            Vec3::Z,
            translation.truncate().to_angle() + std::f32::consts::FRAC_PI_2,
        )),
        RigidBody::Dynamic,
        Collider::rectangle(4.0, 14.0),
        LinearVelocity(-translation.truncate().normalize() * config.speed),
        Sensor,
        StateScoped(Screen::Gameplay),
        DestroyAt(config.destroy_at),
        ItemPosition {
            radius,
            theta,
            speed: 0.0,
            center: Vec2::ZERO,
        },
        Visibility::Visible,
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(OBSTACLE_COLOR)),
    ));
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct AsteroidDebris;

fn collide_obstacles(
    mut commands: Commands,
    player_assets: Option<Res<PlayerAssets>>,
    asset_server: Res<AssetServer>,
    mut screen: ResMut<NextState<Screen>>,
    mut death_reason: ResMut<DeathReason>,
    colliders: Query<(Entity, &CollidingEntities)>,
    obstacles: Query<&Transform, With<Obstacle>>,
    warp_barriers: Query<(), With<WarpBarrier>>,
    mut power: Single<(&mut PlayerPower, &mut PlayerShield)>,
) {
    for (_entity, colliding) in &colliders {
        if colliding.is_empty() {
            continue;
        }

        for collider in colliding.iter() {
            if warp_barriers.get(*collider).is_ok() {
                // uh oh we dead, can't go round hitting things in warp
                screen.set(Screen::GameOver);
                death_reason.0 = BLACK_HOLE.into();

                if let Some(player_assets) = &player_assets {
                    commands.spawn(SamplePlayer::new(player_assets.obstacle_hit.clone()));
                }
            }

            if let Ok(tx) = obstacles.get(*collider) {
                power.0.0 = (power.0.0 - 25.0).clamp(0.0, 100.0);
                power.1.0 = (power.1.0 - SHIELD_COST_ON_OBSTACLE_HIT).clamp(0.0, 100.0);

                // create a particle effect
                let mut new_tx = tx.translation;
                new_tx.z = 0.2;

                let effect = asset_server.load("particles/asteroid_hit.ron");

                commands.spawn((
                    AsteroidDebris,
                    Transform::from_translation(new_tx),
                    ParticleSpawner::default(),
                    ParticleEffectHandle(effect),
                    ParticleSpawnerState::default(),
                    OneShot::Despawn,
                    NoAutoAabb,
                ));

                // destroy obstacle
                commands.entity(*collider).despawn();

                if let Some(player_assets) = &player_assets {
                    commands.spawn(SamplePlayer::new(player_assets.obstacle_hit.clone()));
                }
            }
        }
    }
}

fn update_debris_gravity_direction(
    mut commands: Commands,
    mut particles: Query<(Entity, &Transform, &mut ParticleEffectInstance), With<AsteroidDebris>>,
) {
    for (entity, tx, mut maybe_effect) in &mut particles {
        // info!("Setting gravity on particles");
        let direction = -tx.translation;
        if let Some(effect) = maybe_effect.0.as_mut() {
            effect.gravity_direction = Some(Rval::new(direction.truncate(), 0.0));
        }

        commands.entity(entity).remove::<AsteroidDebris>();
    }
}
