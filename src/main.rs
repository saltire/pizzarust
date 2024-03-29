use bevy::prelude::*;
use bevy::render::texture::ImageSettings;
use bevy::window::{WindowMode, WindowResizeConstraints};

mod bounce;
mod constants;
mod cursor;
mod cycle;
mod display;
mod font;
mod scene;

use constants::*;


#[derive(Component)]
pub struct MainCamera;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            width: INITIAL_SIZE_X,
            height: INITIAL_SIZE_Y,
            resize_constraints: WindowResizeConstraints {
                min_width: 320.,
                min_height: 180.,
                ..Default::default()
            },
            title: "Pizza".into(),
            cursor_visible: false,
            mode: WindowMode::Windowed,
            ..Default::default()
        })
        .insert_resource(ImageSettings::default_nearest())
        .add_startup_system_to_stage(StartupStage::PreStartup, init_cameras)
        .add_system(bevy::window::close_on_esc)
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
    commands.spawn_bundle(Camera2dBundle::default()).insert(MainCamera);
}
