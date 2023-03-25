use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use egui::menu;
use egui::Button;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::menu::MenuBar;
use fruity_editor::ui::elements::menu::MenuSection;
use fruity_editor::ui::hooks::use_read_service;
use fruity_game_engine::FruityResult;
use fruity_input::input_service::InputService;

pub fn draw_menu_bar<'a>(
    elem: MenuBar,
    ctx: &mut UIContext,
    _ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    egui::TopBottomPanel::top("menu_bar")
        .show(&draw_ctx.platform.context(), |ui| {
            menu::bar(ui, |ui| {
                elem.children
                    .into_iter()
                    .try_for_each(|child| draw_element(child, ctx, ui, draw_ctx))
            })
            .inner
        })
        .inner
}

pub fn draw_menu_section<'a>(
    elem: MenuSection,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    _draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    // Draw menu
    if let Some(response) = ui
        .menu_button(elem.label, {
            let items = elem.items.clone();
            |ui| {
                items.into_iter().try_for_each({
                    |item| {
                        let enabled = match item.options.is_enabled {
                            Some(is_enabled) => is_enabled(ctx)?,
                            None => true,
                        };

                        if ui.add_enabled(enabled, Button::new(item.label)).clicked() {
                            (item.action)(ctx)?;
                        }

                        FruityResult::Ok(())
                    }
                })
            }
        })
        .inner
    {
        response?;
    }

    // Handle shortcuts
    elem.items.into_iter().try_for_each({
        |item| {
            let enabled = match item.options.is_enabled {
                Some(is_enabled) => is_enabled(ctx)?,
                None => true,
            };

            if enabled {
                if let Some(shortcut) = &item.options.shortcut {
                    let input_service = use_read_service::<InputService>(ctx);
                    if input_service.is_keyboard_pressed_this_frame(shortcut.clone()) {
                        (item.action)(ctx)?;
                    }
                }
            }

            FruityResult::Ok(())
        }
    })?;

    Ok(())
}
