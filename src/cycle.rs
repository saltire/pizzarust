use bevy::{
    core::Time,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset, RenderAssets},
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        RenderApp, RenderStage,
    },
    sprite::{Material2d, Material2dPipeline, Material2dPlugin, MaterialMesh2dBundle},
};


pub struct CyclePlugin;

impl Plugin for CyclePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugin(Material2dPlugin::<CycleMaterial>::default());
            // .add_startup_system(spawn_mesh);

        let render_device = app.world.get_resource::<RenderDevice>().unwrap();
        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("Time Elapsed Buffer"),
            size: std::mem::size_of::<f32>() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        // Create the buffer for elapsed time, and set up systems to update it every frame.
        app.sub_app_mut(RenderApp)
            .insert_resource(ElapsedBuffer {
                buffer,
            })
            .add_system_to_stage(RenderStage::Extract, extract_time)
            .add_system_to_stage(RenderStage::Prepare, prepare_time_buffer);
    }
}

fn _spawn_mesh(
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

// The material needs a type uuid in order to be used as an asset, and thus a render asset.
#[derive(Debug, Clone, TypeUuid)]
#[uuid = "7db45bd1-8b6e-4c11-bbdc-c5836a645e37"]
pub struct CycleMaterial {
    pub image: Option<Handle<Image>>,
}

// Time data to be extracted into the render world.
struct Elapsed {
    seconds: f32,
}

// Take the time resource from the app world and save the elapsed time into the render world.
fn extract_time(
    mut commands: Commands,
    time: Res<Time>,
) {
    commands.insert_resource(Elapsed {
        seconds: time.seconds_since_startup() as f32,
    });
}

// A buffer for time data to pass to the shader.
pub struct ElapsedBuffer {
    buffer: Buffer,
}

// Take the extracted elapsed time and write it into the buffer used by our material's bind group.
fn prepare_time_buffer(
    render_queue: Res<RenderQueue>,
    elapsed_buffer: ResMut<ElapsedBuffer>,
    elapsed: Res<Elapsed>,
) {
    render_queue.write_buffer(&elapsed_buffer.buffer, 0,
        bevy::core::cast_slice(&[elapsed.seconds]));
}

// Everything the GPU needs to know to render our material.
pub struct PreparedCycleMaterial {
    pub bind_group: BindGroup,
}

impl RenderAsset for CycleMaterial {
    type ExtractedAsset = CycleMaterial;
    type PreparedAsset = PreparedCycleMaterial;
    type Param = (
        SRes<RenderDevice>,
        SRes<Material2dPipeline<CycleMaterial>>,
        SRes<RenderAssets<Image>>,
        SRes<ElapsedBuffer>,
    );

    // Extract the material into the render world as is.
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    // Prepare the bind group for the material. This should only run once after the image is loaded.
    fn prepare_asset(
        material: Self::ExtractedAsset,
        (render_device, pipeline, gpu_images, elapsed_buffer): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let (texture_view, sampler) = if let Some(result) = pipeline
            .mesh2d_pipeline
            .get_image_texture(gpu_images, &material.image)
        {
            result
        } else {
            return Err(PrepareAssetError::RetryNextUpdate(material));
        };

        Ok(PreparedCycleMaterial {
            bind_group: render_device.create_bind_group(&BindGroupDescriptor {
                label: Some("Cycle Material Bind Group"),
                entries: &[
                    // Texture
                    BindGroupEntry {
                        binding: 0,
                        resource: BindingResource::TextureView(texture_view),
                    },
                    // Texture sampler
                    BindGroupEntry {
                        binding: 1,
                        resource: BindingResource::Sampler(sampler),
                    },
                    // Time elapsed
                    BindGroupEntry {
                        binding: 2,
                        resource: elapsed_buffer.buffer.as_entire_binding(),
                    },
                ],
                layout: &pipeline.material2d_layout,
            }),
        })
    }
}

impl Material2d for CycleMaterial {
    // Assign the fragment shader for the material.
    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("cycle.wgsl"))
    }

    // Get the bind group from the prepared material.
    fn bind_group(material: &Self::PreparedAsset) -> &BindGroup {
        &material.bind_group
    }

    // Define the layout for this material's bind group.
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
                // Texture sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Sampler(SamplerBindingType::Filtering),
                    count: None,
                },
                // Time elapsed
                BindGroupLayoutEntry {
                    binding: 2,
                    visibility: ShaderStages::FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
                    },
                    count: None,
                },
            ],
        })
    }
}
