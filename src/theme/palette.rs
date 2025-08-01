use bevy::prelude::*;

use crate::consts::SUN_COLOUR;

/// #ddd369
pub const LABEL_TEXT: Color = Color::srgb(0.867, 0.827, 0.412);

/// #fcfbcc
pub const HEADER_TEXT: Color = Color::srgb(0.988, 0.984, 0.800);

/// #ececec
pub const BUTTON_TEXT: Color = Color::srgb(0.925, 0.925, 0.925);
/// #4666bf
pub const BUTTON_BACKGROUND: Color = SUN_COLOUR;
/// #6299d1
pub const BUTTON_HOVERED_BACKGROUND: Color = Color::hsla(21.0, 0.836, 0.45, 1.0);
/// #3d4999
pub const BUTTON_PRESSED_BACKGROUND: Color = Color::hsla(21.0, 0.966, 0.31, 1.0);
