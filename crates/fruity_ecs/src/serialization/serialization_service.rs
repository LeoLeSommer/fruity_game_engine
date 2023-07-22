use super::Deserialize;
use crate::{component::Component, entity::EntityId};
use fruity_game_engine::{
    any::FruityAny,
    export_impl, export_struct,
    introspect::IntrospectFields,
    javascript::JsIntrospectObject,
    resource::ResourceContainer,
    script_value::{ScriptValue, TryIntoScriptValue},
    settings::Settings,
    FruityError, FruityResult,
};
use std::collections::HashMap;

/// Utility used to deserialize objects, mostly used to restore snapshot
#[derive(FruityAny)]
#[export_struct]
pub struct SerializationService {
    resource_container: ResourceContainer,
    factories: HashMap<
        String,
        Box<
            dyn Fn(
                    &Settings,
                    &ResourceContainer,
                    &HashMap<u64, EntityId>,
                ) -> FruityResult<ScriptValue>
                + Send
                + Sync,
        >,
    >,
}

#[export_impl]
impl SerializationService {
    /// Returns an SerializationService
    pub fn new(resource_container: ResourceContainer) -> SerializationService {
        SerializationService {
            resource_container,
            factories: HashMap::new(),
        }
    }

    /// Register a new deserialize type
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    ///
    /// # Generic Arguments
    /// * `T` - The type of the object
    ///
    pub fn register<T>(&mut self)
    where
        T: Deserialize + TryIntoScriptValue,
    {
        self.factories.insert(
            T::get_identifier(),
            Box::new(|script_value, resource_container, local_id_to_entity_id| {
                let result =
                    T::deserialize(script_value, &resource_container, local_id_to_entity_id)?;

                result.into_script_value()
            }),
        );
    }

    /// Register a new deserialize type
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    ///
    /// # Generic Arguments
    /// * `T` - The type of the object
    ///
    pub fn register_component<T>(&mut self)
    where
        T: Deserialize + Component + TryIntoScriptValue,
    {
        self.factories.insert(
            T::get_identifier(),
            Box::new(|script_value, resource_container, local_id_to_entity_id| {
                let result = Box::new(T::deserialize(
                    script_value,
                    resource_container,
                    local_id_to_entity_id,
                )?) as Box<dyn Component>;

                result.into_script_value()
            }),
        );
    }

    /// Register a new deserialize type from a function
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `constructor` - The constructor
    ///
    pub fn register_func(
        &mut self,
        object_type: &str,
        instantiate: fn(
            &Settings,
            &ResourceContainer,
            &HashMap<u64, EntityId>,
        ) -> FruityResult<ScriptValue>,
    ) {
        self.factories
            .insert(object_type.to_string(), Box::new(instantiate));
    }

    /// Instantiate an object from it's factory
    ///
    /// # Arguments
    /// * `object_type` - The object type identifier
    /// * `serialized` - A serialized value that will populate the new component
    ///
    pub fn instantiate(
        &self,
        serialized: &Settings,
        local_id_to_entity_id: &HashMap<u64, EntityId>,
    ) -> FruityResult<ScriptValue> {
        Ok(match serialized.clone() {
            Settings::F64(value) => ScriptValue::F64(value),
            Settings::Bool(value) => ScriptValue::Bool(value),
            Settings::String(value) => ScriptValue::String(value),
            Settings::Array(value) => ScriptValue::Array(
                value
                    .iter()
                    .map(|item| self.instantiate(item, local_id_to_entity_id))
                    .try_collect::<Vec<ScriptValue>>()?,
            ),
            Settings::Object(value) => {
                let class_name = value.get("class_name").ok_or_else(|| {
                    FruityError::GenericFailure("Missing class_name in object".to_string())
                })?;

                let class_name = if let Settings::String(class_name) = class_name {
                    Ok(class_name.clone())
                } else {
                    Err(FruityError::GenericFailure(
                        "class_name must be a string".to_string(),
                    ))
                }?;

                let fields = value.get("fields").ok_or(FruityError::GenericFailure(
                    "Missing fields in object".to_string(),
                ))?;

                if let Some(factory) = self.factories.get(&class_name) {
                    factory(&fields, &self.resource_container, local_id_to_entity_id)?
                } else {
                    let fields = if let Settings::Object(fields) = fields {
                        Ok(fields.clone())
                    } else {
                        Err(FruityError::GenericFailure(
                            "fields must be an object".to_string(),
                        ))
                    }?;

                    let mut result = JsIntrospectObject::new(class_name)?;

                    fields.iter().try_for_each(|(key, value)| {
                        result.set_field_value(key, self.instantiate(value, local_id_to_entity_id)?)
                    })?;

                    ScriptValue::Object(Box::new(result))
                }
            }
            Settings::Null => ScriptValue::Null,
        })
    }
}

impl std::fmt::Debug for SerializationService {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
