use crate::graphic_service::GraphicService;
use fruity_game_engine::{
    any::FruityAny,
    export_enum, export_impl, export_struct, export_trait,
    introspect::{IntrospectFields, IntrospectMethods},
    resource::ResourceContainer,
    settings::Settings,
    utils::read_file_to_string_async,
    FruityResult,
};
use std::{future::Future, pin::Pin};

#[export_trait]
pub trait ShaderResource: IntrospectFields + IntrospectMethods + Send + Sync {}

#[derive(Debug, Default, Clone, FruityAny)]
#[export_struct(from_raw_js_object = true)]
pub struct ShaderResourceSettings {
    pub binding_groups: Vec<ShaderBindingGroup>,
    pub instance_attributes: Vec<ShaderInstanceAttribute>,
}

#[export_impl]
impl ShaderResourceSettings {}

#[derive(Debug, Default, Clone, FruityAny)]
#[export_struct(from_raw_js_object = true)]
pub struct ShaderBindingGroup {
    pub bindings: Vec<ShaderBinding>,
}

#[export_impl]
impl ShaderBindingGroup {}

#[derive(Debug, Default, Clone, FruityAny)]
#[export_struct(from_raw_js_object = true)]
pub struct ShaderBinding {
    pub visibility: ShaderBindingVisibility,
    pub ty: ShaderBindingType,
}

#[export_impl]
impl ShaderBinding {}

#[derive(Debug, Clone)]
#[export_enum]
pub enum ShaderBindingVisibility {
    Vertex,
    Fragment,
}

impl Default for ShaderBindingVisibility {
    fn default() -> Self {
        ShaderBindingVisibility::Vertex
    }
}

#[derive(Debug, Clone)]
#[export_enum]
pub enum ShaderBindingType {
    Texture,
    Sampler,
    Uniform,
}

impl Default for ShaderBindingType {
    fn default() -> Self {
        ShaderBindingType::Texture
    }
}

#[derive(Debug, Default, Clone, FruityAny)]
#[export_struct(from_raw_js_object = true)]
pub struct ShaderInstanceAttribute {
    pub location: u32,
    pub ty: ShaderInstanceAttributeType,
}

#[export_impl]
impl ShaderInstanceAttribute {}

#[derive(Debug, Clone)]
#[export_enum]
pub enum ShaderInstanceAttributeType {
    Int,
    Uint,
    Float,
    Vector2D,
    Vector4d,
}

impl Default for ShaderInstanceAttributeType {
    fn default() -> Self {
        ShaderInstanceAttributeType::Float
    }
}

pub fn load_shader(
    identifier: &str,
    settings: Settings,
    resource_container: ResourceContainer,
) -> Pin<Box<dyn Send + Future<Output = FruityResult<()>>>> {
    let identifier = identifier.to_string();
    Box::pin(async move {
        // Get the resource path
        let path = settings.get("path", String::default());

        // read the whole file
        let buffer = read_file_to_string_async(&path).await?;

        // Parse settings
        let settings = read_shader_settings(&settings, resource_container.clone());

        // Get the graphic service state
        let graphic_service = resource_container.require::<dyn GraphicService>();
        let graphic_service = graphic_service.read();

        // Build the resource
        let resource = graphic_service.create_shader_resource(&identifier, buffer, settings)?;
        resource_container.add::<dyn ShaderResource>(&identifier, resource);

        Ok(())
    })
}

pub fn read_shader_settings(
    settings: &Settings,
    resource_container: ResourceContainer,
) -> ShaderResourceSettings {
    let binding_groups = settings.get::<Vec<Settings>>("binding_groups", Vec::new());
    let binding_groups = binding_groups
        .iter()
        .filter_map(|params| {
            if let Settings::Array(params) = params {
                Some(params)
            } else {
                None
            }
        })
        .map(|params| read_shader_binding_group_settings(params, resource_container.clone()))
        .collect::<Vec<_>>();

    let instance_attributes = settings.get::<Vec<Settings>>("instance_attributes", Vec::new());
    let instance_attributes =
        read_shader_instance_attributes_settings(&instance_attributes, resource_container.clone());

    ShaderResourceSettings {
        binding_groups,
        instance_attributes,
    }
}

pub fn read_shader_binding_group_settings(
    settings: &Vec<Settings>,
    _resource_container: ResourceContainer,
) -> ShaderBindingGroup {
    let bindings = settings
        .iter()
        .map(|params| ShaderBinding {
            visibility: match &params.get::<String>("visibility", String::default()) as &str {
                "vertex" => ShaderBindingVisibility::Vertex,
                "fragment" => ShaderBindingVisibility::Fragment,
                _ => ShaderBindingVisibility::default(),
            },
            ty: match &params.get::<String>("type", String::default()) as &str {
                "texture" => ShaderBindingType::Texture,
                "sampler" => ShaderBindingType::Sampler,
                "uniform" => ShaderBindingType::Uniform,
                _ => ShaderBindingType::default(),
            },
        })
        .collect::<Vec<_>>();

    ShaderBindingGroup { bindings }
}

pub fn read_shader_instance_attributes_settings(
    settings: &Vec<Settings>,
    _resource_container: ResourceContainer,
) -> Vec<ShaderInstanceAttribute> {
    settings
        .iter()
        .map(|params| ShaderInstanceAttribute {
            location: params.get::<u32>("location", u32::default()),
            ty: match &params.get::<String>("type", String::default()) as &str {
                "int" => ShaderInstanceAttributeType::Int,
                "uint" => ShaderInstanceAttributeType::Uint,
                "float" => ShaderInstanceAttributeType::Float,
                "vec2" => ShaderInstanceAttributeType::Vector2D,
                "vec4" => ShaderInstanceAttributeType::Vector4d,
                _ => ShaderInstanceAttributeType::default(),
            },
        })
        .collect::<Vec<_>>()
}
