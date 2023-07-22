use crate::systems::transform_2d_cascade::transform_2d_cascade;
use fruity_ecs::system::{SystemParams, SystemService};
use fruity_game_engine::{export_function, module::Module, sync::Arc, typescript_import};

pub mod systems;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_hierarchy_2d_module() -> Module {
    Module {
        name: "fruity_hierarchy_2d".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_graphic_2d".to_string(),
            "fruity_hierarchy".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let system_service = resource_container.require::<SystemService>();
            let mut system_service = system_service.write();

            /*system_service.add_system(
                "transform_2d_cascade",
                &transform_2d_cascade as &'static (dyn Fn(_) -> _ + Send + Sync),
                Some(SystemParams {
                    pool_index: Some(96),
                    ignore_pause: Some(true),
                    ..Default::default()
                }),
            );*/

            Ok(())
        })),
        ..Default::default()
    }
}
