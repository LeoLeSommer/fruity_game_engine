use crate::math::matrix3::Matrix3;
use crate::math::matrix4::Matrix4;
use crate::math::vector2d::Vector2D;
use crate::math::Color;
use crate::resources::default_resources::load_default_resources;
use crate::resources::material_resource::load_material;
use crate::resources::shader_resource::load_shader;
use crate::resources::texture_resource::load_texture;
use fruity_ecs::serialization::SerializationService;
use fruity_game_engine::module::Module;
use fruity_game_engine::Arc;
use fruity_game_engine::{export_function, typescript_import};

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

            let serialization_service = resource_container.require::<SerializationService>();
            let mut serialization_service = serialization_service.write();

            serialization_service.register::<Color>();
            serialization_service.register::<Vector2D>();
            serialization_service.register::<Matrix3>();
            serialization_service.register::<Matrix4>();

            resource_container.add_resource_loader("material", load_material);
            resource_container.add_resource_loader("shader", load_shader);
            resource_container.add_resource_loader("texture", load_texture);

            load_default_resources(resource_container)?;

            Ok(())
        })),
        ..Default::default()
    }
}
