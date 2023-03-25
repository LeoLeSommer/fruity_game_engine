use crate::ui_element::DrawContext;
use fruity_editor::ui::context::UIContext;
use fruity_game_engine::FruityResult;

pub fn draw_profiling(
    _ctx: &mut UIContext,
    ui: &mut egui::Ui,
    _draw_ctx: &mut DrawContext,
) -> FruityResult<()> {
    puffin_egui::profiler_ui(ui);

    Ok(())
}
