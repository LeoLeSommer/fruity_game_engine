use super::UIWidget;
use crate::ui::elements::UIElement;
use fruity_game_engine::any::FruityAny;

#[derive(FruityAny, Default)]
pub struct Scene {}

impl UIWidget for Scene {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}
