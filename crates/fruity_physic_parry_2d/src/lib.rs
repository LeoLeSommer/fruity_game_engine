use crate::components::parry_circle_collider::ParryCircleCollider;
use crate::components::parry_rect_collider::ParryRectCollider;
use crate::components::rigid_body::RigidBody;
use crate::systems::update_circle_collider::update_circle_collider;
use crate::systems::update_rect_collider::update_rect_collider;
use crate::systems::update_rigid_body::update_rigid_body;
use fruity_ecs::extension_component_service::ExtensionComponentService;
use fruity_ecs::system_service::{SystemParams, SystemService};
use fruity_game_engine::module::Module;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::{export_function, typescript_import};
use fruity_physic_2d::components::circle_collider::CircleCollider;
use fruity_physic_2d::components::rect_collider::RectCollider;
use std::sync::Arc;

pub mod components;
pub mod systems;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_physic_parry_2d_module() -> Module {
    Module {
        name: "fruity_physic_parry_2d".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_graphic".to_string(),
            "fruity_physic_2d".to_string(),
            "fruity_graphic_2d".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let object_factory_service = resource_container.require::<ObjectFactoryService>();
            let mut object_factory_service = object_factory_service.write();

            object_factory_service.register::<RigidBody>("RigidBody");

            let extension_component_service =
                resource_container.require::<ExtensionComponentService>();
            let mut extension_component_service = extension_component_service.write();

            extension_component_service.register::<CircleCollider, ParryCircleCollider>();
            extension_component_service.register::<RectCollider, ParryRectCollider>();

            let system_service = resource_container.require::<SystemService>();
            let mut system_service = system_service.write();

            system_service.add_system(
                "update_rect_collider",
                &update_rect_collider as &'static (dyn Fn(_) -> _ + Send + Sync),
                Some(SystemParams {
                    ignore_pause: true,
                    pool_index: 72,
                    ..Default::default()
                }),
            );

            system_service.add_system(
                "update_circle_collider",
                &update_circle_collider as &'static (dyn Fn(_) -> _ + Send + Sync),
                Some(SystemParams {
                    ignore_pause: true,
                    pool_index: 72,
                    ..Default::default()
                }),
            );

            system_service.add_system(
                "update_rigid_body",
                &update_rigid_body as &'static (dyn Fn(_) -> _ + Send + Sync),
                Some(SystemParams {
                    ignore_pause: true,
                    pool_index: 73,
                    ..Default::default()
                }),
            );

            Ok(())
        })),
        ..Default::default()
    }
}
