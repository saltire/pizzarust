use bevy::prelude::*;
use bevy::window::{WindowMode, WindowResizeConstraints};

mod bounce;
mod constants;
mod cursor;
mod cycle;
mod display;
mod font;
mod scene;

use constants::*;


fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            resize_constraints: WindowResizeConstraints {
                min_width: PIXEL_WIDTH,
                min_height: PIXEL_HEIGHT,
                ..Default::default()
            },
            title: "Pizza".into(),
            cursor_visible: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .add_startup_system(init_cameras)
        .add_system(keys)
        .add_plugins(DefaultPlugins)
        .add_plugin(bounce::BouncePlugin)
        .add_plugin(cursor::CursorPlugin)
        .add_plugin(cycle::CyclePlugin)
        .add_plugin(display::DisplayPlugin)
        .add_plugin(font::FontPlugin)
        .add_plugin(scene::ScenePlugin)
        .run();
}

fn init_cameras(
    mut commands: Commands,
) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn keys(keys: Res<Input<KeyCode>>) {
    if keys.just_released(KeyCode::Escape) {
        std::process::exit(0);
    }
}
