//! The main menu (seen on the title screen).

use bevy::prelude::*;
use bevy_seedling::prelude::*;

use crate::{
    MusicAssets, asset_tracking::ResourceHandles, consts::SUN_STARTING_RADIUS,
    materials::SunMaterial, menus::Menu, player::ItemPosition, score::Score, screens::Screen,
    sun::Sun, theme::widget, utils::Rotate,
};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Main),
        (spawn_main_menu, spawn_music, despawn_score),
    );
}

fn despawn_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

fn spawn_music(mut commands: Commands, music_assets: Res<MusicAssets>) {
    commands.spawn((
        SamplePlayer::new(music_assets.menu.clone())
            .looping()
            .with_volume(Volume::Linear(0.3)),
        StateScoped(Screen::Title),
    ));
}

fn spawn_main_menu(
    mut commands: Commands,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<SunMaterial>>,
) {
    commands.spawn((
        Name::new("Main Menu"),
        Pickable::IGNORE,
        GlobalZIndex(2),
        StateScoped(Menu::Main),
        Node {
            position_type: PositionType::Absolute,
            top: Val::ZERO,
            left: Val::ZERO,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(20.0)),
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            align_items: AlignItems::Center,
            ..default()
        },
        children![
            Text::new(":::Welcome to NOVA LOOPER:::\nRun close to the sun to collect power, watch out for obstacles and keep your shields above 0.\nUse space, mouse or tap to control."),
            (
                Node   {
                    width: Val::Percent(100.0),
                    flex_direction:FlexDirection::Row,
                    justify_content: JustifyContent::Center,
                    column_gap: Val::Px(20.0),
                    ..default()
                },

                #[cfg(not(target_family = "wasm"))]
                children![
                    widget::menu_button("Play", enter_loading_or_gameplay_screen),
                    // widget::menu_button("Settings", open_settings_menu),
                    widget::menu_button("Credits", open_credits_menu),
                    widget::menu_button("Exit", exit_app),
                ],
                #[cfg(target_family = "wasm")]
                children![
                    widget::menu_button("Play", enter_loading_or_gameplay_screen),
                    // widget::menu_button("Settings", open_settings_menu),
                    widget::menu_button("Credits", open_credits_menu),
                ],
            )
        ]
    ));

    // spawn a random sun and orbiting player for interest
    let sun = Sun::default();
    let mesh = meshes.add(Rectangle::new(2.0 * sun.radius, 2.0 * sun.radius));
    let player_mesh = meshes.add(Triangle2d::new(
        Vec2::Y * 10.0,
        Vec2::new(-5.0, -5.0),
        Vec2::new(5.0, -5.0),
    ));
    let color = Color::hsl(142.0, 0.95, 0.97);

    commands.spawn((
        Mesh2d(mesh),
        MeshMaterial2d(materials.add(SunMaterial::default())),
        StateScoped(Menu::Main),
        Transform::from_translation(Vec3::new(0.0, 0.0, -1.0)).with_scale(Vec3::splat(0.2)),
        sun,
        Rotate(-0.5),
        children![(
            Mesh2d(player_mesh),
            MeshMaterial2d(color_materials.add(color)),
            Transform::from_xyz(1.1 * SUN_STARTING_RADIUS, 0.0, 0.1).with_scale(Vec3::splat(-4.0)),
            ItemPosition::default(),
        )],
    ));
}

fn enter_loading_or_gameplay_screen(
    _: Trigger<Pointer<Click>>,
    resource_handles: Res<ResourceHandles>,
    mut next_screen: ResMut<NextState<Screen>>,
) {
    if resource_handles.is_all_done() {
        next_screen.set(Screen::Gameplay);
    } else {
        next_screen.set(Screen::Loading);
    }
}

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
