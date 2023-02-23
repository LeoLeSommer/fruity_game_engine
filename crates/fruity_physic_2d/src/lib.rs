use std::rc::Rc;

use crate::components::circle_collider::CircleCollider;
use crate::components::rect_collider::RectCollider;
use fruity_game_engine::module::Module;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::{export_function, typescript_import};

pub mod components;

#[typescript_import({Module} from "fruity_game_engine")]
#[typescript_import({Vector2D} from "fruity_graphic")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_physic_2d_module() -> Module {
    Module {
        name: "fruity_physic_2d".to_string(),
        dependencies: vec!["fruity_ecs".to_string(), "fruity_graphic".to_string()],
        setup: Some(Rc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let object_factory_service = resource_container.require::<ObjectFactoryService>();
            let mut object_factory_service = object_factory_service.write();

            object_factory_service.register::<CircleCollider>("CircleCollider");
            object_factory_service.register::<RectCollider>("RectCollider");

            Ok(())
        })),
        load_resources: None,
        run_middleware: None,
    }
}
