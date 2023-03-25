use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_game_engine::any::FruityAny;

#[derive(FruityAny)]
pub struct Profiling {}

impl UIWidget for Profiling {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}
