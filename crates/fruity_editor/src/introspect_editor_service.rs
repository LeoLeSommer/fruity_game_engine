use crate::ui::context::UIContext;
use crate::ui::elements::UIElement;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::Arc;
use fruity_game_engine::{export_impl, export_struct, FruityResult};
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;

pub type IntrospectFieldEditor = Arc<
    dyn Fn(
            &mut UIContext,
            &str,
            Box<dyn ScriptObject>,
            Box<
                dyn Fn(&UIContext, Box<dyn ScriptObject>) -> FruityResult<()>
                    + Send
                    + Sync
                    + 'static,
            >,
        ) -> FruityResult<UIElement>
        + Send
        + Sync
        + 'static,
>;

#[derive(FruityAny)]
#[export_struct]
pub struct IntrospectEditorService {
    component_field_editors: HashMap<TypeId, IntrospectFieldEditor>,
}

#[export_impl]
impl IntrospectEditorService {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        IntrospectEditorService {
            component_field_editors: HashMap::new(),
        }
    }

    pub fn register_field_editor<T, F>(&mut self, editor: F)
    where
        T: 'static,
        F: Fn(
                &mut UIContext,
                &str,
                Box<dyn ScriptObject>,
                Box<
                    dyn Fn(&UIContext, Box<dyn ScriptObject>) -> FruityResult<()>
                        + Send
                        + Sync
                        + 'static,
                >,
            ) -> FruityResult<UIElement>
            + Send
            + Sync
            + 'static,
    {
        let editor = Arc::new(editor);
        self.component_field_editors
            .insert(TypeId::of::<T>(), editor.clone());
    }

    pub fn get_field_editor(&self, type_id: TypeId) -> Option<IntrospectFieldEditor> {
        self.component_field_editors
            .get(&type_id)
            .map(|draw| draw.clone())
    }
}

impl Debug for IntrospectEditorService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
