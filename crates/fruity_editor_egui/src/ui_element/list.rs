use crate::ui_element::app::DrawContext;
use crate::ui_element::draw_element;
use egui::ScrollArea;
use fruity_editor::ui::context::UIContext;
use fruity_editor::ui::elements::list::ListView;
use fruity_game_engine::FruityResult;
use std::ops::Deref;

pub fn draw_list_view<'a>(
    elem: ListView,
    ctx: &mut UIContext,
    ui: &mut egui::Ui,
    draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    let scroll_area = ScrollArea::vertical().auto_shrink([false; 2]);

    let render_item = elem.render_item.clone();
    scroll_area
        .show(ui, |ui| {
            ui.vertical(|ui| {
                elem.items.into_iter().try_for_each(|item| {
                    let item = render_item(ctx, item.deref())?;

                    draw_element(item, ctx, ui, draw_ctx)
                })
            })
            .inner
        })
        .inner?;

    Ok(())
}
