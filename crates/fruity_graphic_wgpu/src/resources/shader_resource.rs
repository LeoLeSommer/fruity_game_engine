use crate::wgpu_bridge::VERTEX_DESC;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::Resource;
use fruity_graphic::resources::shader_resource::ShaderBinding;
use fruity_graphic::resources::shader_resource::ShaderBindingGroup;
use fruity_graphic::resources::shader_resource::ShaderBindingType;
use fruity_graphic::resources::shader_resource::ShaderBindingVisibility;
use fruity_graphic::resources::shader_resource::ShaderInstanceAttribute;
use fruity_graphic::resources::shader_resource::ShaderInstanceAttributeType;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::shader_resource::ShaderResourceSettings;
use std::mem::size_of;

#[derive(Debug, FruityAny, Resource)]
#[export_struct]
pub struct WgpuShaderResource {
    pub params: ShaderResourceSettings,
    pub(crate) render_pipeline: wgpu::RenderPipeline,
}

impl WgpuShaderResource {
    pub fn new(
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
        buffer: &str,
        label: &str,
        params: &ShaderResourceSettings,
    ) -> WgpuShaderResource {
        // Create the shader
        let shader_module = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some(label),
            source: wgpu::ShaderSource::Wgsl(buffer.into()),
        });

        let binding_groups_layout = params
            .binding_groups
            .iter()
            .map(|binding_group| Self::build_binding_group_layout(binding_group, label, device))
            .collect::<Vec<_>>();

        let (instance_attributes, instance_size) =
            Self::build_instance_attributes(&params.instance_attributes);

        let render_pipeline = Self::build_render_pipeline(
            &binding_groups_layout,
            &instance_attributes,
            instance_size,
            &shader_module,
            label,
            device,
            surface_config,
        );

        WgpuShaderResource {
            params: params.clone(),
            render_pipeline,
        }
    }

    fn build_render_pipeline(
        binding_groups_layout: &[wgpu::BindGroupLayout],
        instance_buffer_layout: &[wgpu::VertexAttribute],
        instance_size: usize,
        shader_module: &wgpu::ShaderModule,
        label: &str,
        device: &wgpu::Device,
        surface_config: &wgpu::SurfaceConfiguration,
    ) -> wgpu::RenderPipeline {
        let render_pipeline_layout =
            device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some(label),
                bind_group_layouts: &binding_groups_layout.iter().collect::<Vec<_>>(),
                push_constant_ranges: &[],
            });

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some(label),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                buffers: &[
                    VERTEX_DESC.clone(),
                    wgpu::VertexBufferLayout {
                        array_stride: instance_size as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: instance_buffer_layout,
                    },
                ],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: surface_config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::OVER,
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                ..Default::default()
            },
            multiview: None,
        })
    }

    fn build_binding_group_layout(
        binding_group: &ShaderBindingGroup,
        label: &str,
        device: &wgpu::Device,
    ) -> wgpu::BindGroupLayout {
        device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            entries: &binding_group
                .bindings
                .iter()
                .enumerate()
                .map(|(index, binding)| Self::build_binding_group_layout_entry(index, binding))
                .collect::<Vec<_>>(),
            label: Some(label),
        })
    }

    fn build_binding_group_layout_entry(
        index: usize,
        binding: &ShaderBinding,
    ) -> wgpu::BindGroupLayoutEntry {
        wgpu::BindGroupLayoutEntry {
            binding: index as u32,
            visibility: match binding.visibility {
                ShaderBindingVisibility::Vertex => wgpu::ShaderStages::VERTEX,
                ShaderBindingVisibility::Fragment => wgpu::ShaderStages::FRAGMENT,
            },
            ty: match binding.ty {
                ShaderBindingType::Texture => wgpu::BindingType::Texture {
                    multisampled: false,
                    view_dimension: wgpu::TextureViewDimension::D2,
                    sample_type: wgpu::TextureSampleType::Float { filterable: true },
                },
                ShaderBindingType::Sampler => {
                    wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering)
                }
                ShaderBindingType::Uniform => wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
            },
            count: None,
        }
    }

    fn build_instance_attributes(
        instance_attributes: &[ShaderInstanceAttribute],
    ) -> (Vec<wgpu::VertexAttribute>, usize) {
        let mut current_offset = 0;
        let attributes = instance_attributes
            .iter()
            .map(|instance_attribute| {
                let (format, size) = match instance_attribute.ty {
                    ShaderInstanceAttributeType::Uint => {
                        (wgpu::VertexFormat::Uint32, size_of::<u32>())
                    }
                    ShaderInstanceAttributeType::Int => {
                        (wgpu::VertexFormat::Sint32, size_of::<i32>())
                    }
                    ShaderInstanceAttributeType::Float => {
                        (wgpu::VertexFormat::Float32, size_of::<f32>())
                    }
                    ShaderInstanceAttributeType::Vector2d => {
                        (wgpu::VertexFormat::Float32x2, size_of::<[f32; 2]>())
                    }
                    ShaderInstanceAttributeType::Vector4d => {
                        (wgpu::VertexFormat::Float32x4, size_of::<[f32; 4]>())
                    }
                };

                let result = wgpu::VertexAttribute {
                    offset: current_offset as wgpu::BufferAddress,
                    shader_location: instance_attribute.location,
                    format: format,
                };

                current_offset += size;

                result
            })
            .collect::<Vec<_>>();

        (attributes, current_offset)
    }
}

#[export_impl]
impl ShaderResource for WgpuShaderResource {}
