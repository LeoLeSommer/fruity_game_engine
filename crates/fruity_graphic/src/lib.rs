use crate::math::matrix3::Matrix3;
use crate::math::matrix4::Matrix4;
use crate::math::vector2d::Vector2d;
use crate::math::Color;
use crate::resources::default_resources::load_default_resources;
use crate::resources::material_resource::load_material;
use crate::resources::shader_resource::load_shader;
use crate::resources::texture_resource::load_texture;
use fruity_game_engine::module::Module;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::{export_function, typescript_import};
use std::rc::Rc;

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
        setup: Some(Rc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let object_factory_service = resource_container.require::<ObjectFactoryService>();
            let mut object_factory_service = object_factory_service.write();

            object_factory_service.register::<Color>("Color");
            object_factory_service.register::<Vector2d>("Vector2d");
            object_factory_service.register::<Matrix3>("Matrix3");
            object_factory_service.register::<Matrix4>("Matrix4");

            resource_container.add_resource_loader("material", load_material);
            resource_container.add_resource_loader("shader", load_shader);
            resource_container.add_resource_loader("texture", load_texture);

            load_default_resources(resource_container)?;

            Ok(())
        })),
        load_resources: None,
        run_middleware: None,
    }
}
