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
    commands.spawn_bundle(SpriteBundle {
        texture: asset_server.load("grid-480x270.png"),
        ..Default::default()
    });
}
