use crate::components::circle_collider::CircleCollider;
use crate::components::rect_collider::RectCollider;
use fruity_ecs::serialization_service::SerializationService;
use fruity_game_engine::module::Module;
use fruity_game_engine::{export_function, typescript_import};
use std::sync::Arc;

pub mod components;

#[typescript_import({Module} from "fruity_game_engine")]
#[typescript_import({Vector2D} from "fruity_graphic")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_physic_2d_module() -> Module {
    Module {
        name: "fruity_physic_2d".to_string(),
        dependencies: vec!["fruity_ecs".to_string(), "fruity_graphic".to_string()],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let serialization_service = resource_container.require::<SerializationService>();
            let mut serialization_service = serialization_service.write();

            serialization_service.register_component::<CircleCollider>();
            serialization_service.register_component::<RectCollider>();

            Ok(())
        })),
        ..Default::default()
    }
}
