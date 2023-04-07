use crate::editor_component_service::EditorComponentService;
use crate::inspector_service::InspectorService;
use crate::ui::context::UIContext;
use crate::ui::elements::layout::Empty;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use fruity_ecs::component::component_reference::AnyComponentReference;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityResult;

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct InspectorState {
    inspect_service: ResourceReference<InspectorService>,
    inspect_component_service: ResourceReference<EditorComponentService>,
    selected: Option<Box<dyn ScriptObject>>,
    temporary_disable_gizmos: bool,
    pub on_selected: Signal<Box<dyn ScriptObject>>,
    pub on_unselected: Signal<()>,
}

#[export_impl]
impl InspectorState {
    pub fn new(resource_container: ResourceContainer) -> Self {
        Self {
            inspect_service: resource_container.require::<InspectorService>(),
            inspect_component_service: resource_container.require::<EditorComponentService>(),
            selected: None,
            temporary_disable_gizmos: false,
            on_selected: Signal::new(),
            on_unselected: Signal::new(),
        }
    }

    pub fn get_selected(&self) -> Option<&Box<dyn ScriptObject>> {
        self.selected.as_ref()
    }

    #[export]
    pub fn select(&mut self, selection: Box<dyn ScriptObject>) -> FruityResult<()> {
        self.temporary_disable_gizmos = false;
        self.selected = Some(selection.duplicate());
        self.on_selected.send(selection)
    }

    #[export]
    pub fn unselect(&mut self) -> FruityResult<()> {
        self.temporary_disable_gizmos = false;
        self.selected = None;
        self.on_unselected.send(())
    }

    pub fn inspect(&self, ctx: &mut UIContext) -> FruityResult<UIElement> {
        if let Some(selected) = &self.selected {
            let inspect_service = self.inspect_service.read();
            inspect_service.inspect(ctx, selected.duplicate())
        } else {
            Ok(Empty {}.elem())
        }
    }

    pub fn inspect_component(
        &self,
        ctx: &mut UIContext,
        component: AnyComponentReference,
    ) -> FruityResult<UIElement> {
        let inspect_component_service = self.inspect_component_service.read();
        inspect_component_service.inspect(ctx, component)
    }

    #[export]
    pub fn is_gizmos_enabled(&self) -> bool {
        !self.temporary_disable_gizmos
    }

    #[export]
    pub fn temporary_display_gizmos(&mut self) {
        self.temporary_disable_gizmos = true;
    }
}
