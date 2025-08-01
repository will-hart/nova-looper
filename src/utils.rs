use bevy::prelude::*;

use crate::screens::Screen;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Rotate>();
    app.add_systems(
        Update,
        (rotate_shapes, destroy_at_watcher).run_if(in_state(Screen::Gameplay)),
    );
}

pub fn format_number(number: f32) -> String {
    format!("{number:.0}")
}

/// Marks the shape to rotate around the z-axis with the given speed
/// in units per second
#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct Rotate(pub f32);

impl Default for Rotate {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Rotates shapes a little bit each frame based on the rotation speed
fn rotate_shapes(time: Res<Time>, mut shapes: Query<(&mut Transform, &Rotate)>) {
    for (mut tx, rot) in &mut shapes {
        tx.rotation = Quat::from_axis_angle(Vec3::Z, rot.0 * time.delta_secs());
    }
}

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct DestroyAt(pub f32);

/// Destroys [DestroyAt] components after their scheduled time
fn destroy_at_watcher(
    mut commands: Commands,
    time: Res<Time>,
    destroyees: Query<(Entity, &DestroyAt)>,
) {
    let elapsed = time.elapsed_secs();
    for (ent, destroyee) in &destroyees {
        if elapsed > destroyee.0 {
            // info!("Destroying obstacle");
            commands.entity(ent).despawn();
        }
    }
}
