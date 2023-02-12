use fruity_game_engine::export_function;
use fruity_game_engine::module::Module;
use std::rc::Rc;

pub mod window_service;

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_windows_module() -> Module {
    Module {
        name: "fruity_abstract_windows".to_string(),
        dependencies: vec![],
        setup: Some(Rc::new(|_world, _settings| Ok(()))),
        load_resources: None,
    }
}
