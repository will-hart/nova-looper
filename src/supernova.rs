use bevy::{dev_tools::states::log_transitions, prelude::*};
use bevy_seedling::sample::SamplePlayer;

use crate::{MusicAssets, screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<Nova>();
    app.add_systems(Update, log_transitions::<Nova>);

    app.add_systems(Update, tick_nova_timer.run_if(in_state(Nova::Idle)));

    app.add_systems(OnEnter(Nova::Idle), on_start_idle);
    app.add_systems(OnExit(Nova::Idle), on_finish_idle);

    app.add_systems(OnEnter(Nova::BuildingUp), on_start_buildup);
    app.add_systems(OnExit(Nova::BuildingUp), on_finish_buildup);

    app.add_systems(OnEnter(Nova::DuringNova), on_start_during);
    app.add_systems(OnExit(Nova::DuringNova), on_finish_during);

    app.add_systems(OnEnter(Nova::PostNova), on_start_post);
    app.add_systems(OnExit(Nova::PostNova), on_finish_post);
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
    DuringNova,
    /// While we're arriving at the next star, before we start skimming
    PostNova,
}

impl Nova {
    fn next_state(&self) -> Self {
        match self {
            Nova::Idle => Nova::BuildingUp,
            Nova::BuildingUp => Nova::DuringNova,
            Nova::DuringNova => Nova::PostNova,
            Nova::PostNova => Nova::Idle,
        }
    }
}

/** SHARED **/

fn tick_nova_timer(
    time: Res<Time>,
    mut timer: ResMut<NovaTimer>,
    state: Res<State<Nova>>,
    mut next_nova_state: ResMut<NextState<Nova>>,
) {
    timer.0.tick(time.delta());
    if timer.0.just_finished() {
        next_nova_state.set(state.next_state());
    }
}

/** NOVA IDLE **/

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub struct NovaTimer(Timer);

fn on_start_idle(mut commands: Commands) {
    commands.insert_resource(NovaTimer(Timer::from_seconds(30.0, TimerMode::Once)));
}

fn on_finish_idle() {
    //
}

/** NOVA BUILDING UP **/

fn on_start_buildup(mut commands: Commands, music_assets: Res<MusicAssets>) {
    commands.insert_resource(NovaTimer(Timer::from_seconds(18.5, TimerMode::Once)));
    commands.spawn(SamplePlayer::new(music_assets.supernova.clone()));
}

fn on_finish_buildup() {
    //
}

/** NOVA DURING **/

fn on_start_during(mut commands: Commands) {
    commands.insert_resource(NovaTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn on_finish_during() {
    //
}

/** NOVA POST **/

fn on_start_post(mut commands: Commands) {
    commands.insert_resource(NovaTimer(Timer::from_seconds(5.0, TimerMode::Once)));
}

fn on_finish_post() {
    //
}
