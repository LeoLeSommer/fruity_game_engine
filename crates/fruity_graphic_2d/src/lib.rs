use crate::components::camera::Camera;
use crate::components::rotate_2d::Rotate2D;
use crate::components::scale_2d::Scale2D;
use crate::components::sprite::Sprite;
use crate::components::transform_2d::Transform2D;
use crate::components::translate_2d::Translate2D;
use crate::graphic_2d_service::Graphic2dService;
use crate::systems::draw_camera::draw_camera;
use crate::systems::draw_sprite::draw_sprite;
use crate::systems::update_transform_2d::update_transform_2d;
use fruity_ecs::system_service::{SystemParams, SystemService};
use fruity_game_engine::module::Module;
use fruity_game_engine::object_factory_service::ObjectFactoryService;
use fruity_game_engine::{export_function, typescript_import};
use std::sync::Arc;

pub mod components;
pub mod graphic_2d_service;
pub mod systems;

#[typescript_import({ResourceReference, Module} from "fruity_game_engine")]
#[typescript_import({TextureResource, Color, MaterialResource, MaterialParam, Vector2D, Matrix3} from "fruity_graphic")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_graphic_2d_module() -> Module {
    Module {
        name: "fruity_graphic_2d".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_graphic".to_string(),
            "fruity_windows".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let graphic_2d_service = Graphic2dService::new(resource_container.clone())?;
            resource_container
                .add::<Graphic2dService>("graphic_2d_service", Box::new(graphic_2d_service));

            let object_factory_service = resource_container.require::<ObjectFactoryService>();
            let mut object_factory_service = object_factory_service.write();

            object_factory_service.register::<Transform2D>("Transform2D");
            object_factory_service.register::<Translate2D>("Translate2D");
            object_factory_service.register::<Rotate2D>("Rotate2D");
            object_factory_service.register::<Scale2D>("Scale2D");
            object_factory_service.register::<Sprite>("Sprite");
            object_factory_service.register::<Camera>("Camera");

            let system_service = resource_container.require::<SystemService>();
            let mut system_service = system_service.write();

            system_service.add_system(
                "update_transform_2d",
                &update_transform_2d as &'static (dyn Fn(_) -> _ + Send + Sync),
                Some(SystemParams {
                    pool_index: 95,
                    ignore_pause: true,
                }),
            );

            system_service.add_system(
                "draw_sprite",
                &draw_sprite as &'static (dyn Fn(_, _) -> _ + Send + Sync),
                Some(SystemParams {
                    pool_index: 98,
                    ignore_pause: true,
                }),
            );

            system_service.add_system(
                "draw_camera",
                &draw_camera as &'static (dyn Fn(_, _) -> _ + Send + Sync),
                Some(SystemParams {
                    pool_index: 99,
                    ignore_pause: true,
                }),
            );

            Ok(())
        })),
        ..Default::default()
    }
}
