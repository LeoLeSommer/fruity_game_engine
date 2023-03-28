use crate::component_inspector::circle_collider_inspector::circle_collider_inspector;
use crate::component_inspector::rect_collider_inspector::rect_collider_inspector;
use crate::state::collider::ColliderState;
use fruity_editor::editor_component_service::EditorComponentService;
use fruity_editor::editor_component_service::RegisterComponentParams;
use fruity_game_engine::export_function;
use fruity_game_engine::module::Module;
use fruity_game_engine::typescript_import;
use std::sync::Arc;

pub mod component_inspector;
pub mod state;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_editor_physic_2d_module() -> Module {
    Module {
        name: "fruity_editor_physic_2d".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_editor".to_string(),
            "fruity_input".to_string(),
            "fruity_graphic".to_string(),
            "fruity_graphic_2d".to_string(),
            "fruity_editor_graphic_2d".to_string(),
            "fruity_physic_2d".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            resource_container.add::<ColliderState>(
                "collider_state",
                Box::new(ColliderState::new(resource_container.clone())),
            );

            let editor_component_service = resource_container.require::<EditorComponentService>();
            let mut editor_component_service = editor_component_service.write();

            editor_component_service.register_component(
                "CircleCollider",
                RegisterComponentParams {
                    inspector: Arc::new(circle_collider_inspector),
                    ..Default::default()
                },
            );
            editor_component_service.register_component(
                "RectCollider",
                RegisterComponentParams {
                    inspector: Arc::new(rect_collider_inspector),
                    ..Default::default()
                },
            );

            Ok(())
        })),
        ..Default::default()
    }
}
