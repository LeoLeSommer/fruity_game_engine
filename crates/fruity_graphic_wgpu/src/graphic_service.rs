use crate::resources::material_resource::InstanceField;
use crate::resources::material_resource::WgpuMaterialResource;
use crate::resources::mesh_resource::WgpuMeshResource;
use crate::resources::shader_resource::WgpuShaderResource;
use crate::resources::texture_resource::WgpuTextureResource;
use crate::utils::encode_into_bytes;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::profile::profile_function;
use fruity_game_engine::profile::profile_scope;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::graphic_service::MaterialParam;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2D;
use fruity_graphic::math::Color;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::MaterialResourceSettings;
use fruity_graphic::resources::mesh_resource::MeshResource;
use fruity_graphic::resources::mesh_resource::MeshResourceSettings;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::shader_resource::ShaderResourceSettings;
use fruity_graphic::resources::texture_resource::TextureResource;
use fruity_graphic::resources::texture_resource::TextureResourceSettings;
use fruity_windows::window_service::WindowService;
use fruity_windows_winit::window_service::WinitWindowService;
use image::load_from_memory;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::iter;
use std::sync::Arc;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform(pub [[f32; 4]; 4]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ViewportSizeUniform(pub [f32; 2]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RenderSurfaceSizeUniform(pub [f32; 2]);

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct DrawIndexedIndirectArgs {
    /// The number of indices to draw.
    pub index_count: u32,
    /// The number of instances to draw.
    pub instance_count: u32,
    /// Offset into the index buffer, in indices, begin drawing from.
    pub first_index: u32,
    /// Added to each index value before indexing into the vertex buffers.
    pub base_vertex: i32,
    /// First instance to draw.
    pub first_instance: u32,
}

#[derive(Debug)]
pub struct State {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,
    pub rendering_view: wgpu::TextureView,
    pub camera_transform: RwLock<Matrix4>,
    pub camera_buffer: wgpu::Buffer,
    pub camera_bind_group: Arc<wgpu::BindGroup>,
    pub viewport_size_buffer: wgpu::Buffer,
    pub viewport_size_bind_group: Arc<wgpu::BindGroup>,
    pub render_surface_size_buffer: wgpu::Buffer,
    pub render_surface_size_bind_group: Arc<wgpu::BindGroup>,
}

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct RenderInstanceIdentifier {
    mesh_identifier: String,
    material_identifier: String,
    z_index: i32,
}

#[derive(Debug)]
struct RenderInstance {
    instance_count: usize,
    instance_buffers: BTreeMap<u64, Vec<u8>>,
    mesh: ResourceReference<dyn MeshResource>,
    material: ResourceReference<dyn MaterialResource>,
}

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct WgpuGraphicService {
    state: Option<State>,
    instance: Arc<wgpu::Instance>,
    surface: Arc<wgpu::Surface>,
    window_service: ResourceReference<dyn WindowService>,
    current_output: Option<wgpu::SurfaceTexture>,
    render_instances: RwLock<BTreeMap<RenderInstanceIdentifier, RenderInstance>>,
    render_bundles: RwLock<Vec<wgpu::RenderBundle>>,
    current_encoder: Option<RwLock<wgpu::CommandEncoder>>,
    viewport_offset: RwLock<(u32, u32)>,
    viewport_size: RwLock<(u32, u32)>,
    _on_resize_handle: ObserverHandler<(u32, u32)>,
}

impl WgpuGraphicService {
    pub fn new(resource_container: ResourceContainer) -> FruityResult<WgpuGraphicService> {
        // Subscribe to windows observer to proceed the graphics when it's neededs
        let on_resize_handle = {
            let window_service = resource_container.require::<dyn WindowService>();
            let window_service = window_service.read();

            let resource_container_2 = resource_container.clone();
            window_service
                .on_resize()
                .add_observer(move |(width, height)| {
                    let graphic_service = resource_container_2.require::<dyn GraphicService>();
                    let mut graphic_service = graphic_service.write();
                    graphic_service.resize(*width, *height);

                    Ok(())
                })
        };

        // Initialize the surface
        let (instance, surface) = {
            let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);

            let dx12_shader_compiler =
                wgpu::util::dx12_shader_compiler_from_env().unwrap_or_default();

            // The instance is a handle to our GPU
            // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends,
                dx12_shader_compiler,
            });

            let window_service = resource_container.require::<dyn WindowService>();
            let window_service_reader = window_service.read();
            let window_service_reader = window_service_reader.downcast_ref::<WinitWindowService>();
            let surface = unsafe { instance.create_surface(window_service_reader.get_window()) }
                .map_err(|err| FruityError::GenericFailure(err.to_string()))?;

            (instance, surface)
        };

        // Dispatch initialized event
        let on_initialized = Signal::new();
        on_initialized.notify(())?;

        let window_service = resource_container.require::<dyn WindowService>();
        Ok(WgpuGraphicService {
            state: None,
            instance: Arc::new(instance),
            surface: Arc::new(surface),
            window_service,
            current_output: None,
            render_instances: RwLock::new(BTreeMap::new()),
            render_bundles: RwLock::new(Vec::new()),
            current_encoder: None,
            viewport_offset: Default::default(),
            viewport_size: Default::default(),
            _on_resize_handle: on_resize_handle,
        })
    }

    pub async fn initialize_async(
        resource_container: ResourceContainer,
        instance: &wgpu::Instance,
        surface: &wgpu::Surface,
    ) -> FruityResult<State> {
        let size = {
            let window_service = resource_container.require::<dyn WindowService>();
            let window_service_reader = window_service.read();
            let window_service_reader = window_service_reader.downcast_ref::<WinitWindowService>();
            window_service_reader.get_window().inner_size()
        };

        let backends = wgpu::util::backend_bits_from_env().unwrap_or_else(wgpu::Backends::all);
        let adapter =
            wgpu::util::initialize_adapter_from_env_or_default(&instance, backends, Some(&surface))
                .await
                .ok_or(FruityError::GenericFailure(
                    "No suitable GPU adapters found on the system!".to_string(),
                ))?;

        // Create the device and queue
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                None,
            )
            .await
            .map_err(|err| FruityError::GenericFailure(err.to_string()))?;

        // Base configuration for the surface
        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .filter(|f| f.describe().srgb)
            .next()
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        // Get the texture view where the scene will be rendered
        let output = surface
            .get_current_texture()
            .map_err(|err| FruityError::GenericFailure(err.to_string()))?;

        let rendering_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Create camera bind group
        let (camera_buffer, camera_bind_group) = Self::initialize_camera(&device);

        // Create camera bind group
        let (viewport_size_buffer, viewport_size_bind_group) =
            Self::initialize_viewport_size(&device);

        // Create camera bind group
        let (render_surface_size_buffer, render_surface_size_bind_group) =
            Self::initialize_render_surface_size(&device);

        // Update state
        Ok(State {
            device,
            queue,
            config,
            rendering_view,
            camera_transform: RwLock::new(Matrix4::identity()),
            camera_buffer,
            camera_bind_group,
            viewport_size_buffer,
            viewport_size_bind_group,
            render_surface_size_buffer,
            render_surface_size_bind_group,
        })
    }

    pub fn push_render_instance(
        &self,
        instance_identifier: u64,
        mut instance_buffer: Vec<u8>,
        mesh: ResourceReference<dyn MeshResource>,
        material: ResourceReference<dyn MaterialResource>,
        z_index: i32,
    ) {
        profile_function!();

        let identifier = RenderInstanceIdentifier {
            mesh_identifier: mesh.get_name(),
            material_identifier: material.get_name(),
            z_index,
        };

        let mut render_instances = self.render_instances.write();
        if let Some(render_instance) = render_instances.get_mut(&identifier) {
            render_instance.instance_count += 1;

            if let Some(existing_instance_buffer) = render_instance
                .instance_buffers
                .get_mut(&instance_identifier)
            {
                existing_instance_buffer.append(&mut instance_buffer);
            } else {
                render_instance
                    .instance_buffers
                    .insert(instance_identifier, instance_buffer);
            }
        } else {
            let mut instance_buffers = BTreeMap::new();
            instance_buffers.insert(instance_identifier, instance_buffer);

            render_instances.insert(
                identifier,
                RenderInstance {
                    instance_count: 1,
                    instance_buffers,
                    mesh,
                    material,
                },
            );
        }
    }

    pub fn update_render_bundles(&self) {
        // We update the bundles only once per frame and not per camera per frame
        let render_instances_reader = self.render_instances.read();
        if render_instances_reader.len() > 0 {
            let mut render_bundles = self.render_bundles.write();

            *render_bundles = render_instances_reader
                .iter()
                .filter_map(|(_test, render_instance)| {
                    let device = self.get_device();
                    let config = self.get_config();

                    // Get resources
                    let material = render_instance.material.read();
                    let material = material.downcast_ref::<WgpuMaterialResource>();

                    let shader = if let Some(shader) = material.get_shader() {
                        shader
                    } else {
                        return None;
                    };

                    let shader = shader.read();
                    let shader = shader.downcast_ref::<WgpuShaderResource>();

                    let mesh = render_instance.mesh.read();
                    let mesh = mesh.downcast_ref::<WgpuMeshResource>();

                    // Create the instance buffer
                    // TODO: Don't do it every frame by implementing a cache system
                    let instance_buffer = render_instance
                        .instance_buffers
                        .values()
                        .flatten()
                        .map(|elem| *elem)
                        .collect::<Vec<_>>();

                    let instance_buffer =
                        device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                            label: Some("Instance Buffer"),
                            contents: &instance_buffer,
                            usage: wgpu::BufferUsages::VERTEX,
                        });
                    let instance_count = render_instance.instance_count as u32;

                    // Render the instances
                    let mut encoder =
                        device.create_render_bundle_encoder(&wgpu::RenderBundleEncoderDescriptor {
                            label: Some("draw_mesh"),
                            color_formats: &[Some(config.format)],
                            depth_stencil: None,
                            sample_count: 1,
                            multiview: None,
                        });
                    encoder.set_pipeline(&shader.render_pipeline);
                    material
                        .binding_groups
                        .iter()
                        .for_each(|(index, bind_group)| {
                            encoder.set_bind_group(*index, &bind_group, &[]);
                        });
                    encoder.set_vertex_buffer(0, mesh.vertex_buffer.slice(..));
                    encoder.set_vertex_buffer(1, instance_buffer.slice(..));
                    encoder
                        .set_index_buffer(mesh.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                    encoder.draw_indexed(0..mesh.index_count as u32, 0, 0..instance_count);

                    let bundle = encoder.finish(&wgpu::RenderBundleDescriptor {
                        label: Some("main"),
                    });

                    Some(bundle)
                })
                .collect::<Vec<wgpu::RenderBundle>>();
            std::mem::drop(render_instances_reader);

            let mut render_instances = self.render_instances.write();
            render_instances.clear();
        }
    }

    fn update_camera(&self, view_proj: Matrix4) {
        let state = self.state.as_ref().unwrap();

        // Update camera viewproj bind group
        let mut camera_transform = state.camera_transform.write();
        *camera_transform = view_proj.clone();
        let camera_uniform = CameraUniform(view_proj.into());
        state.queue.write_buffer(
            &state.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        // Update viewport size bind group
        let screen_bottom_left = view_proj * Vector2D::new(-1.0, -1.0);
        let screen_top_right = view_proj * Vector2D::new(1.0, 1.0);
        let viewport_size = (screen_bottom_left - screen_top_right).abs();
        let viewport_size_uniform = ViewportSizeUniform([viewport_size.x, viewport_size.y]);
        state.queue.write_buffer(
            &state.viewport_size_buffer,
            0,
            bytemuck::cast_slice(&[viewport_size_uniform]),
        );
    }

    pub(crate) fn set_state(&mut self, state: State) {
        self.state = Some(state);
    }

    pub fn get_device(&self) -> &wgpu::Device {
        &self.state.as_ref().unwrap().device
    }

    pub fn get_queue(&self) -> &wgpu::Queue {
        &self.state.as_ref().unwrap().queue
    }

    pub fn get_surface(&self) -> &wgpu::Surface {
        &self.surface
    }

    pub fn get_surface_arc(&self) -> Arc<wgpu::Surface> {
        self.surface.clone()
    }

    pub fn get_instance_arc(&self) -> Arc<wgpu::Instance> {
        self.instance.clone()
    }

    pub fn get_config(&self) -> &wgpu::SurfaceConfiguration {
        &self.state.as_ref().unwrap().config
    }

    pub fn get_rendering_view(&self) -> &wgpu::TextureView {
        &self.state.as_ref().unwrap().rendering_view
    }

    pub fn get_camera_bind_group(&self) -> Arc<wgpu::BindGroup> {
        self.state.as_ref().unwrap().camera_bind_group.clone()
    }

    pub fn get_viewport_size_bind_group(&self) -> Arc<wgpu::BindGroup> {
        self.state
            .as_ref()
            .unwrap()
            .viewport_size_bind_group
            .clone()
    }

    pub fn get_render_surface_size_bind_group(&self) -> Arc<wgpu::BindGroup> {
        self.state
            .as_ref()
            .unwrap()
            .render_surface_size_bind_group
            .clone()
    }

    pub fn get_encoder(&self) -> Option<&RwLock<wgpu::CommandEncoder>> {
        self.current_encoder.as_ref()
    }

    fn initialize_camera(device: &wgpu::Device) -> (wgpu::Buffer, Arc<wgpu::BindGroup>) {
        let camera_uniform = CameraUniform(Matrix4::identity().into());

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Camera Buffer"),
            }),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("Camera Buffer"),
        });

        (camera_buffer, Arc::new(camera_bind_group))
    }

    fn initialize_viewport_size(device: &wgpu::Device) -> (wgpu::Buffer, Arc<wgpu::BindGroup>) {
        let viewport_size = Vector2D::default();

        let viewport_size_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Viewport Size Buffer"),
            contents: bytemuck::cast_slice(&[viewport_size]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let viewport_size_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Viewport Size Buffer"),
            }),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: viewport_size_buffer.as_entire_binding(),
            }],
            label: Some("Viewport Size Buffer"),
        });

        (viewport_size_buffer, Arc::new(viewport_size_bind_group))
    }

    fn initialize_render_surface_size(
        device: &wgpu::Device,
    ) -> (wgpu::Buffer, Arc<wgpu::BindGroup>) {
        let render_surface_size = (u32::default(), u32::default());

        let render_surface_size_buffer =
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Render Surface Size Buffer"),
                contents: bytemuck::cast_slice(&[render_surface_size.0, render_surface_size.1]),
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            });

        let render_surface_size_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Render Surface Size Buffer"),
            }),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: render_surface_size_buffer.as_entire_binding(),
            }],
            label: Some("Render Surface Size Buffer"),
        });

        (
            render_surface_size_buffer,
            Arc::new(render_surface_size_bind_group),
        )
    }

    fn build_instance_buffer(
        material: &ResourceReference<dyn MaterialResource>,
        params: HashMap<String, MaterialParam>,
    ) -> Vec<u8> {
        // Get references
        let material = material.read();
        let material = material.downcast_ref::<WgpuMaterialResource>();

        // Allocate instance buffer
        let mut instance_buffer = Vec::with_capacity(material.instance_size);
        instance_buffer.resize(material.instance_size, 0);

        // Inject the values into the instance buffer
        params.into_iter().for_each(|(param_name, param)| {
            let material_fields = if let Some(material_fields) = material.fields.get(&param_name) {
                material_fields
            } else {
                return;
            };

            material_fields
                .iter()
                .for_each(|material_field| match param {
                    MaterialParam::Uint(value) => {
                        if let InstanceField::Uint { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Int(value) => {
                        if let InstanceField::Int { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Float(value) => {
                        if let InstanceField::Float { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Vector2D(value) => {
                        if let InstanceField::Vector2D { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Color(value) => {
                        if let InstanceField::Vector4d { location } = material_field {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location.offset,
                                location.size,
                                value,
                            );
                        }
                    }
                    MaterialParam::Rect {
                        bottom_left,
                        top_right,
                    } => {
                        if let InstanceField::Rect {
                            location_0,
                            location_1,
                        } = material_field
                        {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location_0.offset,
                                location_0.size,
                                bottom_left,
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                location_1.offset,
                                location_1.size,
                                top_right,
                            );
                        }
                    }
                    MaterialParam::Matrix4(value) => {
                        if let InstanceField::Matrix4 {
                            location_0,
                            location_1,
                            location_2,
                            location_3,
                        } = material_field
                        {
                            encode_into_bytes(
                                &mut instance_buffer,
                                location_0.offset,
                                location_0.size,
                                value.0[0],
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                location_1.offset,
                                location_1.size,
                                value.0[1],
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                location_2.offset,
                                location_2.size,
                                value.0[2],
                            );
                            encode_into_bytes(
                                &mut instance_buffer,
                                location_3.offset,
                                location_3.size,
                                value.0[3],
                            );
                        }
                    }
                });
        });

        instance_buffer
    }
}

#[export_impl]
impl GraphicService for WgpuGraphicService {
    #[export]
    fn start_draw(&mut self) -> FruityResult<()> {
        profile_scope("start_draw");

        let mut state = self.state.as_mut().unwrap();

        // Get the texture view where the scene will be rendered
        let output = self
            .surface
            .get_current_texture()
            .map_err(|err| FruityError::GenericFailure(err.to_string()))?;

        let rendering_view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        self.current_output = Some(output);
        state.rendering_view = rendering_view;

        let encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        // Store the handles about this frame
        self.current_encoder = Some(RwLock::new(encoder));

        Ok(())
    }

    #[export]
    fn end_draw(&mut self) {
        profile_scope("end_draw");

        let encoder = if let Some(encoder) = self.current_encoder.take() {
            encoder.into_inner()
        } else {
            return;
        };

        let output = if let Some(output) = self.current_output.take() {
            output
        } else {
            return;
        };

        self.get_queue().submit(std::iter::once(encoder.finish()));
        output.present();

        let mut render_instances = self.render_instances.write();
        render_instances.clear();

        let mut render_bundles = self.render_bundles.write();
        render_bundles.clear();
    }

    #[export]
    fn render_scene(
        &self,
        view_proj: Matrix4,
        background_color: Color,
        target: Option<ResourceReference<dyn TextureResource>>,
    ) {
        profile_function!();

        let state = self.state.as_ref().unwrap();

        self.update_camera(view_proj);

        let mut encoder = if let Some(encoder) = self.current_encoder.as_ref() {
            encoder.write()
        } else {
            return;
        };

        let (rendering_view, render_surface_size) = target
            .as_ref()
            .map(|texture| {
                let texture = texture.read();
                let texture = texture.downcast_ref::<WgpuTextureResource>();

                // TODO: Try to find a way to remove that
                let value = unsafe {
                    std::mem::transmute::<&wgpu::TextureView, &wgpu::TextureView>(&texture.view)
                };

                (value, texture.get_size())
            })
            .unwrap_or_else(|| (&state.rendering_view, self.get_viewport_size()));

        // Update viewport size bind group
        let render_surface_size_uniform =
            RenderSurfaceSizeUniform([render_surface_size.0 as f32, render_surface_size.1 as f32]);
        state.queue.write_buffer(
            &state.render_surface_size_buffer,
            0,
            bytemuck::cast_slice(&[render_surface_size_uniform]),
        );

        let mut render_pass = {
            encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: rendering_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: background_color.r as f64,
                            g: background_color.g as f64,
                            b: background_color.b as f64,
                            a: background_color.a as f64,
                        }),
                        store: true,
                    },
                })],
                depth_stencil_attachment: None,
            })
        };

        // Render the instances bundles
        self.update_render_bundles();

        let render_bundles = self.render_bundles.read();
        render_bundles.iter().for_each(move |bundle| {
            render_pass.execute_bundles(iter::once(bundle));
        });
    }

    #[export]
    fn get_camera_transform(&self) -> Matrix4 {
        let state = self.state.as_ref().unwrap();
        let camera_transform = state.camera_transform.read();
        camera_transform.clone()
    }

    #[export]
    fn resize(&mut self, width: u32, height: u32) {
        let mut state = self.state.as_mut().unwrap();

        state.config.width = width;
        state.config.height = height;

        self.surface.configure(&state.device, &state.config);
    }

    fn draw_mesh(
        &self,
        identifier: u64,
        mesh: ResourceReference<dyn MeshResource>,
        material: ResourceReference<dyn MaterialResource>,
        params: HashMap<String, MaterialParam>,
        z_index: i32,
    ) {
        let instance_buffer = Self::build_instance_buffer(&material, params);
        self.push_render_instance(identifier, instance_buffer, mesh, material, z_index)
    }

    fn create_mesh_resource(
        &self,
        identifier: &str,
        params: MeshResourceSettings,
    ) -> FruityResult<Box<dyn MeshResource>> {
        let device = self.get_device();

        let resource = WgpuMeshResource::new(device, identifier, &params);

        Ok(Box::new(resource))
    }

    fn create_shader_resource(
        &self,
        identifier: &str,
        contents: String,
        params: ShaderResourceSettings,
    ) -> FruityResult<Box<dyn ShaderResource>> {
        let device = self.get_device();
        let surface_config = self.get_config();

        let resource =
            WgpuShaderResource::new(device, surface_config, &contents, identifier, &params);

        Ok(Box::new(resource))
    }

    fn create_texture_resource(
        &self,
        identifier: &str,
        contents: &[u8],
        _params: TextureResourceSettings,
    ) -> FruityResult<Box<dyn TextureResource>> {
        let device = self.get_device();
        let queue = self.get_queue();

        let image = load_from_memory(contents)
            .map_err(|err| FruityError::GenericFailure(err.to_string()))?;
        let resource = WgpuTextureResource::from_image(device, queue, &image, Some(&identifier))?;

        Ok(Box::new(resource))
    }

    fn create_material_resource(
        &self,
        _identifier: &str,
        params: MaterialResourceSettings,
    ) -> FruityResult<Box<dyn MaterialResource>> {
        let resource = WgpuMaterialResource::new(self, &params)?;

        Ok(Box::new(resource))
    }

    #[export]
    fn world_position_to_viewport_position(&self, pos: Vector2D) -> (u32, u32) {
        let viewport_offset = self.get_viewport_offset();
        let viewport_size = self.get_viewport_size();
        let camera_transform = self.get_camera_transform().clone();

        let viewport_pos = camera_transform * pos;

        (
            ((viewport_pos.x + 1.0) / 2.0 * viewport_size.0 as f32 + viewport_offset.0 as f32)
                as u32,
            ((viewport_pos.y - 1.0) / -2.0 * viewport_size.1 as f32 + viewport_offset.1 as f32)
                as u32,
        )
    }

    #[export]
    fn viewport_position_to_world_position(&self, x: u32, y: u32) -> Vector2D {
        let viewport_offset = self.get_viewport_offset();
        let viewport_size = self.get_viewport_size();
        let camera_transform = self.get_camera_transform().clone();

        // Transform the cursor in the engine world (especialy taking care of camera)
        let cursor_pos = Vector2D::new(
            ((x as f32 - viewport_offset.0 as f32) / viewport_size.0 as f32) * 2.0 - 1.0,
            ((y as f32 - viewport_offset.1 as f32) / viewport_size.1 as f32) * -2.0 + 1.0,
        );

        camera_transform.invert() * cursor_pos
    }

    #[export]
    fn get_cursor_position(&self) -> Vector2D {
        // Get informations from the resource dependencies
        let cursor_position = {
            let window_service = self.window_service.read();
            window_service.get_cursor_position()
        };

        self.viewport_position_to_world_position(cursor_position.0, cursor_position.1)
    }

    #[export]
    fn is_cursor_hover_scene(&self) -> bool {
        // Get informations from the resource dependencies
        let cursor_position = {
            let window_service = self.window_service.read();
            window_service.get_cursor_position()
        };

        let viewport_offset = self.get_viewport_offset();
        let viewport_size = self.get_viewport_size();

        let cursor_pos = Vector2D::new(
            (cursor_position.0 as f32 - viewport_offset.0 as f32) / viewport_size.0 as f32,
            (cursor_position.1 as f32 - viewport_offset.1 as f32) / viewport_size.1 as f32,
        );

        cursor_pos.x >= 0.0 && cursor_pos.x < 1.0 && cursor_pos.y >= 0.0 && cursor_pos.y < 1.0
    }

    #[export]
    fn get_viewport_offset(&self) -> (u32, u32) {
        let viewport_offset = self.viewport_offset.read();
        viewport_offset.clone()
    }

    #[export]
    fn set_viewport_offset(&self, x: u32, y: u32) {
        let mut viewport_offset = self.viewport_offset.write();
        *viewport_offset = (x, y);
    }

    #[export]
    fn get_viewport_size(&self) -> (u32, u32) {
        let viewport_size = self.viewport_size.read();
        viewport_size.clone()
    }

    #[export]
    fn set_viewport_size(&self, x: u32, y: u32) {
        let mut viewport_size = self.viewport_size.write();
        *viewport_size = (x, y);
    }
}
