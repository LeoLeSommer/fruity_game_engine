use drag_service::DragService;
use fruity_game_engine::module::Module;
use fruity_game_engine::sync::Arc;
use fruity_game_engine::{export_function, typescript_import};
use input_service::InputService;

pub mod drag_service;
pub mod input_service;

#[typescript_import({Signal, Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_input_module() -> Module {
    Module {
        name: "fruity_abstract_input".to_string(),
        dependencies: vec!["fruity_windows".to_string()],
        setup: Some(Arc::new(|world, settings| {
            let resource_container = world.get_resource_container();

            let mut input_service = InputService::new(resource_container.clone());
            input_service.read_input_settings(&settings)?;
            resource_container.add::<InputService>("input_service", Box::new(input_service));

            let drag_service = DragService::new(resource_container.clone());
            resource_container.add::<DragService>("drag_service", Box::new(drag_service));

            Ok(())
        })),
        ..Default::default()
    }
}
