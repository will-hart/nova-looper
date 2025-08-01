use bevy::prelude::*;

use crate::{
    player::{ItemPosition, Player},
    screens::Screen,
};

const RANDOM_MOVEMENT_SCALE: f32 = 150.0;
pub const MAX_PLAYER_RADIUS: f32 = 400.0;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, control_player.run_if(in_state(Screen::Gameplay)));
}

fn control_player(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player: Single<&mut ItemPosition, With<Player>>,
    time: Res<Time>,
) {
    let mut delta = 0f32;

    if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW) {
        delta += 1.0;
    }
    if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS) {
        delta -= 1.0;
    }

    if delta.abs() > 0.1 {
        player.radius = (player.radius + delta * time.delta_secs() * RANDOM_MOVEMENT_SCALE)
            .clamp(0.5, MAX_PLAYER_RADIUS);
    }
}
