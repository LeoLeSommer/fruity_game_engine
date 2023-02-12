use drag_service::DragService;
use fruity_game_engine::export_function;
use fruity_game_engine::module::Module;
use input_service::InputService;
use std::rc::Rc;

pub mod drag_service;
pub mod input_service;

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_input_module() -> Module {
    Module {
        name: "fruity_abstract_input".to_string(),
        dependencies: vec!["fruity_windows".to_string()],
        setup: Some(Rc::new(|world, settings| {
            let resource_container = world.get_resource_container();

            let mut input_service = InputService::new(resource_container.clone());
            input_service.read_input_settings(&settings);
            resource_container.add::<InputService>("input_service", Box::new(input_service));

            let drag_service = DragService::new(resource_container.clone());
            resource_container.add::<DragService>("drag_service", Box::new(drag_service));

            Ok(())
        })),
        load_resources: None,
    }
}
