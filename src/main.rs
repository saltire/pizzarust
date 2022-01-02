use bevy::prelude::*;
use bevy::window::WindowMode;

mod constants;
mod cursor;
mod scene;

use constants::*;


fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Pizza".to_string(),
            cursor_visible: false,
            mode: WindowMode::Fullscreen {
                use_size: false,
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(scene::ScenePlugin)
        .add_plugin(cursor::CursorPlugin)
        .add_startup_system(initialize.system().label("init"))
        .run();
}

#[derive(Debug)]
pub struct Display {
    scale: f32,
    window_size: Vec2,
}

fn initialize(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
) {
    if let Some(window) = windows.get_primary_mut() {
        let window_size = Vec2::new(window.width(), window.height());
        let scale_x = window_size.x / PIXEL_WIDTH;
        let scale_y = window_size.y / PIXEL_HEIGHT;
        let scale = scale_x.min(scale_y).floor();

        window.set_scale_factor_override(Some(scale.into()));

        commands.spawn().insert(Display {
            scale,
            window_size,
        });
    }

    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}
