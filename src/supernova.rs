use bevy::{
    color::palettes::css::{RED, WHITE},
    prelude::*,
};
use bevy_seedling::sample::SamplePlayer;

#[cfg(debug_assertions)]
pub use bevy::dev_tools::states::log_transitions;

use crate::{
    MusicAssets, PlayerAssets,
    consts::{INNER_SUN_COLOUR, SPLASH_BACKGROUND_COLOR, SUN_COLOUR},
    materials::{StarfieldMaterial, SunMaterial},
    player::Player,
    screens::Screen,
};

const IDLE_PHASE: f32 = 30.0;
const BUILD_PHASE: f32 = 6.0;
const DURING_PHASE: f32 = 10.0;
const AFTER_PHASE: f32 = 4.0;

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<Nova>();
    #[cfg(debug_assertions)]
    app.add_systems(Update, log_transitions::<Nova>);
    app.add_systems(
        Update,
        tick_nova_timer.run_if(resource_exists::<NovaTimer>.and(state_exists::<Nova>)),
    );
    app.add_systems(OnExit(Screen::Gameplay), destroy_timer);

    app.add_systems(OnEnter(Nova::Idle), on_start_idle);
    app.add_systems(OnExit(Nova::Idle), on_finish_idle);

    app.add_systems(OnEnter(Nova::BuildingUp), on_start_buildup);
    app.add_systems(Update, during_buildup.run_if(in_state(Nova::BuildingUp)));
    app.add_systems(OnExit(Nova::BuildingUp), on_finish_buildup);

    app.add_systems(OnEnter(Nova::During), on_start_during);
    app.add_systems(OnExit(Nova::During), on_finish_during);

    app.add_systems(OnEnter(Nova::After), on_start_after);
    app.add_systems(Update, during_after.run_if(in_state(Nova::After)));
    app.add_systems(OnExit(Nova::After), on_finish_after);
}

#[derive(SubStates, Copy, Clone, Eq, PartialEq, Hash, Debug, Default)]
#[source(Screen = Screen::Gameplay)]
#[states(scoped_entities)]
pub enum Nova {
    /// The default state where we are skipping the star
    #[default]
    Idle,
    /// When the nova is started but we haven't yet left the star
    BuildingUp,
    /// While the nova is happening and we're travelling to the next star
    During,
    /// While we're arriving at the next star, before we start skimming
    After,
}

impl Nova {
    fn next_state(&self) -> Self {
        match self {
            Nova::Idle => Nova::BuildingUp,
            Nova::BuildingUp => Nova::During,
            Nova::During => Nova::After,
            Nova::After => Nova::Idle,
        }
    }
}

/* SHARED */

fn tick_nova_timer(
    time: Res<Time>,
    mut timer: ResMut<NovaTimer>,
    state: Res<State<Nova>>,
    mut next_nova_state: ResMut<NextState<Nova>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        info!("Nova timer pinged!");
        next_nova_state.set(state.next_state());
    }
}

fn destroy_timer(mut commands: Commands) {
    commands.remove_resource::<NovaTimer>();
}

/* NOVA IDLE */

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct NovaTimer(Timer);

fn on_start_idle(mut commands: Commands) {
    commands.insert_resource(NovaTimer(Timer::from_seconds(IDLE_PHASE, TimerMode::Once)));
}

fn on_finish_idle() {}

/* NOVA BUILDING UP */

fn on_start_buildup(
    mut commands: Commands,
    player_assets: Res<PlayerAssets>,
    music_assets: Res<MusicAssets>,
) {
    commands.spawn(SamplePlayer::new(player_assets.nova_alert.clone()));
    commands.insert_resource(NovaTimer(Timer::from_seconds(BUILD_PHASE, TimerMode::Once)));
    commands.spawn(SamplePlayer::new(music_assets.supernova.clone()));

    commands.spawn((
        StateScoped(Nova::BuildingUp),
        Node {
            position_type: PositionType::Absolute,
            top: Val::ZERO,
            left: Val::ZERO,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        children![(
            Node {
                position_type: PositionType::Relative,
                top: Val::Px(45.0),
                ..default()
            },
            Text::new("NOVA ALERT - AUTOPILOT ON"),
            TextLayout {
                justify: JustifyText::Center,
                ..default()
            },
            TextFont {
                font_size: 12.0,
                ..default()
            },
            TextColor(WHITE.into()),
        )],
    ));
}

fn during_buildup(
    timer: Res<NovaTimer>,
    mut starfield_mats: ResMut<Assets<StarfieldMaterial>>,
    mut player_mats: ResMut<Assets<ColorMaterial>>,
    mut sun_mats: ResMut<Assets<SunMaterial>>,
    starfield: Single<&mut MeshMaterial2d<StarfieldMaterial>>,
    player: Single<&mut MeshMaterial2d<ColorMaterial>, With<Player>>,
    sun: Single<&mut MeshMaterial2d<SunMaterial>>,
) {
    let counter = 1.0 - 2.0 * timer.0.fraction_remaining();

    let starfield_col = SPLASH_BACKGROUND_COLOR.mix(&Color::Srgba(WHITE), counter);
    let player_col = Color::Srgba(WHITE).mix(&Color::Srgba(RED), counter);
    let sun_outer = SUN_COLOUR.mix(&Color::Srgba(WHITE), counter);
    let sun_inner = INNER_SUN_COLOUR.mix(&Color::Srgba(WHITE), counter);

    if let Some(material) = starfield_mats.get_mut(&starfield.0) {
        material.background = starfield_col.into();
    }

    if let Some(material) = player_mats.get_mut(&player.0) {
        material.color = player_col;
    }

    if let Some(material) = sun_mats.get_mut(&sun.0) {
        material.inner_color = sun_inner.to_srgba().to_vec4();
        material.color = sun_outer.to_srgba().to_vec4();
    }
}

fn on_finish_buildup() {}

/* NOVA DURING */

fn on_start_during(mut commands: Commands) {
    commands.insert_resource(NovaTimer(Timer::from_seconds(
        DURING_PHASE,
        TimerMode::Once,
    )));
}

fn on_finish_during() {
    //
}

/* NOVA POST */

fn on_start_after(mut commands: Commands) {
    commands.insert_resource(NovaTimer(Timer::from_seconds(AFTER_PHASE, TimerMode::Once)));
}

fn during_after(
    timer: Res<NovaTimer>,
    mut starfield_mats: ResMut<Assets<StarfieldMaterial>>,
    mut player_mats: ResMut<Assets<ColorMaterial>>,
    mut sun_mats: ResMut<Assets<SunMaterial>>,
    starfield: Single<&mut MeshMaterial2d<StarfieldMaterial>>,
    player: Single<&mut MeshMaterial2d<ColorMaterial>, With<Player>>,
    sun: Single<&mut MeshMaterial2d<SunMaterial>>,
) {
    let counter = 1.0 - timer.0.fraction_remaining();

    let starfield_col = WHITE.mix(&SPLASH_BACKGROUND_COLOR.to_srgba(), counter);
    let player_col = Color::Srgba(RED).mix(&Color::Srgba(WHITE), counter);
    let sun_outer = WHITE.mix(&SUN_COLOUR.to_srgba(), counter);

    let sun_inner = WHITE.mix(&INNER_SUN_COLOUR.to_srgba(), counter);

    if let Some(material) = starfield_mats.get_mut(&starfield.0) {
        material.background = starfield_col.into();
    }

    if let Some(material) = player_mats.get_mut(&player.0) {
        material.color = player_col;
    }

    if let Some(material) = sun_mats.get_mut(&sun.0) {
        material.inner_color = sun_inner.to_vec4();
        material.color = sun_outer.to_vec4();
    }
}

fn on_finish_after() {
    //
}
