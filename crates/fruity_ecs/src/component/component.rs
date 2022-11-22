use crate::component::serialized_component::ScriptComponent;
use crate::entity::archetype::component_collection::ComponentCollection;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::convert::FruityFrom;
use fruity_game_engine::introspect::FieldInfo;
use fruity_game_engine::introspect::IntrospectObject;
use fruity_game_engine::introspect::MethodCaller;
use fruity_game_engine::introspect::MethodInfo;
use fruity_game_engine::introspect::SetterCaller;
use fruity_game_engine::script_value::IntrospectObjectClone;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::utils::introspect::cast_introspect_mut;
use fruity_game_engine::utils::introspect::cast_introspect_ref;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::FruityStatus;
use std::fmt::Debug;
use std::ops::Deref;
use std::rc::Rc;

/// An abstraction over a component, should be implemented for every component
pub trait StaticComponent {
    /// Return the class type name
    fn get_component_name() -> &'static str;
}

/// An abstraction over a component, should be implemented for every component
pub trait Component: IntrospectObject + IntrospectObjectClone + Debug + Send + Sync {
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
            ScriptValue::NativeObject(value) => {
                match value.as_any_box().downcast::<AnyComponent>() {
                    Ok(value) => Ok(*value),
                    Err(_) => Err(FruityError::new(
                        FruityStatus::InvalidArg,
                        format!("Couldn't convert An AnyComponent to native object"),
                    )),
                }
            }
            ScriptValue::Object { class_name, fields } => {
                let serialized_component = ScriptComponent::new(class_name, fields);
                Ok(AnyComponent::new(serialized_component))
            }
            _ => Err(FruityError::new(
                FruityStatus::InvalidArg,
                format!("Couldn't convert {:?} to native object", value),
            )),
        }
    }
}

impl IntrospectObject for AnyComponent {
    fn get_class_name(&self) -> String {
        self.component.get_class_name()
    }

    fn get_field_infos(&self) -> Vec<FieldInfo> {
        self.component
            .as_ref()
            .get_field_infos()
            .into_iter()
            .map(|field_info| FieldInfo {
                name: field_info.name,
                getter: Rc::new(move |this| {
                    let this = cast_introspect_ref::<AnyComponent>(this)?;
                    (field_info.getter)(this.component.as_ref().as_fruity_any_ref())
                }),
                setter: match field_info.setter {
                    SetterCaller::Const(call) => SetterCaller::Const(Rc::new(move |this, args| {
                        let this = cast_introspect_ref::<AnyComponent>(this)?;
                        call(this.component.as_ref().as_fruity_any_ref(), args)
                    })),
                    SetterCaller::Mut(call) => SetterCaller::Mut(Rc::new(move |this, args| {
                        let this = cast_introspect_mut::<AnyComponent>(this)?;
                        call(this.component.as_mut().as_fruity_any_mut(), args)
                    })),
                    SetterCaller::None => SetterCaller::None,
                },
            })
            .collect::<Vec<_>>()
    }

    fn get_method_infos(&self) -> Vec<MethodInfo> {
        self.component
            .as_ref()
            .get_method_infos()
            .into_iter()
            .map(|method_info| MethodInfo {
                name: method_info.name,
                call: match method_info.call {
                    MethodCaller::Const(call) => MethodCaller::Const(Rc::new(move |this, args| {
                        let this = cast_introspect_ref::<AnyComponent>(this)?;
                        call(this.component.as_ref().as_fruity_any_ref(), args)
                    })),
                    MethodCaller::Mut(call) => MethodCaller::Mut(Rc::new(move |this, args| {
                        let this = cast_introspect_mut::<AnyComponent>(this)?;
                        call(this.component.as_mut().as_fruity_any_mut(), args)
                    })),
                },
            })
            .collect::<Vec<_>>()
    }
}

/*impl Deserialize for AnyComponent {
    type Output = Self;

    fn deserialize(
        serialized: &ScriptValue,
        object_factory: &ObjectFactoryService,
    ) -> Option<Self> {
        let native_serialized = serialized.deserialize_native_objects(object_factory);
        if let ScriptValue::NativeObject(native_object) = native_serialized {
            native_object
                .as_any_box()
                .downcast::<AnyComponent>()
                .ok()
                .map(|component| *component)
        } else if let ScriptValue::Object { class_name, fields } = native_serialized {
            Some(AnyComponent::new(ScriptComponent::new(class_name, fields)))
        } else {
            None
        }
    }
}

impl SerializableObject for AnyComponent {
    fn duplicate(&self) -> Box<dyn SerializableObject> {
        let component = self.component.duplicate();
        Box::new(AnyComponent { component })
    }
}
*/
