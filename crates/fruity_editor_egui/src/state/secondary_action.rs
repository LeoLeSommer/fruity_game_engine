use fruity_editor::editor_menu_service::MenuItem;
use fruity_editor::ui::context::UIContext;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::{export_impl, export_struct, FruityResult};

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct SecondaryActionState {
    below_widget: Option<egui::Response>,
    secondary_actions: Vec<MenuItem>,
}

impl Default for SecondaryActionState {
    fn default() -> Self {
        Self {
            below_widget: None,
            secondary_actions: Vec::new(),
        }
    }
}

#[export_impl]
impl SecondaryActionState {
    pub fn draw_secondary_actions(&self, ctx: &UIContext, ui: &mut egui::Ui) -> FruityResult<()> {
        if let Some(below_widget) = &self.below_widget {
            if let Some(result) = egui::popup::popup_below_widget(
                ui,
                egui::Id::new("secondary_actions"),
                &below_widget,
                |ui| {
                    ui.vertical(|ui| {
                        self.secondary_actions
                            .iter()
                            .try_for_each(|secondary_action| {
                                if ui
                                    .small_button(secondary_action.label.to_string())
                                    .clicked()
                                {
                                    let on_click = secondary_action.action.clone();
                                    on_click(ctx)?;
                                }

                                FruityResult::Ok(())
                            })
                    })
                    .inner
                },
            ) {
                result?
            }
        }

        Ok(())
    }

    pub fn display_secondary_actions(
        &mut self,
        ui: &mut egui::Ui,
        below_widget: egui::Response,
        secondary_actions: Vec<MenuItem>,
    ) {
        self.below_widget = Some(below_widget);
        self.secondary_actions = secondary_actions;
        ui.memory_mut(|memory| memory.open_popup(egui::Id::new("secondary_actions")));
    }
}
