use bevy::{
    prelude::*,
    reflect::TypeUuid,
    render::render_resource::*,
    sprite::{Material2d, Material2dPlugin, MaterialMesh2dBundle},
};


pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Material2dPlugin::<CycleMaterial>::default())
            // .add_startup_system(_spawn_test_mesh)
            .add_system(update_material_elapsed_seconds);
    }
}

// The material needs a type uuid in order to be used as an asset, and thus a render asset.
#[derive(AsBindGroup, Clone, Debug, Default, TypeUuid)]
#[uuid = "7db45bd1-8b6e-4c11-bbdc-c5836a645e37"]
pub struct CycleMaterial {
    #[uniform(0)]
    pub elapsed_seconds: f32,
    #[texture(1)]
    #[sampler(2)]
    pub image: Option<Handle<Image>>,
}

impl Material2d for CycleMaterial {
    fn fragment_shader() -> ShaderRef {
        "cycle.wgsl".into()
    }
}

fn update_material_elapsed_seconds(
    mut materials: ResMut<Assets<CycleMaterial>>,
    time: Res<Time>,
) {
    for (_, material) in materials.iter_mut() {
        material.elapsed_seconds = time.seconds_since_startup() as f32;
    }
}

// Testing system: spawn an image as a mesh and assign the material.
fn _spawn_test_mesh(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CycleMaterial>>,
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
        material: materials.add(CycleMaterial {
            image: Some(image),
            ..Default::default()
        }),
        ..Default::default()
    });
}
