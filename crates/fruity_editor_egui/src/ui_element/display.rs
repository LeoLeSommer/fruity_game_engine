use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::display::Popup;
use fruity_editor::ui::elements::display::Text;
use fruity_game_engine::FruityResult;

pub fn draw_text<'a>(
    elem: Text,
    _ctx: &mut UIContext,
    ui: &mut egui::Ui,
    _draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    ui.add(egui::Label::new(elem.text));

    Ok(())
}

pub fn draw_popup<'a>(
    elem: Popup,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let popup_id = ui.make_persistent_id(ctx.local_index());

    let response =
        ui.allocate_response(egui::vec2(ui.available_size().x, 0.0), egui::Sense::click());
    egui::popup::popup_below_widget(ui, popup_id, &response, |ui| {
        draw_element(elem.content, ctx, ui, draw_ctx)
    });
    ui.memory_mut(|memory| memory.open_popup(popup_id));

    Ok(())
}
