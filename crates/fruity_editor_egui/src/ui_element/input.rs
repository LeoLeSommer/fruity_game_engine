use crate::ui_element::app::DrawContext;
use crate::SecondaryActionState;
use egui::epaint;
use egui::CursorIcon;
use egui::Id;
use egui::LayerId;
use egui::Order;
use egui::Response;
use egui::Sense;
use egui::Shape;
use egui::Ui;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::input::Button;
use fruity_editor::ui::elements::input::Checkbox;
use fruity_editor::ui::elements::input::FloatInput;
use fruity_editor::ui::elements::input::ImageButton;
use fruity_editor::ui::elements::input::Input;
use fruity_editor::ui::elements::input::IntegerInput;
use fruity_editor::ui::hooks::use_state;
use fruity_editor::ui::hooks::use_write_service;
use fruity_game_engine::Arc;
use fruity_game_engine::FruityResult;
use fruity_game_engine::Mutex;
use fruity_graphic_wgpu::resources::texture_resource::WgpuTextureResource;
use lazy_static::*;
use std::any::Any;

lazy_static! {
    static ref CURRENT_DRAGGED_ITEM: Mutex::<Option<Arc<dyn Any + Send + Sync>>> = Mutex::new(None);
}

pub fn draw_button<'a>(
    elem: Button,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    _draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let response = ui.add_enabled(elem.enabled, egui::Button::new(elem.label.clone()));

    if response.clicked() {
        (elem.on_click)(ctx)?;
    }

    if elem.secondary_actions.len() > 0 {
        if response.secondary_clicked() {
            let mut secondary_action_state = use_write_service::<SecondaryActionState>(&ctx);
            secondary_action_state.display_secondary_actions(
                ui,
                response.clone(),
                elem.secondary_actions.clone(),
            )
        }
    }

    // Handle drag & drop
    if let Some(drag_item) = &elem.drag_item {
        drag_source(
            ui,
            Id::new("item").with(ctx.local_index()),
            response.clone(),
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock();
                *current_dragged_item = Some(drag_item.clone());

                Ok(())
            },
            |ui| {
                ui.add_enabled(false, egui::Button::new(elem.label.clone()));

                Ok(())
            },
        )?;
    }

    if let Some(on_drag) = &elem.on_drag {
        let accept_dragged = if let Some(accept_drag) = &elem.accept_drag {
            let current_dragged_item = CURRENT_DRAGGED_ITEM.lock();

            if let Some(current_dragged_item) = current_dragged_item.deref() {
                accept_drag(ctx, current_dragged_item.deref())?
            } else {
                false
            }
        } else {
            false
        };

        drag_target(
            ui,
            Id::new("item").with(ctx.local_index()),
            accept_dragged,
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock();

                if let Some(current_dragged_item) = current_dragged_item.take() {
                    on_drag(ctx, current_dragged_item.deref())?;
                }

                Ok(())
            },
            response.clone(),
        )?;
    }

    Ok(())
}

pub fn draw_image_button<'a>(
    elem: ImageButton,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let egui_texture_id = {
        let image = elem.image.read();
        let image = image.downcast_ref::<WgpuTextureResource>();

        draw_ctx.egui_rpass.egui_texture_from_wgpu_texture(
            draw_ctx.device,
            &image.get_view(),
            wgpu::FilterMode::Linear,
        )
    };

    let response = ui.add(egui::ImageButton::new(
        egui_texture_id,
        egui::Vec2::new(elem.width, elem.height),
    ));

    if response.clicked() {
        (elem.on_click)(ctx)?;
    }

    // Handle drag & drop
    if let Some(drag_item) = &elem.drag_item {
        drag_source(
            ui,
            Id::new("item").with(ctx.local_index()),
            response.clone(),
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock();
                *current_dragged_item = Some(drag_item.clone());

                Ok(())
            },
            |ui| {
                ui.add(egui::ImageButton::new(
                    egui_texture_id,
                    egui::Vec2::new(elem.width, elem.height),
                ));

                Ok(())
            },
        )?;
    }

    if let Some(on_drag) = &elem.on_drag {
        let accept_dragged = if let Some(accept_drag) = &elem.accept_drag {
            let current_dragged_item = CURRENT_DRAGGED_ITEM.lock();

            if let Some(current_dragged_item) = current_dragged_item.deref() {
                accept_drag(ctx, current_dragged_item.deref())?
            } else {
                false
            }
        } else {
            false
        };

        drag_target(
            ui,
            Id::new("item").with(ctx.local_index()),
            accept_dragged,
            move || {
                let mut current_dragged_item = CURRENT_DRAGGED_ITEM.lock();

                if let Some(current_dragged_item) = current_dragged_item.take() {
                    on_drag(ctx, current_dragged_item.deref())?;
                }

                Ok(())
            },
            response.clone(),
        )?;
    }

    Ok(())
}

fn drag_source(
    ui: &mut Ui,
    id: Id,
    response: Response,
    on_drag: impl FnOnce() -> FruityResult<()>,
    body: impl FnOnce(&mut Ui) -> FruityResult<()>,
) -> FruityResult<()> {
    let is_being_dragged = ui.memory(|memory| memory.is_being_dragged(id));
    let response = ui.interact(response.rect, id, Sense::drag());

    if response.drag_started() {
        on_drag()?;
    }

    if !is_being_dragged {
        if response.hovered() {
            ui.output_mut(|output| output.cursor_icon = CursorIcon::Grab);
        }
    } else {
        ui.output_mut(|output| output.cursor_icon = CursorIcon::Grabbing);

        // Paint the body to a new layer:
        let layer_id = LayerId::new(Order::Tooltip, id);
        let response = ui.with_layer_id(layer_id, body).response;

        // Move the visuals of the body to where the mouse is.
        if let Some(pointer_pos) = ui.input(|input| input.pointer.interact_pos()) {
            let delta = pointer_pos - response.rect.center();
            ui.ctx().translate_layer(layer_id, delta);
        }
    }

    Ok(())
}

fn drag_target(
    ui: &mut Ui,
    id: Id,
    accept_dragged: bool,
    on_drag: impl FnOnce() -> FruityResult<()>,
    response: Response,
) -> FruityResult<()> {
    let response = ui.interact(response.rect, id, Sense::hover());
    let is_being_dragged = ui.memory(|memory| memory.is_anything_being_dragged());
    let where_to_put_background = ui.painter().add(Shape::Noop);

    let style = ui.visuals().widgets.active;

    if is_being_dragged && accept_dragged {
        if response.hovered() {
            ui.painter().set(
                where_to_put_background,
                epaint::RectShape::stroke(response.rect, style.rounding, style.fg_stroke),
            );
        } else {
            ui.painter().set(
                where_to_put_background,
                epaint::RectShape::stroke(response.rect, style.rounding, style.bg_stroke),
            );
        }
    }

    if !is_being_dragged && response.hovered() {
        on_drag()?;
    }

    Ok(())
}

pub fn draw_input<'a>(
    elem: Input,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    _draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let (mut input_value, set_input_value) = use_state(ctx, String::default());

    let response =
        ui.add(egui::TextEdit::singleline(&mut input_value).hint_text(&elem.placeholder));

    if response.lost_focus() {
        (elem.on_change)(ctx, &input_value)?;
    }

    if response.changed() {
        (elem.on_edit)(ctx, &input_value)?;
        set_input_value(input_value);
    }

    if !response.has_focus() {
        set_input_value(elem.value);
    }

    Ok(())
}

pub fn draw_integer_input<'a>(
    elem: IntegerInput,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let input = Input {
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |ctx, value| {
            if let Ok(value) = value.parse::<i64>() {
                (elem.on_change)(ctx, value)?;
            }

            Ok(())
        }),
        ..Default::default()
    };

    draw_input(input, ctx, ui, draw_ctx)
}

pub fn draw_float_input<'a>(
    elem: FloatInput,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let input = Input {
        value: elem.value.to_string(),
        placeholder: "".to_string(),
        on_change: Arc::new(move |ctx, value| {
            if let Ok(value) = value.parse::<f64>() {
                (elem.on_change)(ctx, value)?;
            }

            Ok(())
        }),
        ..Default::default()
    };

    draw_input(input, ctx, ui, draw_ctx)
}

pub fn draw_checkbox<'a>(
    elem: Checkbox,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    _draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let mut new_value = elem.value;
    ui.add(egui::Checkbox::new(&mut new_value, &elem.label));

    if new_value != elem.value {
        (elem.on_change)(ctx, new_value)?;
    }

    Ok(())
}
