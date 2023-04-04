use crate::ui::elements::UIContext;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::Arc;
use fruity_game_engine::FruityResult;
use std::any::Any;

#[derive(FruityAny)]
pub struct ListView {
    pub items: Vec<Arc<dyn Any + Send + Sync>>,
    pub render_item: Arc<dyn Fn(&mut UIContext, &dyn Any) -> FruityResult<UIElement> + Send + Sync>,
}

impl UIWidget for ListView {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}
