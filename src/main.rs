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
        .add_startup_system(initial_size.system())
        .run();
}

#[derive(Debug)]
pub struct Display {
    scale: f32,
    offset: Vec2,
}

fn initial_size(mut commands: Commands, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        let width = window.width();
        let height = window.height();
        let scale_x = width / PIXEL_WIDTH;
        let scale_y = height / PIXEL_HEIGHT;
        let scale = scale_x.min(scale_y).floor();

        commands.spawn().insert(Display {
            scale,
            offset: Vec2::new(
                (width - PIXEL_WIDTH * scale) / 2.,
                (height - PIXEL_HEIGHT * scale) / 2.,
            ),
        });

        window.set_scale_factor_override(Some(scale.into()));
    }
}
