#![deny(clippy::all)]

use fruity_game_engine::Arc;
use fruity_game_engine::{export_function, module::Module, typescript};

pub extern crate fruity_ecs;
pub extern crate fruity_editor;
pub extern crate fruity_editor_egui;
pub extern crate fruity_editor_graphic;
pub extern crate fruity_editor_graphic_2d;
pub extern crate fruity_editor_hierarchy;
pub extern crate fruity_editor_physic_2d;
pub extern crate fruity_game_engine;
pub extern crate fruity_graphic;
pub extern crate fruity_graphic_2d;
pub extern crate fruity_graphic_wgpu;
pub extern crate fruity_hierarchy;
pub extern crate fruity_hierarchy_2d;
pub extern crate fruity_input;
pub extern crate fruity_input_winit;
pub extern crate fruity_physic_2d;
pub extern crate fruity_physic_parry_2d;
pub extern crate fruity_windows;
pub extern crate fruity_windows_winit;

// Just for the typescript generation, this function lives in js
#[typescript("default function initFruityBundle();")]
#[warn(dead_code)]
pub fn init_fruity_bundle() {}

#[export_function]
pub fn create_fruity_native_bundle_module() -> Module {
    Module {
        name: "fruity_native_bundle".to_string(),
        dependencies: vec!["fruity_ecs".to_string()],
        setup: Some(Arc::new(|_world, _settings| Ok(()))),
        ..Default::default()
    }
}
