use fruity_game_engine::module::Module;
use fruity_game_engine::{export_function, typescript_import};
use std::rc::Rc;

pub mod window_service;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_windows_module() -> Module {
    Module {
        name: "fruity_abstract_windows".to_string(),
        dependencies: vec![],
        ..Default::default()
    }
}
