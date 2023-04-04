use crate::systems::display_grid::display_grid;
use fruity_ecs::system::SystemParams;
use fruity_ecs::system::SystemService;
use fruity_editor::editor_component_service::EditorComponentService;
use fruity_editor::editor_component_service::RegisterComponentParams;
use fruity_game_engine::export_function;
use fruity_game_engine::module::Module;
use fruity_game_engine::typescript_import;
use fruity_game_engine::Arc;

pub mod systems;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_editor_graphic_2d_module() -> Module {
    Module {
        name: "fruity_editor_graphic_2d".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_editor".to_string(),
            "fruity_input".to_string(),
            "fruity_graphic".to_string(),
            "fruity_graphic_2d".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let system_service = resource_container.require::<SystemService>();
            let mut system_service = system_service.write();

            system_service.add_system(
                "display_grid",
                &display_grid as &'static (dyn Fn(_, _, _) -> _ + Send + Sync),
                Some(SystemParams {
                    pool_index: Some(98),
                    ignore_pause: Some(true),
                    ..Default::default()
                }),
            );

            let editor_component_service = resource_container.require::<EditorComponentService>();
            let mut editor_component_service = editor_component_service.write();

            editor_component_service
                .register_component("Transform2d", RegisterComponentParams::default());
            editor_component_service.register_component(
                "Translate2d",
                RegisterComponentParams {
                    dependencies: vec!["Transform2d".to_string()],
                    ..Default::default()
                },
            );
            editor_component_service.register_component(
                "Rotate2d",
                RegisterComponentParams {
                    dependencies: vec!["Transform2d".to_string()],
                    ..Default::default()
                },
            );
            editor_component_service.register_component(
                "Scale2d",
                RegisterComponentParams {
                    dependencies: vec!["Transform2d".to_string()],
                    ..Default::default()
                },
            );
            editor_component_service
                .register_component("Sprite", RegisterComponentParams::default());
            editor_component_service.register_component(
                "Camera",
                RegisterComponentParams {
                    dependencies: vec!["Transform2d".to_string()],
                    ..Default::default()
                },
            );

            Ok(())
        })),
        ..Default::default()
    }
}
