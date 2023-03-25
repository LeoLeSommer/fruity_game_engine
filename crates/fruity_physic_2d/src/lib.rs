use crate::components::circle_collider::CircleCollider;
use crate::components::rect_collider::RectCollider;
use fruity_ecs::deserialize_service::DeserializeService;
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

            let deserialize_service = resource_container.require::<DeserializeService>();
            let mut deserialize_service = deserialize_service.write();

            deserialize_service.register_component::<CircleCollider>();
            deserialize_service.register_component::<RectCollider>();

            Ok(())
        })),
        ..Default::default()
    }
}
