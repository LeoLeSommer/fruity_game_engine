use crate::math::matrix3::Matrix3;
use crate::math::matrix4::Matrix4;
use crate::math::vector2d::Vector2D;
use crate::math::Color;
use crate::resources::default_resources::load_default_resources;
use crate::resources::material_resource::load_material;
use crate::resources::shader_resource::load_shader;
use crate::resources::texture_resource::load_texture;
use fruity_ecs::deserialize_service::DeserializeService;
use fruity_game_engine::module::Module;
use fruity_game_engine::{export_function, typescript_import};
use std::sync::Arc;

pub mod graphic_service;
pub mod math;
pub mod resources;

#[typescript_import({ResourceReference, Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_graphic_module() -> Module {
    Module {
        name: "fruity_graphic".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_windows".to_string(),
            "fruity_graphic_platform".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let deserialize_service = resource_container.require::<DeserializeService>();
            let mut deserialize_service = deserialize_service.write();

            deserialize_service.register::<Color>("Color");
            deserialize_service.register::<Vector2D>("Vector2D");
            deserialize_service.register::<Matrix3>("Matrix3");
            deserialize_service.register::<Matrix4>("Matrix4");

            resource_container.add_resource_loader("material", load_material);
            resource_container.add_resource_loader("shader", load_shader);
            resource_container.add_resource_loader("texture", load_texture);

            load_default_resources(resource_container)?;

            Ok(())
        })),
        ..Default::default()
    }
}
