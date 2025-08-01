use bevy::prelude::*;

use crate::{
    consts::{MAGIC_MOVEMENT_ACCEL_SCALE, MAGIC_MOVEMENT_SCALE, MAX_PLAYER_RADIUS},
    player::{ItemPosition, Player},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, control_player.run_if(in_state(Screen::Gameplay)));
    app.init_resource::<PlayerInputAngle>();
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct PlayerInputAngle(pub f32);

fn control_player(
    mut delta: ResMut<PlayerInputAngle>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut player: Single<&mut ItemPosition, With<Player>>,
) {
    if keyboard_input.pressed(KeyCode::Space) {
        delta.0 += MAGIC_MOVEMENT_ACCEL_SCALE * time.delta_secs();
    } else {
        delta.0 -= MAGIC_MOVEMENT_ACCEL_SCALE * time.delta_secs();
    }

    delta.0 = delta.0.clamp(-1.0, 1.0);

    player.radius = (player.radius + delta.0 * time.delta_secs() * MAGIC_MOVEMENT_SCALE)
        .clamp(0.5, MAX_PLAYER_RADIUS);
}
