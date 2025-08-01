use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Obstacle>();
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Obstacle;
