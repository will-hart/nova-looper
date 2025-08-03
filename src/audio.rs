use bevy::prelude::*;
use bevy_seedling::sample::{Sample, SamplePlayer};

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Music>();
    app.register_type::<SoundEffect>();

    app.register_type::<MusicAssets>();
    app.load_resource::<MusicAssets>();

    app.add_systems(
        Update,
        apply_global_volume.run_if(resource_changed::<GlobalVolume>),
    );
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct MusicAssets {
    #[dependency]
    pub(super) menu: Handle<Sample>,
    #[dependency]
    pub(super) supernova: Handle<Sample>,
    #[dependency]
    pub(super) sun_proximity: Handle<Sample>,
}

impl FromWorld for MusicAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            menu: assets.load("audio/music/menu.ogg"),
            supernova: assets.load("audio/music/supernova.ogg"),
            sun_proximity: assets.load("audio/music/sun_proximity.ogg"),
        }
    }
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "music" category (e.g. global background music, soundtrack).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Music;

/// A music audio instance.
#[expect(unused)]
pub fn music(handle: Handle<Sample>) -> impl Bundle {
    (SamplePlayer::new(handle).looping(), Music)
}

/// An organizational marker component that should be added to a spawned [`AudioPlayer`] if it's in the
/// general "sound effect" category (e.g. footsteps, the sound of a magic spell, a door opening).
///
/// This can then be used to query for and operate on sounds in that category.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct SoundEffect;

/// A sound effect audio instance.
pub fn sound_effect(handle: Handle<Sample>) -> impl Bundle {
    (SamplePlayer::new(handle), SoundEffect)
}

/// [`GlobalVolume`] doesn't apply to already-running audio entities, so this system will update them.
fn apply_global_volume(
    global_volume: Res<GlobalVolume>,
    mut audio_query: Query<(&PlaybackSettings, &mut AudioSink)>,
) {
    for (playback, mut sink) in &mut audio_query {
        sink.set_volume(global_volume.volume * playback.volume);
    }
}
