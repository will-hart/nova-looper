//! The main menu (seen on the title screen).

use bevy::prelude::*;

use crate::{asset_tracking::ResourceHandles, menus::Menu, screens::Screen, theme::widget};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        OnEnter(Menu::Main),
        (spawn_main_menu, spawn_music, despawn_score),
    );
}

fn despawn_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

}

fn spawn_main_menu(mut commands: Commands) {
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
            flex_direction: FlexDirection::Row,
            justify_content: JustifyContent::Center,
            align_items: AlignItems::FlexEnd,
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

// fn open_settings_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
//     next_menu.set(Menu::Settings);
// }

fn open_credits_menu(_: Trigger<Pointer<Click>>, mut next_menu: ResMut<NextState<Menu>>) {
    next_menu.set(Menu::Credits);
}

#[cfg(not(target_family = "wasm"))]
fn exit_app(_: Trigger<Pointer<Click>>, mut app_exit: EventWriter<AppExit>) {
    app_exit.write(AppExit::Success);
}
