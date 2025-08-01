use bevy::prelude::*;

mod starfield;
mod sun;

pub use sun::SunMaterial;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((starfield::plugin, sun::plugin));
}
