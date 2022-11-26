use crate::entity::archetype::component_collection::ComponentCollection;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::convert::FruityFrom;
use fruity_game_engine::introspect::IntrospectObject;
use fruity_game_engine::script_value::ScriptObject;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::FruityStatus;
use std::fmt::Debug;
use std::ops::Deref;

/// An abstraction over a component, should be implemented for every component
pub trait StaticComponent {
    /// Return the class type name
    fn get_component_name() -> &'static str;
}

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectObject + ScriptObject + Debug + Send + Sync {
    /// Get a collection to store this component in the archetype
    fn get_collection(&self) -> Box<dyn ComponentCollection>;

    /// Create a new component that is a clone of self
    fn duplicate(&self) -> Box<dyn Component>;
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
    pub fn from_box(component: Box<dyn Component>) -> AnyComponent {
        AnyComponent { component }
    }

    /// Returns an AnyComponent
    pub fn into_box(self) -> Box<dyn Component> {
        self.component
    }
}

impl Deref for AnyComponent {
    type Target = dyn Component;

    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        self.component.as_ref()
    }
}

impl FruityFrom<ScriptValue> for AnyComponent {
    fn fruity_from(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.as_any_box().downcast::<AnyComponent>() {
                Ok(value) => Ok(*value),
                Err(_) => Err(FruityError::new(
                    FruityStatus::InvalidArg,
                    format!("Couldn't convert An AnyComponent to native object"),
                )),
            },
            _ => Err(FruityError::new(
                FruityStatus::InvalidArg,
                format!("Couldn't convert {:?} to native object", value),
            )),
        }
    }
}

impl IntrospectObject for AnyComponent {
    fn get_class_name(&self) -> FruityResult<String> {
        self.component.get_class_name()
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        self.component.get_field_names()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        self.component.set_field_value(name, value)
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        self.component.get_field_value(name)
    }

    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        self.component.get_const_method_names()
    }

    fn call_const_method(&self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.component.call_const_method(name, args)
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        self.component.get_mut_method_names()
    }

    fn call_mut_method(&mut self, name: &str, args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        self.component.call_mut_method(name, args)
    }
}
