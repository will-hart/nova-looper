//! The game over screen that appears after the player loses.

use bevy::prelude::*;
use bevy_seedling::sample::SamplePlayer;

use crate::{DeathReason, PlayerAssets, score::Score, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::GameOver), spawn_gameover_menu);
}

fn spawn_gameover_menu(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    score: Option<Res<Score>>,
    death_reason: Res<DeathReason>,
) {
    commands.spawn(SamplePlayer::new(player_assets.end_game.clone()));

    let score = score.map(|s| s.score).unwrap_or_default();

    commands.spawn((
        widget::ui_root("Game over scren"),
        GlobalZIndex(2),
        StateScoped(Screen::GameOver),
        children![
            widget::header("Game Over!"),
            widget::label(death_reason.0.clone()),
            widget::label(format!("You scored {score:.0}!")),
            widget::menu_button("Play again", play_again),
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
