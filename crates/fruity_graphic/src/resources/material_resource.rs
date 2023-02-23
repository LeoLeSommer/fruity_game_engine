use crate::graphic_service::GraphicService;
use crate::resources::shader_resource::ShaderResource;
use crate::resources::texture_resource::TextureResource;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::resource::Resource;
use fruity_game_engine::settings::Settings;
use fruity_game_engine::{export_trait, FruityResult};
use std::collections::HashMap;

#[export_trait]
pub trait MaterialResource: Resource {
    fn get_shader(&self) -> Option<ResourceReference<dyn ShaderResource>>;
}

#[derive(Debug, Clone, FruityAny)]
pub struct MaterialResourceSettings {
    pub shader: Option<ResourceReference<dyn ShaderResource>>,
    pub bindings: Vec<MaterialSettingsBinding>,
    pub instance_attributes: HashMap<String, MaterialSettingsInstanceAttribute>,
}

#[derive(Debug, Clone, FruityAny)]
pub enum MaterialSettingsBinding {
    Texture {
        value: ResourceReference<dyn TextureResource>,
        bind_group: u32,
    },
    Camera {
        bind_group: u32,
    },
    ViewportSize {
        bind_group: u32,
    },
    RenderSurfaceSize {
        bind_group: u32,
    },
}

#[derive(Debug, Clone, FruityAny)]
pub enum MaterialSettingsInstanceAttribute {
    Uint {
        location: u32,
    },
    Int {
        location: u32,
    },
    Float {
        location: u32,
    },
    Vector2D {
        location: u32,
    },
    Vector4d {
        location: u32,
    },
    Rect {
        location_0: u32,
        location_1: u32,
    },
    Matrix4 {
        location_0: u32,
        location_1: u32,
        location_2: u32,
        location_3: u32,
    },
}

pub fn load_material(
    identifier: &str,
    settings: Settings,
    resource_container: ResourceContainer,
) -> FruityResult<()> {
    // Get the graphic service state
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    // Parse settings
    let settings = read_material_settings(&settings, resource_container.clone());

    // Build the resource
    let resource = graphic_service.create_material_resource(identifier, settings)?;
    resource_container.add::<dyn MaterialResource>(identifier, resource);

    Ok(())
}

pub fn read_material_settings(
    settings: &Settings,
    resource_container: ResourceContainer,
) -> MaterialResourceSettings {
    let shader_identifier = settings.get::<String>("shader", String::default());
    let shader = resource_container.get::<dyn ShaderResource>(&shader_identifier);

    let bindings_settings = settings.get::<Vec<Settings>>("bindings", Vec::new());
    let bindings = bindings_settings
        .iter()
        .filter_map(|params| build_material_binding(params, resource_container.clone()))
        .collect::<Vec<_>>();

    let instance_attributes_settings =
        settings.get::<Vec<Settings>>("instance_attributes", Vec::new());
    let mut instance_attributes = HashMap::<String, MaterialSettingsInstanceAttribute>::new();
    instance_attributes_settings.iter().for_each(|params| {
        let name = params.get::<Option<String>>("name", None);

        if let Some(name) = name {
            if let Some(instance_attribute) =
                build_material_instance_attribute(params, resource_container.clone())
            {
                instance_attributes.insert(name, instance_attribute);
            }
        }
    });

    MaterialResourceSettings {
        shader,
        bindings,
        instance_attributes,
    }
}

fn build_material_binding(
    settings: &Settings,
    resource_container: ResourceContainer,
) -> Option<MaterialSettingsBinding> {
    match &settings.get::<String>("type", String::default()) as &str {
        "texture" => {
            let value = settings.get::<String>("value", String::default());
            let value = resource_container.get::<dyn TextureResource>(&value);
            let bind_group = settings.get::<u32>("bind_group", u32::default());

            if let Some(value) = value {
                Some(MaterialSettingsBinding::Texture { value, bind_group })
            } else {
                None
            }
        }
        "camera" => {
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            Some(MaterialSettingsBinding::Camera { bind_group })
        }
        "viewport_size" => {
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            Some(MaterialSettingsBinding::ViewportSize { bind_group })
        }
        "render_surface_size" => {
            let bind_group = settings.get::<u32>("bind_group", u32::default());
            Some(MaterialSettingsBinding::RenderSurfaceSize { bind_group })
        }
        _ => None,
    }
}

fn build_material_instance_attribute(
    settings: &Settings,
    _resource_container: ResourceContainer,
) -> Option<MaterialSettingsInstanceAttribute> {
    match &settings.get::<String>("type", String::default()) as &str {
        "matrix4" => {
            let location_0 = settings.get::<u32>("location_0", u32::default());
            let location_1 = settings.get::<u32>("location_1", u32::default());
            let location_2 = settings.get::<u32>("location_2", u32::default());
            let location_3 = settings.get::<u32>("location_3", u32::default());

            Some(MaterialSettingsInstanceAttribute::Matrix4 {
                location_0,
                location_1,
                location_2,
                location_3,
            })
        }
        "rect" => {
            let location_0 = settings.get::<u32>("location_0", u32::default());
            let location_1 = settings.get::<u32>("location_1", u32::default());

            Some(MaterialSettingsInstanceAttribute::Rect {
                location_0,
                location_1,
            })
        }
        "vec2" => {
            let location = settings.get::<u32>("location", u32::default());

            Some(MaterialSettingsInstanceAttribute::Vector2D { location })
        }
        "vec4" => {
            let location = settings.get::<u32>("location", u32::default());

            Some(MaterialSettingsInstanceAttribute::Vector4d { location })
        }
        _ => None,
    }
}
