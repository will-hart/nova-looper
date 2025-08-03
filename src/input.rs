use bevy::prelude::*;

use crate::{
    consts::{MAGIC_MOVEMENT_ACCEL_SCALE, MAX_PLAYER_RADIUS, PLAYER_RADIUS_CHANGE_SPEED},
    player::{ItemPosition, Player},
    screens::Screen,
    supernova::Nova,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, control_player.run_if(in_state(Screen::Gameplay)));
    app.init_resource::<PlayerInputAngle>();
}

#[derive(Resource, Reflect, Debug, Default)]
#[reflect(Resource)]
pub struct PlayerInputAngle(pub f32);

fn control_player(
    nova: Res<State<Nova>>,
    mut delta: ResMut<PlayerInputAngle>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    touches: Res<Touches>,
    mouse: Res<ButtonInput<MouseButton>>,
    time: Res<Time>,
    mut player: Single<&mut ItemPosition, With<Player>>,
) {
    match **nova {
        Nova::Idle | Nova::During => {
            if keyboard_input.pressed(KeyCode::Space)
                || touches.iter().next().is_some()
                || mouse.pressed(MouseButton::Left)
            {
                delta.0 += MAGIC_MOVEMENT_ACCEL_SCALE * time.delta_secs();
            } else {
                delta.0 -= MAGIC_MOVEMENT_ACCEL_SCALE * time.delta_secs();
            }
            delta.0 = delta.0.clamp(-1.0, 1.0);
        }
        Nova::BuildingUp => {
            delta.0 = 1.0;
        }
        Nova::After => {
            delta.0 = -1.0;
        }
    };

    player.radius = (player.radius + delta.0 * time.delta_secs() * PLAYER_RADIUS_CHANGE_SPEED)
        .clamp(0.5, MAX_PLAYER_RADIUS);
}
