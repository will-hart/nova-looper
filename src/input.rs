use bevy::prelude::*;

use crate::{
    consts::{MAGIC_MOVEMENT_SCALE, MAX_PLAYER_RADIUS},
    player::{ItemPosition, Player},
    screens::Screen,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, control_player.run_if(in_state(Screen::Gameplay)));
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut ItemPosition, With<Player>>,
    time: Res<Time>,
) {
    let mut delta = -1.0f32;

    if keyboard_input.pressed(KeyCode::Space) {
        delta = 1.0;
    }

    player.radius = (player.radius + delta * time.delta_secs() * MAGIC_MOVEMENT_SCALE)
        .clamp(0.5, MAX_PLAYER_RADIUS);
}
