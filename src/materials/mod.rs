use bevy::prelude::*;

mod starfield;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(starfield::plugin);
}
