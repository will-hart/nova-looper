use bevy::{color::palettes::css::RED, prelude::*};
use bevy_seedling::sample::SamplePlayer;

use crate::{
    PlayerAssets,
    consts::SCORE_INCREASE_RATE,
    materials::BarDataSource,
    player::PlayerPower,
    screens::Screen,
    supernova::Nova,
    utils::{self, DestroyAt, MoveInDirection, ScaleTextOverTime},
};

pub(super) fn plugin(app: &mut App) {
    app.register_type::<Score>();
    app.register_type::<ScoreMarker>();
    app.register_type::<MultiplierMarker>();

    app.add_systems(OnEnter(Screen::Gameplay), setup_score);

    app.add_systems(
        Update,
        (
            increase_multiplier.run_if(in_state(Nova::Idle)),
            increase_score,
            update_score_text,
            update_multiplier_text,
        )
            .distributive_run_if(resource_exists::<Score>),
    );
}

#[derive(Resource, Debug, Reflect)]
#[reflect(Resource)]
pub struct Score {
    pub score: f32,
    pub multiplier: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0.0,
            multiplier: 1,
        }
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct ScoreMarker;
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MultiplierMarker;

fn setup_score(mut commands: Commands) {
    commands.insert_resource(Score::default());

    commands.spawn((
        Text::new("0"),
        TextFont {
            font_size: 32.0,
            ..default()
        },
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(60.0),
            right: Val::Px(10.0),
            ..default()
        },
        ScoreMarker,
        StateScoped(Screen::Gameplay),
    ));

    commands.spawn((
        Text::new("0x"),
        TextFont {
            font_size: 24.0,
            ..default()
        },
        TextColor(RED.into()),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(90.0),
            right: Val::Px(10.0),
            ..default()
        },
        MultiplierMarker,
        StateScoped(Screen::Gameplay),
    ));
}

fn update_score_text(score: Res<Score>, mut text: Single<&mut Text, With<ScoreMarker>>) {
    text.0 = utils::format_number(score.score);
}

fn update_multiplier_text(score: Res<Score>, mut text: Single<&mut Text, With<MultiplierMarker>>) {
    text.0 = format!("{:}x", score.multiplier);
}

fn increase_multiplier(
    mut commands: Commands,
    time: Res<Time>,
    player_assets: Res<PlayerAssets>,
    mut score: ResMut<Score>,
    player: Single<(&Transform, &mut PlayerPower)>,
) {
    let (player_tx, mut power) = player.into_inner();

    if power.0 > 99.0 {
        power.0 = 0.0;
        score.multiplier += 1;

        commands.spawn((
            Text2d::new(format!("{}x", score.multiplier)),
            TextFont::from_font_size(32.0),
            Transform::from_translation(player_tx.translation),
            TextColor(Color::Srgba(Srgba::new(3.93, 0.56, 0.08, 1.0))),
            ScaleTextOverTime {
                rate: 24.0,
                max: 64.0,
            },
            MoveInDirection(player_tx.translation.truncate().normalize() * 100.0),
            DestroyAt {
                time: time.elapsed_secs() + 4.0,
            },
        ));

        commands.spawn(SamplePlayer::new(player_assets.multiplier_up.clone()));
    }
}

fn increase_score(mut score: ResMut<Score>, time: Res<Time>, power: Single<&PlayerPower>) {
    let increase =
        time.delta_secs() * SCORE_INCREASE_RATE * power.current_frac() * score.multiplier as f32;
    score.score += increase;
}
