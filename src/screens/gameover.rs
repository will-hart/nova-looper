//! The game over screen that appears after the player loses.

use bevy::prelude::*;

use crate::{screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), spawn_gameover_menu);
}

fn spawn_gameover_menu(mut commands: Commands) {
    commands.spawn((
        widget::ui_root("Game over scren"),
        GlobalZIndex(2),
        StateScoped(Screen::GameOver),
        children![
            widget::header("Game Over!"),
            widget::header("Your shields were down for too long!"),
            widget::menu_button("Play again", play_again),
            #[cfg(not(target_family = "wasm"))]
            widget::menu_button("Main Menu", return_to_menu),
        ],
    ));
}

fn play_again(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Gameplay);
}

fn return_to_menu(_: Trigger<Pointer<Click>>, mut next_screen: ResMut<NextState<Screen>>) {
    next_screen.set(Screen::Title);
}
