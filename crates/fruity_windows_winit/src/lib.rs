use fruity_game_engine::module::Module;
use fruity_game_engine::{export_function, typescript_import};
use std::rc::Rc;

pub mod fps_counter;
pub mod window_middleware;
pub mod window_service;

#[typescript_import({Signal, Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_windows_winit_module() -> Module {
    Module {
        name: "fruity_windows".to_string(),
        dependencies: vec!["fruity_abstract_windows".to_string()],
        run_middleware: Some(Rc::new(window_middleware::window_middleware)),
        ..Default::default()
    }
}
