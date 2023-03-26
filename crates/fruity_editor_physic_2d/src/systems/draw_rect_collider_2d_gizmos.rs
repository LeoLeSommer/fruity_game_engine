use crate::ColliderState;
use fruity_editor::mutations::mutation_service::MutationService;
use fruity_editor::mutations::set_field_mutation::SetFieldMutation;
use fruity_editor_graphic_2d::gizmos_service::GizmosService;
use fruity_game_engine::inject::{Const, Ref};
use fruity_game_engine::script_value::convert::TryIntoScriptValue;
use fruity_game_engine::FruityResult;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::Color;
use fruity_graphic_2d::components::transform_2d::Transform2D;
use fruity_graphic_2d::graphic_2d_service::Graphic2dService;
use fruity_physic_2d::components::rect_collider::RectCollider;

pub fn draw_rectangle_collider_2d_gizmos(
    collider_state: Const<ColliderState>,
    gizmos_service: Const<GizmosService>,
    graphic_2d_service: Ref<Graphic2dService>,
    graphic_service: Ref<dyn GraphicService>,
    mutation_service: Ref<MutationService>,
) -> FruityResult<()> {
    if !collider_state.is_editing_collider() {
        return Ok(());
    }

    let entity = if let Some(entity) = collider_state.get_editing_entity() {
        entity
    } else {
        return Ok(());
    };

    if let Some(collider) = collider_state.get_editing_collider() {
        let transform = {
            let entity_reader = entity.read()?;
            let transform = entity_reader.get_component_by_type::<Transform2D>()?;

            if let Some(transform) = transform.map(|transform| transform.transform) {
                transform
            } else {
                return Ok(());
            }
        };

        if let Ok(rect_collider) = collider.clone().read_typed::<RectCollider>() {
            // Draw the collider
            {
                let graphic_2d_service_reader = graphic_2d_service.read();
                graphic_2d_service_reader.draw_rect(
                    rect_collider.bottom_left,
                    rect_collider.top_right,
                    4,
                    Color::overlay(),
                    Color::green(),
                    1000,
                    transform,
                );
            }

            // Draw the gizmos to resize the collider
            gizmos_service.draw_resize_helper(
                rect_collider.bottom_left,
                rect_collider.top_right,
                Color::green(),
                Color::red(),
                transform,
                move |fixed_x, fixed_y| {
                    let graphic_service = graphic_service.clone();
                    let collider = collider.clone();
                    let collider_2 = collider.clone();

                    // Get the rect origin
                    let (bottom_left_origin, top_right_origin) = {
                        let collider = collider.read_typed::<RectCollider>().unwrap();
                        (collider.bottom_left, collider.top_right)
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

                            let cursor_movement = cursor_pos - start_pos;

                            // Move the entity with the cursor
                            let mut collider = collider.write_typed::<RectCollider>().unwrap();
                            collider.bottom_left = bottom_left_origin + cursor_movement / 2.0;

                            // Resize the entity with the cursor
                            collider.top_right.x = if fixed_x {
                                top_right_origin.x + cursor_movement.x
                            } else {
                                top_right_origin.x - cursor_movement.x
                            };

                            collider.top_right.y = if fixed_y {
                                top_right_origin.y + cursor_movement.y
                            } else {
                                top_right_origin.y - cursor_movement.y
                            };

                            Ok(())
                        }),
                        Box::new(move |_| {
                            let collider = collider_2.clone();

                            let mut mutation_service = mutation_service_2.write();

                            // Get current values
                            let (bottom_left_current, top_right_current) = {
                                let collider = collider.read_typed::<RectCollider>().unwrap();
                                (collider.bottom_left, collider.top_right)
                            };

                            // Store the mutations
                            mutation_service.push_action((
                                SetFieldMutation::new(
                                    Box::new(collider.clone()),
                                    "bottom_left".to_string(),
                                    bottom_left_current.into_script_value()?,
                                ),
                                SetFieldMutation::new(
                                    Box::new(collider.clone()),
                                    "top_right".to_string(),
                                    top_right_current.into_script_value()?,
                                ),
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
