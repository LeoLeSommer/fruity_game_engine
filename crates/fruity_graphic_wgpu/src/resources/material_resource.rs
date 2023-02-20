use crate::resources::shader_resource::WgpuShaderResource;
use crate::resources::texture_resource::WgpuTextureResource;
use crate::utils::insert_in_hashmap_vec;
use crate::WgpuGraphicService;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::resource::Resource;
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::material_resource::MaterialResourceSettings;
use fruity_graphic::resources::material_resource::MaterialSettingsBinding;
use fruity_graphic::resources::material_resource::MaterialSettingsInstanceAttribute;
use fruity_graphic::resources::shader_resource::ShaderInstanceAttributeType;
use fruity_graphic::resources::shader_resource::ShaderResource;
use std::collections::HashMap;
use std::mem::size_of;
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct BufferLocation {
    pub offset: usize,
    pub size: usize,
}

#[derive(Debug)]
pub enum InstanceField {
    Uint {
        location: BufferLocation,
    },
    Int {
        location: BufferLocation,
    },
    Float {
        location: BufferLocation,
    },
    Vector2d {
        location: BufferLocation,
    },
    Vector4d {
        location: BufferLocation,
    },
    Rect {
        vec0_location: BufferLocation,
        vec1_location: BufferLocation,
    },
    Matrix4 {
        vec0_location: BufferLocation,
        vec1_location: BufferLocation,
        vec2_location: BufferLocation,
        vec3_location: BufferLocation,
    },
}

#[derive(Debug, FruityAny, Resource)]
#[export_struct]
pub struct WgpuMaterialResource {
    pub(crate) params: MaterialResourceSettings,
    pub(crate) binding_groups: Vec<(u32, Arc<wgpu::BindGroup>)>,
    pub(crate) fields: HashMap<String, Vec<InstanceField>>,
    pub(crate) instance_size: usize,
}

impl WgpuMaterialResource {
    pub fn new(graphic_service: &WgpuGraphicService, params: &MaterialResourceSettings) -> Self {
        let shader = if let Some(shader) = params.shader.as_ref().map(|shader| shader.read()) {
            shader
        } else {
            return Self {
                params: params.clone(),
                binding_groups: Vec::new(),
                fields: HashMap::new(),
                instance_size: 0,
            };
        };

        let shader = shader.downcast_ref::<WgpuShaderResource>();

        // Get the binding group
        let binding_groups = params
            .bindings
            .iter()
            .map(|binding| match binding {
                MaterialSettingsBinding::Texture { value, bind_group } => {
                    let value = value.read();
                    let value = value.downcast_ref::<WgpuTextureResource>();
                    (*bind_group, value.bind_group.clone())
                }
                MaterialSettingsBinding::Camera { bind_group } => {
                    (*bind_group, graphic_service.get_camera_bind_group())
                }
                MaterialSettingsBinding::ViewportSize { bind_group } => {
                    (*bind_group, graphic_service.get_viewport_size_bind_group())
                }
                MaterialSettingsBinding::RenderSurfaceSize { bind_group } => (
                    *bind_group,
                    graphic_service.get_render_surface_size_bind_group(),
                ),
            })
            .collect::<Vec<_>>();

        // Build an association beween location and the position of datas in the buffer
        let mut current_offset = 0;
        let mut fields_by_locations = HashMap::<u32, BufferLocation>::new();
        shader
            .params
            .instance_attributes
            .iter()
            .for_each(|instance_attribute| {
                let size = match instance_attribute.ty {
                    ShaderInstanceAttributeType::Int => size_of::<i32>(),
                    ShaderInstanceAttributeType::Uint => size_of::<u32>(),
                    ShaderInstanceAttributeType::Float => size_of::<f32>(),
                    ShaderInstanceAttributeType::Vector2d => size_of::<[f32; 2]>(),
                    ShaderInstanceAttributeType::Vector4d => size_of::<[f32; 4]>(),
                };

                fields_by_locations.insert(
                    instance_attribute.location,
                    BufferLocation {
                        offset: current_offset,
                        size: size,
                    },
                );

                current_offset += size;
            });

        // Insert the instance fields
        let mut fields = HashMap::<String, Vec<InstanceField>>::new();
        params
            .instance_attributes
            .iter()
            .for_each(|instance_attribute| match instance_attribute.1 {
                MaterialSettingsInstanceAttribute::Uint { location } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Uint {
                            location: fields_by_locations.get(location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Int { location } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Int {
                            location: fields_by_locations.get(location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Float { location } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Float {
                            location: fields_by_locations.get(location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Vector2d { location } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Vector2d {
                            location: fields_by_locations.get(location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Vector4d { location } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Vector4d {
                            location: fields_by_locations.get(location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Rect {
                    vec0_location,
                    vec1_location,
                } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Rect {
                            vec0_location: fields_by_locations.get(vec0_location).unwrap().clone(),
                            vec1_location: fields_by_locations.get(vec1_location).unwrap().clone(),
                        },
                    );
                }
                MaterialSettingsInstanceAttribute::Matrix4 {
                    vec0_location,
                    vec1_location,
                    vec2_location,
                    vec3_location,
                } => {
                    insert_in_hashmap_vec(
                        &mut fields,
                        instance_attribute.0.clone(),
                        InstanceField::Matrix4 {
                            vec0_location: fields_by_locations.get(vec0_location).unwrap().clone(),
                            vec1_location: fields_by_locations.get(vec1_location).unwrap().clone(),
                            vec2_location: fields_by_locations.get(vec2_location).unwrap().clone(),
                            vec3_location: fields_by_locations.get(vec3_location).unwrap().clone(),
                        },
                    );
                }
            });

        Self {
            params: params.clone(),
            binding_groups,
            fields,
            instance_size: current_offset,
        }
    }
}

#[export_impl]
impl MaterialResource for WgpuMaterialResource {
    #[export]
    fn get_shader(&self) -> Option<ResourceReference<dyn ShaderResource>> {
        self.params.shader.clone()
    }
}
