use bevy::color::Color;

/// The speed the player starts with
pub const PLAYER_STARTING_SPEED: f32 = 0.4;

/// The radius of the sun at the start
pub const SUN_STARTING_RADIUS: f32 = 1000.0;

/// The random fudge factor for movement speed
pub const MAGIC_MOVEMENT_SCALE: f32 = 150.0;

/// The random fudge factor that makes controls feel more responsive
pub const MAGIC_MOVEMENT_ACCEL_SCALE: f32 = 12.0;

/// The maximum radius a player can have relateive the centre of the sun
pub const MAX_PLAYER_RADIUS: f32 = 800.0;

/// The colour of the sun
pub const SUN_COLOUR: Color = Color::hsla(21.0, 0.936, 0.51, 1.0);

/// The rate the score increases each second
pub const SCORE_INCREASE_RATE: f32 = 7.0;
