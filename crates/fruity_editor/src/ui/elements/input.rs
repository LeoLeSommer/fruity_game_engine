use crate::editor_menu_service::MenuItem;
use crate::ui::elements::UIContext;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::FruityResult;
use fruity_graphic::resources::texture_resource::TextureResource;
use std::any::Any;
use std::sync::Arc;

#[derive(FruityAny)]
pub struct Button {
    pub label: String,
    pub enabled: bool,
    pub on_click: Arc<dyn Fn(&UIContext) -> FruityResult<()> + Send + Sync>,
    pub secondary_actions: Vec<MenuItem>,
    pub drag_item: Option<Arc<dyn Any + Send + Sync>>,
    pub accept_drag: Option<Arc<dyn Fn(&UIContext, &dyn Any) -> FruityResult<bool> + Send + Sync>>,
    pub on_drag: Option<Arc<dyn Fn(&UIContext, &dyn Any) -> FruityResult<()> + Send + Sync>>,
}

impl Default for Button {
    fn default() -> Self {
        Button {
            label: Default::default(),
            enabled: true,
            on_click: Arc::new(|_| Ok(())),
            secondary_actions: Vec::new(),
            drag_item: None,
            accept_drag: None,
            on_drag: None,
        }
    }
}

impl UIWidget for Button {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
pub struct ImageButton {
    pub image: ResourceReference<dyn TextureResource>,
    pub enabled: bool,
    pub on_click: Arc<dyn Fn(&UIContext) -> FruityResult<()> + Send + Sync>,
    pub width: f32,
    pub height: f32,
    pub drag_item: Option<Arc<dyn Any + Send + Sync>>,
    pub accept_drag: Option<Arc<dyn Fn(&UIContext, &dyn Any) -> FruityResult<bool> + Send + Sync>>,
    pub on_drag: Option<Arc<dyn Fn(&UIContext, &dyn Any) -> FruityResult<bool> + Send + Sync>>,
}

impl UIWidget for ImageButton {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
pub struct Input {
    pub value: String,
    pub placeholder: String,
    pub on_change: Arc<dyn Fn(&UIContext, &str) -> FruityResult<()> + Send + Sync>,
    pub on_edit: Arc<dyn Fn(&UIContext, &str) -> FruityResult<()> + Send + Sync>,
}

impl Default for Input {
    fn default() -> Self {
        Input {
            value: String::default(),
            placeholder: String::default(),
            on_change: Arc::new(|_, _| Ok(())),
            on_edit: Arc::new(|_, _| Ok(())),
        }
    }
}

impl UIWidget for Input {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
pub struct IntegerInput {
    pub value: i64,
    pub on_change: Arc<dyn Fn(&UIContext, i64) -> FruityResult<()> + Send + Sync>,
}

impl UIWidget for IntegerInput {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
pub struct FloatInput {
    pub value: f64,
    pub on_change: Arc<dyn Fn(&UIContext, f64) -> FruityResult<()> + Send + Sync>,
}

impl UIWidget for FloatInput {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}

#[derive(FruityAny)]
pub struct Checkbox {
    pub label: String,
    pub value: bool,
    pub on_change: Arc<dyn Fn(&UIContext, bool) -> FruityResult<()> + Send + Sync>,
}

impl UIWidget for Checkbox {
    fn elem(self) -> UIElement {
        UIElement::from_widget(self)
    }
}
