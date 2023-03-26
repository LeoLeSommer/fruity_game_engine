use crate::gizmos_service::GizmosService;
use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_editor::mutations::mutation_service::MutationService;
use fruity_editor::mutations::set_field_mutation::SetFieldMutation;
use fruity_editor::state::inspector::InspectorState;
use fruity_game_engine::inject::{Const, Ref};
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::FruityResult;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::vector2d::Vector2D;
use fruity_graphic::math::Color;
use fruity_graphic_2d::components::scale_2d::Scale2D;
use fruity_graphic_2d::components::transform_2d::Transform2D;
use fruity_graphic_2d::components::translate_2d::Translate2D;

pub fn draw_gizmos_2d(
    inspector_state: Const<InspectorState>,
    gizmos_service: Const<GizmosService>,
    mutation_service: Ref<MutationService>,
    graphic_service: Ref<dyn GraphicService>,
) -> FruityResult<()> {
    if !inspector_state.is_gizmos_enabled() {
        return Ok(());
    }

    if let Some(selected) = inspector_state.get_selected() {
        // TODO: Try to remove that
        let selected = unsafe {
            std::mem::transmute::<&Box<dyn ScriptObject>, &Box<dyn ScriptObject>>(selected)
        };

        let entity = if let Some(entity) = selected.as_any_ref().downcast_ref::<EntityReference>() {
            entity
        } else {
            return Ok(());
        };

        let transform = {
            let entity_reader = entity.read()?;
            let component_reader = entity_reader.get_component_by_type::<Transform2D>()?;

            if let Some(transform) = component_reader.map(|transform| transform.transform) {
                transform
            } else {
                return Ok(());
            }
        };

        let translate_2d = {
            let entity_reader = entity.read()?;
            let component_reader = entity_reader.get_component_by_type::<Translate2D>()?;

            component_reader.map(|translate| translate.vec)
        };

        let scale_2d = {
            let entity_reader = entity.read()?;
            let component_reader = entity_reader.get_component_by_type::<Scale2D>()?;

            component_reader.map(|scale| scale.vec)
        };

        if let Some(_) = translate_2d {
            let bottom_left = Vector2D::new(-0.5, -0.5);
            let top_right = Vector2D::new(0.5, 0.5);

            if let Some(_) = scale_2d {
                gizmos_service.draw_resize_helper(
                    bottom_left,
                    top_right,
                    Color::green(),
                    Color::red(),
                    transform,
                    |fixed_x, fixed_y| {
                        let graphic_service = graphic_service.clone();

                        // Get the translate and the scale origin
                        let translate_origin = {
                            let entity_reader = entity.read()?;
                            let translate = entity_reader
                                .get_component_by_type::<Translate2D>()?
                                .unwrap();
                            translate.vec
                        };

                        let scale_origin = {
                            let entity_reader = entity.read()?;
                            let scale = entity_reader.get_component_by_type::<Scale2D>()?.unwrap();
                            scale.vec
                        };

                        let mutation_service_2 = mutation_service.clone();
                        Ok((
                            Box::new(move |action| {
                                let (cursor_pos, start_pos) = {
                                    let graphic_service_reader = graphic_service.read();
                                    (
                                        graphic_service_reader.viewport_position_to_world_position(
                                            action.cursor_pos.0,
                                            action.cursor_pos.1,
                                        ),
                                        graphic_service_reader.viewport_position_to_world_position(
                                            action.start_pos.0,
                                            action.start_pos.1,
                                        ),
                                    )
                                };

                                let entity_writer = entity.write()?;

                                // Move the entity with the cursor
                                let mut translate = entity_writer
                                    .get_component_by_type_mut::<Translate2D>()?
                                    .unwrap();

                                let cursor_movement = cursor_pos - start_pos;
                                translate.vec = translate_origin + cursor_movement / 2.0;

                                // Resize the entity with the cursor
                                let mut scale = entity_writer
                                    .get_component_by_type_mut::<Scale2D>()?
                                    .unwrap();

                                scale.vec.x = if fixed_x {
                                    scale_origin.x + cursor_movement.x
                                } else {
                                    scale_origin.x - cursor_movement.x
                                };

                                scale.vec.y = if fixed_y {
                                    scale_origin.y + cursor_movement.y
                                } else {
                                    scale_origin.y - cursor_movement.y
                                };

                                Ok(())
                            }),
                            Box::new(move |_| {
                                let mut mutation_service = mutation_service_2.write();

                                // Get current values
                                let translate_current = {
                                    let entity_reader = entity.read()?;
                                    let translate = entity_reader
                                        .get_component_by_type::<Translate2D>()?
                                        .unwrap();
                                    translate.vec
                                };

                                let scale_current = {
                                    let entity_reader = entity.read()?;
                                    let scale =
                                        entity_reader.get_component_by_type::<Scale2D>()?.unwrap();
                                    scale.vec
                                };

                                // Get component references
                                let translate_component =
                                    entity.get_components_by_type::<Translate2D>()?.remove(0);

                                let scale_component =
                                    entity.get_components_by_type::<Scale2D>()?.remove(0);

                                // Store the mutations
                                mutation_service.push_action((
                                    SetFieldMutation::new(
                                        Box::new(translate_component),
                                        "vec".to_string(),
                                        translate_current.into_script_value()?,
                                    ),
                                    SetFieldMutation::new(
                                        Box::new(scale_component),
                                        "vec".to_string(),
                                        scale_current.into_script_value()?,
                                    ),
                                ))?;

                                Ok(())
                            }),
                        ))
                    },
                )?;
            }

            let center = (bottom_left + top_right) / 2.0;
            let size = top_right - bottom_left;
            let mutation_service_2 = mutation_service.clone();
            gizmos_service.draw_move_helper(
                center,
                size,
                Color::green(),
                Color::red(),
                transform,
                move |move_x, move_y| {
                    let graphic_service = graphic_service.clone();

                    // Get the translate origin
                    let translate_origin = {
                        let entity_reader = entity.read()?;
                        let translate = entity_reader
                            .get_component_by_type::<Translate2D>()?
                            .unwrap();
                        translate.vec
                    };

                    let mutation_service = mutation_service_2.clone();
                    Ok((
                        Box::new(move |action| {
                            let (cursor_pos, start_pos) = {
                                let graphic_service_reader = graphic_service.read();
                                (
                                    graphic_service_reader.viewport_position_to_world_position(
                                        action.cursor_pos.0,
                                        action.cursor_pos.1,
                                    ),
                                    graphic_service_reader.viewport_position_to_world_position(
                                        action.start_pos.0,
                                        action.start_pos.1,
                                    ),
                                )
                            };

                            let entity_writer = entity.write()?;
                            let mut translate = entity_writer
                                .get_component_by_type_mut::<Translate2D>()?
                                .unwrap();

                            // Move the entity with the cursor
                            let cursor_movement = cursor_pos - start_pos;
                            if move_x {
                                translate.vec.x = translate_origin.x + cursor_movement.x;
                            }

                            if move_y {
                                translate.vec.y = translate_origin.y + cursor_movement.y;
                            }

                            Ok(())
                        }),
                        Box::new(move |_| {
                            let mut mutation_service = mutation_service.write();

                            // Get current values
                            let translate_current = {
                                let entity_reader = entity.read()?;
                                let translate = entity_reader
                                    .get_component_by_type::<Translate2D>()?
                                    .unwrap();
                                translate.vec
                            };

                            // Get component references
                            let translate_component =
                                entity.get_components_by_type::<Translate2D>()?.remove(0);

                            // Store the mutations
                            mutation_service.push_action(SetFieldMutation::new(
                                Box::new(translate_component),
                                "vec".to_string(),
                                translate_current.into_script_value()?,
                            ))?;

                            Ok(())
                        }),
                    ))
                },
            )?;
        }
    }

    Ok(())
}
