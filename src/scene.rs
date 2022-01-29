use bevy::prelude::*;


pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(scene);
    }
}

fn scene(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    // Background
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("grid-960x540.png"),
        ..Default::default()
    });

    // Conveyor belt
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.1, 0.2, 0.3),
            ..Default::default()
        },
        transform: Transform {
            scale: Vec3::new(960., 90., 1.),
            translation: Vec3::new(0., -225., 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}
