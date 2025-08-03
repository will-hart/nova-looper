use bevy::prelude::*;

use crate::{player::Player, screens::Screen, supernova::Nova};

const MEGA_AGES_AWAY: Vec3 = Vec3::new(100_000.0, 100_000.0, 0.0);
const TRAIL_ITEM_OFFSET: Vec3 = Vec3::new(0.0, 0.0, -0.3);

pub struct TrailPlugin<const N: usize>;

impl<const N: usize> Plugin for TrailPlugin<N> {
    fn build(&self, app: &mut App) {
        app.register_type::<Trail>();
        app.register_type::<TrailItem>();

        app.add_systems(OnEnter(Screen::Gameplay), spawn_trail::<N>);
        app.add_systems(
            Update,
            update_trail.run_if(in_state(Nova::Idle).or(in_state(Nova::BuildingUp))),
        );
        app.add_systems(OnEnter(Nova::Idle), reset_trail);
        app.add_systems(OnExit(Nova::BuildingUp), hide_trail);
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Trail {
    items: Vec<Entity>,
    index: usize,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct TrailItem;

fn spawn_trail<const N: usize>(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut items: Vec<Entity> = Vec::with_capacity(N);
    let color = materials.add(Color::srgba(3.0, 4.0, 4.0, 1.0));
    let mesh = meshes.add(Rectangle::new(3.0, 22.0));

    for _ in 0..N {
        let entity = commands
            .spawn((
                TrailItem,
                Transform::from_translation(MEGA_AGES_AWAY),
                Visibility::Visible,
                StateScoped(Screen::Gameplay),
                children![(
                    Transform::from_translation(Vec3::new(0.0, -11.0, 0.0)),
                    Mesh2d(mesh.clone()),
                    MeshMaterial2d(color.clone()),
                )],
            ))
            .id();

        items.push(entity);
    }

    commands.spawn(Trail { items, index: 0 });
}

/// Resets all the items in the trail to the player's position
/// TODO: could make this more general but setting an entity to reset to.
fn reset_trail(
    player: Single<&Transform, With<Player>>,
    trails: Query<&Trail>,
    mut trail_items: Query<&mut Transform, (With<TrailItem>, Without<Player>)>,
) {
    for trail in &trails {
        for entity in trail.items.iter() {
            if let Ok(mut tx) = trail_items.get_mut(*entity) {
                tx.translation = player.translation + TRAIL_ITEM_OFFSET;
            }
        }
    }
}

/// Moves trail items freakin' ages away
fn hide_trail(trails: Query<&Trail>, mut trail_items: Query<&mut Transform, With<TrailItem>>) {
    for trail in &trails {
        for entity in trail.items.iter() {
            if let Ok(mut tx) = trail_items.get_mut(*entity) {
                tx.translation = MEGA_AGES_AWAY;
            }
        }
    }
}

/// Updates trails every frame
fn update_trail(
    player: Single<&Transform, With<Player>>,
    mut trails: Query<&mut Trail>,
    mut trail_items: Query<&mut Transform, (With<TrailItem>, Without<Player>)>,
) {
    for mut trail in &mut trails {
        let next_index = (trail.index + 1) % trail.items.len();
        if let Ok(mut tx) = trail_items.get_mut(trail.items[next_index]) {
            tx.translation = player.translation + TRAIL_ITEM_OFFSET;
            tx.rotation = player.rotation;
        }

        trail.index = next_index;
    }
}
