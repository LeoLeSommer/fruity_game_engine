#![feature(iterator_try_collect)]

use components::{inspector::inspector_component, scene::scene_component};
use editor_component_service::EditorComponentService;
use editor_menu_service::{EditorMenuService, MenuItemOptions};
use editor_panels_service::EditorPanelsService;
use fruity_ecs::system::SystemService;
use fruity_game_engine::{export_function, module::Module, typescript_import};
use inspect::inspect_entity::inspect_entity;
use inspector_service::InspectorService;
use introspect_editor_service::IntrospectEditorService;
use menu::{is_redo_enabled, is_undo_enabled, redo, undo};
use mutations::mutation_service::MutationService;
use resources::default_resources::load_default_resources_async;
use state::{file_explorer::FileExplorerState, inspector::InspectorState, scene::SceneState};
use std::sync::Arc;
use systems::pause_at_startup::pause_at_startup;
use ui::elements::{pane::UIPaneSide, profiling::Profiling, UIWidget};

pub mod components;
pub mod editor_component_service;
pub mod editor_menu_service;
pub mod editor_panels_service;
pub mod fields;
pub mod inspect;
pub mod inspector_service;
pub mod introspect_editor_service;
pub mod menu;
pub mod mutations;
pub mod resources;
pub mod state;
pub mod systems;
pub mod ui;
pub mod utils;

#[typescript_import({Module, ScriptObject, Signal} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_editor_module() -> Module {
    Module {
        name: "fruity_abstract_editor".to_string(),
        dependencies: vec!["fruity_ecs".to_string(), "fruity_graphic".to_string()],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let inspector_service = InspectorService::new(resource_container.clone());
            let introspect_editor_service =
                IntrospectEditorService::new(resource_container.clone());
            let editor_menu_service = EditorMenuService::new(resource_container.clone());
            let editor_panels_service = EditorPanelsService::new(resource_container.clone());
            let editor_component_service = EditorComponentService::new(resource_container.clone());
            let mutation_service = MutationService::new(resource_container.clone());

            resource_container
                .add::<InspectorService>("inspector_service", Box::new(inspector_service));
            resource_container.add::<IntrospectEditorService>(
                "introspect_editor_service",
                Box::new(introspect_editor_service),
            );
            resource_container
                .add::<EditorMenuService>("editor_menu_service", Box::new(editor_menu_service));
            resource_container.add::<EditorPanelsService>(
                "editor_panels_service",
                Box::new(editor_panels_service),
            );
            resource_container.add::<EditorComponentService>(
                "editor_component_service",
                Box::new(editor_component_service),
            );
            resource_container
                .add::<MutationService>("mutation_service", Box::new(mutation_service));

            resource_container.add::<InspectorState>(
                "inspector_state",
                Box::new(InspectorState::new(resource_container.clone())),
            );
            resource_container.add::<SceneState>(
                "scene_state",
                Box::new(SceneState::new(resource_container.clone())),
            );
            resource_container.add::<FileExplorerState>(
                "file_explorer_state",
                Box::new(FileExplorerState::default()),
            );

            let system_service = resource_container.require::<SystemService>();
            let mut system_service = system_service.write();

            system_service.add_startup_system(
                "pause_at_startup",
                &pause_at_startup as &'static (dyn Fn(_) -> _ + Send + Sync),
                None,
            );
            /* system_service.disable_pool(99); */

            let inspector_service = resource_container.require::<InspectorService>();
            let mut inspector_service = inspector_service.write();

            inspector_service.register_inspect_type(inspect_entity);

            let editor_menu_service = resource_container.require::<EditorMenuService>();
            let mut editor_menu_service = editor_menu_service.write();

            editor_menu_service.add_section("File", 10);
            editor_menu_service.add_section("Edit", 20);
            editor_menu_service.add_menu(
                "Undo",
                "Edit",
                undo,
                MenuItemOptions {
                    is_enabled: Some(Arc::new(is_undo_enabled)),
                    shortcut: Some("Ctrl + Z".to_string()),
                    ..Default::default()
                },
            );
            editor_menu_service.add_menu(
                "Redo",
                "Edit",
                redo,
                MenuItemOptions {
                    is_enabled: Some(Arc::new(is_redo_enabled)),
                    shortcut: Some("Ctrl + Shift + Z".to_string()),
                    ..Default::default()
                },
            );

            let editor_panels_service = resource_container.require::<EditorPanelsService>();
            let mut editor_panels_service = editor_panels_service.write();

            editor_panels_service.add_panel("Scene", UIPaneSide::Center, scene_component);
            editor_panels_service.add_panel("Inspector", UIPaneSide::Right, inspector_component);
            editor_panels_service.add_panel("Profiling", UIPaneSide::Right, |_ctx| {
                Ok(Profiling {}.elem())
            });

            Ok(())
        })),
        load_resources_async: Some(Arc::new(|world, _settings| {
            Box::pin(async move {
                let resource_container = world.get_resource_container();
                load_default_resources_async(resource_container).await?;

                Ok(())
            })
        })),
        ..Default::default()
    }
}
