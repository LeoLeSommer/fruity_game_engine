use crate::component::component::Component;
use crate::entity::archetype::component_array::ComponentArray;
use crate::entity::archetype::component_collection::ComponentCollection;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::FieldInfo;
use fruity_game_engine::introspect::IntrospectObject;
use fruity_game_engine::introspect::MethodInfo;
use fruity_game_engine::introspect::SetterCaller;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::utils::introspect::cast_introspect_mut;
use fruity_game_engine::utils::introspect::cast_introspect_ref;
use std::collections::HashMap;
use std::rc::Rc;

/// A wrapper for components that come from scripting languages as serialized
#[derive(Debug, Clone, FruityAny)]
pub struct ScriptComponent {
    class_name: String,
    fields: HashMap<String, ScriptValue>,
}

impl ScriptComponent {
    /// Returns a ScriptComponent
    pub fn new(class_name: String, fields: HashMap<String, ScriptValue>) -> ScriptComponent {
        ScriptComponent { class_name, fields }
    }
}

unsafe impl Sync for ScriptComponent {}
unsafe impl Send for ScriptComponent {}

impl Component for ScriptComponent {
    fn get_collection(&self) -> Box<dyn ComponentCollection> {
        Box::new(ComponentArray::<ScriptComponent>::new())
    }

    fn duplicate(&self) -> Box<dyn Component> {
        Box::new(self.clone())
    }
}

impl IntrospectObject for ScriptComponent {
    fn get_class_name(&self) -> String {
        self.class_name.clone()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        vec![]
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.fields
            .iter()
            .map(|(key, _)| {
                let key1 = key.clone();
                let key2 = key.clone();

                FieldInfo {
                    name: key.clone(),
                    getter: Rc::new(move |this| {
                        let this = cast_introspect_ref::<ScriptComponent>(this)?;
                        Ok(this.fields.get(&key1).unwrap().clone())
                    }),
                    setter: SetterCaller::Mut(Rc::new(move |this, value| {
                        let this = cast_introspect_mut::<ScriptComponent>(this)?;
                        this.fields.insert(key2.clone(), value);

                        Ok(())
                    })),
                }
            })
            .collect::<Vec<_>>()
    }
}
