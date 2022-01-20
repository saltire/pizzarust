use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::*,
        renderer::RenderDevice,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
};


pub struct MeshPlugin;

impl Plugin for MeshPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Material2dPlugin::<CycleMaterial>::default())
            .add_startup_system(spawn_mesh);
    }
}

fn spawn_mesh(
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
        }),
        ..Default::default()
    });
}

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "7db45bd1-8b6e-4c11-bbdc-c5836a645e37"]
struct CycleMaterial {
    image: Option<Handle<Image>>,
}

struct GpuCycleMaterial {
    pub bind_group: BindGroup,
}

impl RenderAsset for CycleMaterial {
    type ExtractedAsset = CycleMaterial;
    type PreparedAsset = GpuCycleMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<CycleMaterial>>,
        SRes<RenderAssets<Image>>,
    );

    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, pipeline, gpu_images): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let (texture_view, sampler) = if let Some(result) = pipeline
            .mesh2d_pipeline
            .get_image_texture(gpu_images, &material.image)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        Ok(GpuCycleMaterial {
            bind_group: render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("Cycle Material Bind Group"),
                entries: &[
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(texture_view),
                    },
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(sampler),
                    },
                ],
                layout: &pipeline.material2d_layout,
            }),
        })
    }
}

impl Material2d for CycleMaterial {
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("cycle.wgsl"))
    }

    fn bind_group(material: &Self::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("Cycle Material Bind Group Layout"),
            entries: &[
                // Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Texture {
                        multisampled: false,
                        sample_type: TextureSampleType::Float { filterable: true },
                        view_dimension: TextureViewDimension::D2,
                    },
                    count: None,
                },
                // Texture Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
            ],
        })
    }
}
