use bevy::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::window::WindowMode;

mod scene;


fn initial_size(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();
    let scale_x = window.width() / 480.;
    let scale_y = window.height() / 270.;
    let scale = scale_x.min(scale_y).floor();
    window.set_scale_factor_override(Some(scale.into()));
}

fn main() {
    App::build()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(WindowDescriptor {
            title: "Pizza".to_string(),
            width: 480.,
            height: 270.,
            mode: WindowMode::Fullscreen {
                use_size: false,
            },
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(scene::ScenePlugin)
        .add_startup_system(initial_size.system())
        .run();
}
