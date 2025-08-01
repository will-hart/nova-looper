use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_enoki::{
    NoAutoAabb, ParticleEffectHandle, ParticleSpawner,
    prelude::{OneShot, ParticleEffectInstance, ParticleSpawnerState, Rval},
};
use rand::{Rng, thread_rng};

use crate::{
    consts::MAX_PLAYER_RADIUS,
    player::{ItemPosition, Player, PlayerShield, PlayerPower},
    screens::Screen,
    sun::Sun,
    utils::DestroyAt,
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Obstacle>();
    app.register_type::<AsteroidDebris>();

    app.add_systems(
        Update,
        (
            periodically_spawn_obstacles,
            collide_obstacles,
            update_debris_gravity_direction,
        ),
    );
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Obstacle;

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

    *timer = thread_rng().gen_range(0.4..0.6);
    let radius = thread_rng().gen_range(20.0..(MAX_PLAYER_RADIUS * 0.35));
    let theta = player.theta + std::f32::consts::PI;
    commands.queue(SpawnObstacle {
        theta,
        radius,
        // destroy after one player revolution
        destroy_at: std::f32::consts::TAU / player.speed + time.elapsed_secs(),
    });
}

#[derive(Debug, Clone, Copy)]
struct SpawnObstacle {
    theta: f32,
    radius: f32,
    destroy_at: f32,
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
    let mesh = meshes.add(Rhombus::new(15.0, 15.0));
    let color = Color::hsl(76.0, 0.30, 0.47);

    let radius = config.radius + sun.radius;
    let theta = config.theta;

    commands.spawn((
        Obstacle,
        Transform::from_translation(Vec3::new(radius * theta.sin(), radius * theta.cos(), 0.0)),
        RigidBody::Kinematic,
        Collider::circle(7.0),
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
        MeshMaterial2d(materials.add(color)),
    ));
}

#[derive(Debug, Component, Reflect)]
#[reflect(Component)]
pub struct AsteroidDebris;

fn collide_obstacles(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    colliders: Query<(Entity, &CollidingEntities)>,
    obstacles: Query<&Transform, With<Obstacle>>,
    mut power: Single<(&mut PlayerPower, &mut PlayerShield)>,
) {
    for (_entity, colliding) in &colliders {
        if colliding.is_empty() {
            continue;
        }

        for collider in colliding.iter() {
            if let Ok(tx) = obstacles.get(*collider) {
                power.0.0 = 0.0;
                power.1.0 = (power.1.0 + 30.0).clamp(0.0, 100.0);

                // create a particle effect
                let mut new_tx = tx.translation.clone();
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
            }
        }
    }
}

fn update_debris_gravity_direction(
    mut commands: Commands,
    mut particles: Query<(Entity, &Transform, &mut ParticleEffectInstance), With<AsteroidDebris>>,
) {
    for (entity, tx, mut maybe_effect) in &mut particles {
        info!("Setting gravity on particles");
        let direction = -tx.translation;
        if let Some(effect) = maybe_effect.0.as_mut() {
            effect.gravity_direction = Some(Rval::new(direction.truncate(), 0.0));
        }

        commands.entity(entity).remove::<AsteroidDebris>();
    }
}
