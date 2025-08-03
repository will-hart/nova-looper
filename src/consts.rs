use bevy::color::Color;

// The clear colour
pub const SPLASH_BACKGROUND_COLOR: Color = Color::srgb(0.057, 0.057, 0.057);

/// The speed the player starts with
pub const PLAYER_STARTING_SPEED: f32 = 0.45;

/// The radius of the sun at the start
pub const SUN_STARTING_RADIUS: f32 = 1000.0;

/// The random fudge factor for movement speed inwards and outwards
/// along the radius
pub const PLAYER_RADIUS_CHANGE_SPEED: f32 = 180.0;

/// The random fudge factor that makes controls feel more responsive.
/// This controls how fast the movement direction changes
pub const MAGIC_MOVEMENT_ACCEL_SCALE: f32 = 14.0;

/// The maximum radius a player can have relateive the centre of the sun
pub const MAX_PLAYER_RADIUS: f32 = 800.0;

/// The main theme colour
pub const MAIN_THEME_COLOR: Color = Color::hsla(21.0, 0.936, 0.51, 1.0);

/// The colour of the middle of the sun
pub const INNER_SUN_COLOUR: Color = Color::srgba(3.968, 0.372, 0.051, 1.0);
/// The colour of the sun
pub const SUN_COLOUR: Color = Color::srgba(1.868, 0.522, 0.021, 1.0);

/// The rate the score increases each second
pub const SCORE_INCREASE_RATE: f32 = 7.0;

/// The cost to shields of hitting an obstacle
pub const SHIELD_COST_ON_OBSTACLE_HIT: f32 = 30.0;
