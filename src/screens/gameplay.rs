//! The screen state for the main gameplay.

use bevy::prelude::*;
use bevy_seedling::{
    pool::SamplerPool,
    prelude::{PoolLabel, Volume, VolumeNode},
    sample::SamplePlayer,
};

use crate::{
    MusicAssets,
    player::{ItemPosition, Player},
    screens::Screen,
    supernova::Nova,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Gameplay), spawn_background_music);
    app.add_systems(
        Update,
        update_volume_based_on_proximity.run_if(in_state(Screen::Gameplay)),
    );
}

#[derive(PoolLabel, PartialEq, Eq, Debug, Hash, Clone)]
struct BackgroundNoiseMarker;

fn spawn_background_music(mut commands: Commands, music: Res<MusicAssets>) {
    info!("Spawning background music");
    commands.spawn((
        SamplerPool(BackgroundNoiseMarker),
        VolumeNode {
            volume: Volume::Linear(0.0),
        },
    ));

    commands.spawn((
        Name::new("Background noise"),
        StateScoped(Screen::Gameplay),
        BackgroundNoiseMarker,
        SamplePlayer::new(music.sun_proximity.clone()).looping(),
    ));
}

fn update_volume_based_on_proximity(
    nova: Res<State<Nova>>,
    player: Single<&ItemPosition, With<Player>>,
    mut volume: Single<&mut VolumeNode, With<SamplerPool<BackgroundNoiseMarker>>>,
) {
    match **nova {
        Nova::Idle | Nova::BuildingUp => {
            let radius = player.radius;
            let new_volume = (1.0 - (radius / 500.0)).clamp(0.0, 1.0) / 2.0;
            volume.volume = Volume::Linear(new_volume);
        }
        Nova::During => {
            if volume.volume.linear() <= 0.0 {
                return;
            }

            volume.volume = decrement_volume(volume.volume, 0.01);
        }
        Nova::After => {
            // nop
        }
    }
}

fn decrement_volume(volume: Volume, step: f32) -> Volume {
    Volume::Linear((volume.linear() - step).max(0.0))
}
