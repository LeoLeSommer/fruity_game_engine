use crate::ui::context::UIContext;
use crate::ui::elements::pane::UIPaneSide;
use crate::ui::elements::UIElement;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::Arc;
use fruity_game_engine::{export_impl, export_struct, FruityResult};
use std::fmt::Debug;

pub struct PanelItem {
    pub label: String,
    pub default_side: UIPaneSide,
    pub renderer: Arc<dyn Fn(&mut UIContext) -> FruityResult<UIElement> + Send + Sync>,
}

#[derive(FruityAny)]
#[export_struct]
pub struct EditorPanelsService {
    panels: Vec<PanelItem>,
}

#[export_impl]
impl EditorPanelsService {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self { panels: Vec::new() }
    }

    pub fn add_panel(
        &mut self,
        label: &str,
        default_side: UIPaneSide,
        renderer: impl Fn(&mut UIContext) -> FruityResult<UIElement> + Send + Sync + 'static,
    ) {
        self.panels.push(PanelItem {
            label: label.to_string(),
            default_side,
            renderer: Arc::new(renderer),
        });
    }

    pub fn iter_panels(&self) -> impl Iterator<Item = &PanelItem> {
        self.panels.iter()
    }
}

impl Debug for EditorPanelsService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
