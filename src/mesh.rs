use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle},
};


pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(spawn_mesh);
    }
}

fn spawn_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let image: Handle<Image> = asset_server.load("tron.png");

    commands.spawn_bundle(MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
        transform: Transform {
            scale: Vec3::new(100., 100., 1.),
            translation: Vec3::new(0., 0., 1.),
            ..Default::default()
        },
        material: materials.add(ColorMaterial::from(image)),
        ..Default::default()
    });
}
