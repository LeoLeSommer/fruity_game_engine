use std::sync::Arc;

use crate::editor_service::EditorService;
use crate::state::secondary_action::SecondaryActionState;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_game_engine::module::Module;
use fruity_game_engine::profile_scope;
use fruity_game_engine::{export_function, typescript_import, FruityResult};
use fruity_graphic::graphic_service::GraphicService;

pub mod editor_service;
pub mod state;
pub mod ui_element;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_editor_egui_module() -> Module {
    Module {
        name: "fruity_editor".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_graphic".to_string(),
            "fruity_input".to_string(),
            "fruity_windows".to_string(),
            "fruity_abstract_editor".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let editor_service = EditorService::new(resource_container.clone());
            resource_container.add::<EditorService>("editor_service", Box::new(editor_service));

            resource_container.add::<SecondaryActionState>(
                "secondary_action_state",
                Box::new(SecondaryActionState::default()),
            );

            {
                let graphic_service = resource_container.require::<dyn GraphicService>();
                let entity_service = resource_container.require::<EntityService>();
                world.add_run_frame_middleware(move |_next, world| {
                    {
                        let mut graphic_service = graphic_service.write();
                        graphic_service.start_draw()?;
                    }

                    {
                        profile_scope!("draw_editor");
                        let editor_service =
                            world.get_resource_container().require::<EditorService>();
                        let mut editor_service = editor_service.write();

                        editor_service.draw()?;
                    }

                    {
                        let mut entity_service_writer = entity_service.write();
                        unsafe { entity_service_writer.apply_pending_mutations()? };
                    }

                    {
                        let mut graphic_service = graphic_service.write();
                        graphic_service.end_draw();
                    }

                    FruityResult::Ok(())
                });
            }

            Ok(())
        })),
        ..Default::default()
    }
}
