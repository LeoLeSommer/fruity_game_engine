use fruity_game_engine::module::Module;
use fruity_game_engine::Arc;
use fruity_game_engine::{export_function, typescript_import};

pub mod window_service;
pub mod world_fn;

#[typescript_import({Signal, Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_windows_winit_module() -> Module {
    Module {
        name: "fruity_windows".to_string(),
        dependencies: vec!["fruity_abstract_windows".to_string()],
        setup_world_middleware: Some(Arc::new(world_fn::setup_world_middleware)),
        run_world_middleware: Some(Arc::new(world_fn::run_world_middleware)),
        ..Default::default()
    }
}
