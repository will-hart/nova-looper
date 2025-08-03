use bevy::prelude::*;
use bevy_polyline::prelude::{Polyline, PolylineHandle};

use crate::{player::Player, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_tail.run_if(in_state(Screen::Gameplay)));
}

fn update_tail(
    mut polylines: ResMut<Assets<Polyline>>,
    tail: Single<&mut PolylineHandle>,
    player: Single<&Transform, With<Player>>,
) {
    if let Some(tail) = polylines.get_mut(&tail.0) {
        for idx in (1..tail.vertices.len() - 1).rev() {
            tail.vertices[idx] = tail.vertices[idx - 1];
        }

        tail.vertices[0] = player.translation;
    }
}
