use bevy::prelude::*;
use bevy_seedling::sample::Sample;

use crate::asset_tracking::LoadResource;

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PlayerAssets>();
    app.load_resource::<PlayerAssets>();
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct PlayerAssets {
    #[dependency]
    pub(super) rocket_trail_particle: Handle<Image>,
    #[dependency]
    pub(super) shield_alert: Handle<Sample>,
}

impl FromWorld for PlayerAssets {
    fn from_world(world: &mut World) -> Self {
        let assets = world.resource::<AssetServer>();
        Self {
            rocket_trail_particle: assets.load("particles/circle.png"),
            shield_alert: assets.load("audio/sound_effects/shield_alert.ogg"),
        }
    }
}
