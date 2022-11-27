use crate::script_value::convert::TryFromScriptValue;
use crate::settings::Settings;
use crate::world::World;
use crate::FruityResult;
use std::rc::Rc;

/// A service to manage modules loading
pub mod modules_service;

/// A module for the engine
#[derive(Clone, TryFromScriptValue, Default)]
pub struct Module {
    /// The name of the module
    pub name: String,

    /// The dependencies of the module
    pub dependencies: Vec<String>,

    /// A function that initialize the module
    pub setup: Option<Rc<dyn Fn(World, Settings) -> FruityResult<()>>>,

    /// A function that initialize the module resources
    pub load_resources: Option<Rc<dyn Fn(World, Settings) -> FruityResult<()>>>,
}

/*impl crate::script_value::convert::TryFromScriptValue for Module {
    fn from_script_value(value: crate::script_value::ScriptValue) -> crate::FruityResult<Self> {
        match value {
            crate::script_value::ScriptValue::Object(value) => {
                match value.downcast::<Self>() {
                    Ok(value) => Ok(*value),
                    Err(value) => {
                        Ok(Self {
                            name: <String>::from_script_value(value.get_field_value("name")?)?,
                            dependencies: <Vec<String>>::from_script_value(value.get_field_value("dependencies")?)?,
                            setup: <Option<Rc<dyn Fn(World, Settings) -> FruityResult<()>>>>::from_script_value(value.get_field_value("setup")?)?,
                            load_resources: <Option<Rc<dyn Fn(ResourceContainer, Settings) -> FruityResult<()>>>>::from_script_value(value.get_field_value("load_resources")?)?,
                        })
                    }
                }
            }
            _ => Err(crate::FruityError::new(
                crate::FruityStatus::InvalidArg,
                format!("Couldn't convert {:?} to native object", value),
            )),
        }
    }
}
*/
