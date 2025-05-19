use mylib;

use bevy::{
    prelude::*,
    window::{WindowLevel, WindowResolution},
};

fn main() {
    dioxus_devtools::connect_subsecond();

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                position: WindowPosition::new(IVec2::new(0, 0)),
                resolution: WindowResolution::new(640.0, 880.0),
                window_level: WindowLevel::AlwaysOnTop,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (update_text, mylib::update_text))
        .run();
}

fn update_text(text_query: Query<&mut Text, With<UpdatedFromMain>>) {
    dioxus_devtools::subsecond::HotFn::current(wrapped_main).call((text_query,));
}

fn wrapped_main(mut text_query: Query<&mut Text, With<UpdatedFromMain>>) {
    let mut text = text_query.single_mut().unwrap();
    **text = "from main".to_string();
}

#[derive(Component)]
struct UpdatedFromMain;

pub fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            ..default()
        },
        children![
            (
                Text::default(),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
                UpdatedFromMain
            ),
            (
                Text::default(),
                TextFont {
                    font_size: 100.0,
                    ..default()
                },
                mylib::UpdatedFromCrate
            )
        ],
    ));
}
