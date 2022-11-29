use crate::entity::archetype::component_collection::ComponentCollection;
pub use fruity_ecs_macro::Component;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::introspect::IntrospectObject;
use fruity_game_engine::javascript::JsIntrospectObject;
use fruity_game_engine::script_value::convert::TryFromScriptValue;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::send_wrapper::SendWrapper;
use fruity_game_engine::{FruityError, FruityResult, FruityStatus};
use std::fmt::Debug;
use std::ops::Deref;

use super::script_component::ScriptComponent;

/// An abstraction over a component, should be implemented for every component
pub trait StaticComponent {
    /// Return the class type name
    fn get_component_name() -> &'static str;
}

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectObject + Debug + Send + Sync {
    /// Get a collection to store this component in the archetype
    fn get_collection(&self) -> Box<dyn ComponentCollection>;

    /// Create a new component that is a clone of self
    fn duplicate(&self) -> Box<dyn Component>;
}

impl Clone for Box<dyn Component> {
    fn clone(&self) -> Self {
        self.duplicate()
    }
}

/// An container for a component without knowing the instancied type
#[derive(FruityAny, Debug)]
pub struct AnyComponent {
    component: Box<dyn Component>,
}

impl AnyComponent {
    /// Returns an AnyComponent
    pub fn new(component: impl Component) -> AnyComponent {
        AnyComponent {
            component: Box::new(component),
        }
    }

    /// Returns an AnyComponent
    pub fn into_box(self) -> Box<dyn Component> {
        self.component
    }
}

impl From<Box<dyn Component>> for AnyComponent {
    fn from(component: Box<dyn Component>) -> Self {
        Self { component }
    }
}

impl From<JsIntrospectObject> for AnyComponent {
    fn from(value: JsIntrospectObject) -> Self {
        Self {
            component: Box::new(ScriptComponent(SendWrapper::new(value))),
        }
    }
}

impl Deref for AnyComponent {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.component.as_ref()
    }
}

impl TryFromScriptValue for AnyComponent {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.downcast::<Box<dyn Component>>() {
                Ok(value) => Ok(AnyComponent::from(*value)),
                Err(value) => match value.downcast::<JsIntrospectObject>() {
                    Ok(value) => Ok(AnyComponent::from(*value)),
                    Err(value) => Err(FruityError::new(
                        FruityStatus::InvalidArg,
                        format!("Couldn't convert a {} to Component", value.get_type_name(),),
                    )),
                },
            },
            value => Err(FruityError::new(
                FruityStatus::InvalidArg,
                format!("Couldn't convert {:?} to native object", value),
            )),
        }
    }
}
