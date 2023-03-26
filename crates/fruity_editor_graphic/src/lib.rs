use crate::fields::matrix3::draw_editor_matrix3;
use crate::fields::shader_reference::draw_editor_shader_reference;
use crate::fields::texture_reference::draw_editor_texture_reference;
use crate::fields::vector2d::draw_editor_vector_2d;
use crate::resources::default_resources::load_default_resources_async;
use fruity_editor::introspect_editor_service::IntrospectEditorService;
use fruity_game_engine::module::Module;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::{export_function, typescript_import};
use fruity_graphic::math::matrix3::Matrix3;
use fruity_graphic::math::vector2d::Vector2D;
use fruity_graphic::resources::shader_resource::ShaderResource;
use fruity_graphic::resources::texture_resource::TextureResource;
use std::sync::Arc;

pub mod fields;
pub mod resources;

#[typescript_import({Module} from "fruity_game_engine")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_editor_graphic_module() -> Module {
    Module {
        name: "fruity_editor_graphic".to_string(),
        dependencies: vec![
            "fruity_ecs".to_string(),
            "fruity_editor".to_string(),
            "fruity_graphic".to_string(),
        ],
        setup: Some(Arc::new(|world, _settings| {
            let resource_container = world.get_resource_container();

            let introspect_editor_service = resource_container.require::<IntrospectEditorService>();
            let mut introspect_editor_service = introspect_editor_service.write();
            introspect_editor_service.register_field_editor::<Vector2D, _>(draw_editor_vector_2d);
            introspect_editor_service.register_field_editor::<Matrix3, _>(draw_editor_matrix3);
            introspect_editor_service
                .register_field_editor::<Option<ResourceReference<dyn TextureResource>>, _>(
                    draw_editor_texture_reference,
                );
            introspect_editor_service
                .register_field_editor::<Option<ResourceReference<dyn ShaderResource>>, _>(
                    draw_editor_shader_reference,
                );

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
