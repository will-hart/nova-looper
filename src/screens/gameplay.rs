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
struct SunProximityPool;

#[derive(PoolLabel, PartialEq, Eq, Debug, Hash, Clone)]
struct SkimmingSunPool;

fn spawn_background_music(mut commands: Commands, music: Res<MusicAssets>) {
    info!("Spawning background music");
    commands.spawn((
        SamplerPool(SunProximityPool),
        VolumeNode {
            volume: Volume::Linear(0.0),
        },
    ));
    commands.spawn((
        SamplerPool(SkimmingSunPool),
        VolumeNode {
            volume: Volume::Linear(0.0),
        },
    ));

    commands.spawn((
        Name::new("Proximity noise"),
        StateScoped(Screen::Gameplay),
        SunProximityPool,
        SamplePlayer::new(music.sun_proximity.clone()).looping(),
    ));

    commands.spawn((
        Name::new("Skimming noise"),
        StateScoped(Screen::Gameplay),
        SkimmingSunPool,
        SamplePlayer::new(music.sun_proximity.clone()).looping(),
    ));
}

fn update_volume_based_on_proximity(
    nova: Res<State<Nova>>,
    player: Single<&ItemPosition, With<Player>>,
    mut prox_volume: Single<
        &mut VolumeNode,
        (
            With<SamplerPool<SunProximityPool>>,
            Without<SamplerPool<SkimmingSunPool>>,
        ),
    >,
    mut skimming_volume: Single<
        &mut VolumeNode,
        (
            With<SamplerPool<SkimmingSunPool>>,
            Without<SamplerPool<SunProximityPool>>,
        ),
    >,
) {
    match **nova {
        Nova::Idle | Nova::BuildingUp => {
            let radius = player.radius;
            let new_volume = (1.0 - (radius / 500.0)).clamp(0.0, 1.0) / 2.0;
            prox_volume.volume = Volume::Linear(new_volume);

            skimming_volume.volume = Volume::Linear(if radius < 5.0 { 0.65 } else { 0.0 });
        }
        Nova::During => {
            skimming_volume.volume = Volume::Linear(0.0);
            if prox_volume.volume.linear() <= 0.0 {
                return;
            }

            prox_volume.volume = decrement_volume(prox_volume.volume, 0.01);
        }
        Nova::After => {
            skimming_volume.volume = Volume::Linear(0.0);
        }
    }
}

fn decrement_volume(volume: Volume, step: f32) -> Volume {
    Volume::Linear((volume.linear() - step).max(0.0))
}
