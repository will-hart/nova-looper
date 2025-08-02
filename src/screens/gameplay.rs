//! The screen state for the main gameplay.

use bevy::prelude::*;
use bevy_seedling::{prelude::Volume, sample::SamplePlayer};

use crate::MusicAssets;

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_music);
}

fn spawn_music(mut commands: Commands, music: Res<MusicAssets>) {
    commands.spawn((
        SamplePlayer::new(music.gameplay.clone())
            .looping()
            .with_volume(Volume::Linear(0.4)),
        StateScoped(Screen::Gameplay),
    ));
}
