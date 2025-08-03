use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Rotate>();
    app.add_systems(
        Update,
        (
            rotate_shapes,
            destroy_at_watcher,
            text_scaling_system,
            move_items_in_direction,
        ),
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
        tx.rotation *= Quat::from_axis_angle(Vec3::Z, rot.0 * time.delta_secs());
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

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct ScaleTextOverTime {
    pub rate: f32,
    pub max: f32,
}

impl ScaleTextOverTime {
    fn update(&self, dt: f32, current: f32) -> f32 {
        let amount = self.rate * dt;
        (current + amount).min(self.max)
    }
}

fn text_scaling_system(time: Res<Time>, mut texts: Query<(&mut TextFont, &ScaleTextOverTime)>) {
    for (mut font, scale) in &mut texts {
        font.font_size = scale.update(time.delta_secs(), font.font_size);
    }
}

#[derive(Component, Reflect, Debug)]
#[reflect(Component)]
pub struct MoveInDirection(pub Vec2);

pub fn move_items_in_direction(
    time: Res<Time>,
    mut items: Query<(&mut Transform, &MoveInDirection)>,
) {
    for (mut tx, mover) in &mut items {
        tx.translation += (time.delta_secs() * mover.0).extend(0.0);
    }
}
