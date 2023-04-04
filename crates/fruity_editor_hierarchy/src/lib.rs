#![feature(iterator_try_collect)]

use crate::components::entity::entity_list::entity_list_component;
use fruity_editor::editor_component_service::EditorComponentService;
use fruity_editor::editor_component_service::RegisterComponentParams;
use fruity_editor::editor_panels_service::EditorPanelsService;
use fruity_editor::ui::elements::pane::UIPaneSide;
use fruity_game_engine::export_function;
use fruity_game_engine::module::Module;
use fruity_game_engine::typescript_import;
use fruity_game_engine::Arc;

pub mod components;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_editor_hierarchy_module() -> Module {
    Module {
        name: "fruity_editor_hierarchy".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_editor".to_string(),
            "fruity_hierarchy".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let editor_component_service = resource_container.require::<EditorComponentService>();
            let mut editor_component_service = editor_component_service.write();

            editor_component_service
                .register_component("Parent", RegisterComponentParams::default());

            let editor_panels_service = resource_container.require::<EditorPanelsService>();
            let mut editor_panels_service = editor_panels_service.write();

            editor_panels_service.add_panel("Entities", UIPaneSide::Left, entity_list_component);

            Ok(())
        })),
        ..Default::default()
    }
}
