use bevy::{
    color::palettes::tailwind::{AMBER_400, EMERALD_400},
    prelude::*,
    render::render_resource::AsBindGroup,
};

use crate::{
    Pause,
    player::{PlayerHeat, PlayerPower},
    screens::Screen,
};

const SHADER_ASSET_PATH: &str = "shaders/power_bar.wgsl";

pub(super) fn plugin(app: &mut App) {
    app.register_type::<PowerBarParentMarker>();
    app.register_type::<PowerBarMarker>();

    app.add_plugins(UiMaterialPlugin::<UiProgressBarMaterial>::default());

    app.add_systems(
        OnEnter(Screen::Gameplay),
        (spawn_power_bar, spawn_health_bar),
    );

    app.add_systems(
        Update,
        (
            update_bar::<PowerBarMarker, PlayerPower>,
            update_bar::<HealthBarMarker, PlayerHeat>,
        )
            .run_if(in_state(Screen::Gameplay).and(in_state(Pause(false)))),
    );
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct UiProgressBarMaterial {
    #[uniform(0)]
    color: Vec4,
    #[uniform(1)]
    slider: Vec4,
    #[uniform(2)]
    border_color: Vec4,
}

impl UiMaterial for UiProgressBarMaterial {
    fn fragment_shader() -> bevy::render::render_resource::ShaderRef {
        SHADER_ASSET_PATH.into()
    }
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerBarParentMarker;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct PowerBarMarker;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HealthBarParentMarker;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct HealthBarMarker;

fn spawn_power_bar(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<UiProgressBarMaterial>>,
    power_bars: Query<Entity, With<PowerBarParentMarker>>,
) {
    for entity in &power_bars {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(12.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        PowerBarParentMarker,
        StateScoped(Screen::Gameplay),
        children![
            Text::new("POWER "),
            (
                PowerBarMarker,
                Node {
                    width: Val::Px(250.0 - 12.0),
                    height: Val::Px(30.0 - 12.0),
                    border: UiRect::all(Val::Px(3.0)),
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                MaterialNode(ui_materials.add(UiProgressBarMaterial {
                    color: EMERALD_400.to_vec4(),
                    slider: Vec4::splat(0.4),
                    border_color: LinearRgba::WHITE.to_vec4(),
                })),
                BorderRadius::all(Val::Px(3.0)),
            )
        ],
    ));
}

fn spawn_health_bar(
    mut commands: Commands,
    mut ui_materials: ResMut<Assets<UiProgressBarMaterial>>,
    heat_bars: Query<Entity, With<HealthBarParentMarker>>,
) {
    for entity in &heat_bars {
        commands.entity(entity).despawn();
    }

    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(40.0),
            right: Val::Px(12.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        PowerBarParentMarker,
        StateScoped(Screen::Gameplay),
        children![
            Text::new("HEAT "),
            (
                HealthBarMarker,
                Node {
                    width: Val::Px(250.0 - 12.0),
                    height: Val::Px(30.0 - 12.0),
                    border: UiRect::all(Val::Px(3.0)),
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                MaterialNode(ui_materials.add(UiProgressBarMaterial {
                    color: AMBER_400.to_vec4(),
                    slider: Vec4::splat(0.4),
                    border_color: LinearRgba::WHITE.to_vec4(),
                })),
                BorderRadius::all(Val::Px(3.0)),
            )
        ],
    ));
}

pub trait BarDataSource {
    fn current_frac(&self) -> f32;
}

fn update_bar<M, S>(
    mut materials: ResMut<Assets<UiProgressBarMaterial>>,
    source: Single<&S>,
    bar: Single<&MaterialNode<UiProgressBarMaterial>, With<M>>,
) where
    M: Component,
    S: Component + BarDataSource,
{
    if let Some(material) = materials.get_mut(*bar) {
        material.slider.x = source.current_frac();
    }
}
