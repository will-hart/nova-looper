use bevy::prelude::*;

mod power_bar;
mod starfield;
mod sun;

pub use power_bar::BarDataSource;
pub use sun::SunMaterial;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((power_bar::plugin, starfield::plugin, sun::plugin));
}
