use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::FruityResult;

use crate::components::fields::edit_introspect_fields;
use crate::ui::context::UIContext;
use crate::ui::elements::layout::Empty;
use crate::ui::elements::UIElement;
use crate::ui::elements::UIWidget;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::ops::DerefMut;
use std::sync::Arc;

#[derive(FruityAny)]
#[export_struct]
pub struct InspectorService {
    inspect_types: HashMap<
        TypeId,
        Arc<dyn Fn(&mut UIContext, Box<dyn ScriptObject>) -> FruityResult<UIElement> + Send + Sync>,
    >,
}

#[export_impl]
impl InspectorService {
    pub fn new(_resource_container: ResourceContainer) -> Self {
        Self {
            inspect_types: HashMap::new(),
        }
    }

    pub fn register_inspect_type<T: ScriptObject>(
        &mut self,
        inspect: impl Fn(&mut UIContext, &mut T) -> FruityResult<UIElement> + Send + Sync + 'static,
    ) {
        self.inspect_types.insert(
            TypeId::of::<T>(),
            Arc::new(move |ctx: &mut UIContext, obj: Box<dyn ScriptObject>| {
                match obj.as_any_box().downcast::<T>() {
                    Ok(mut obj) => inspect(ctx, obj.deref_mut()),
                    Err(_) => Ok(Empty {}.elem()),
                }
            }),
        );
    }

    pub fn inspect(
        &self,
        ctx: &mut UIContext,
        obj: Box<dyn ScriptObject>,
    ) -> FruityResult<UIElement> {
        match self.inspect_types.get(&obj.type_id()) {
            Some(inspect) => inspect(ctx, obj),
            None => edit_introspect_fields(ctx, obj),
        }
    }
}

impl Debug for InspectorService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
