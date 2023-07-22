#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use crate::components::camera::Camera;
use crate::components::rotate_2d::Rotate2D;
use crate::components::scale_2d::Scale2D;
use crate::components::sprite::Sprite;
use crate::components::transform_2d::Transform2D;
use crate::components::translate_2d::Translate2D;
use crate::graphic_2d_service::Graphic2dService;
use crate::systems::draw_camera::draw_camera;
use crate::systems::draw_sprite::draw_sprite;
use crate::systems::update_transform_2d::update_transform_2d;
use fruity_ecs::serialization::SerializationService;
use fruity_ecs::system::{SystemParams, SystemService};
use fruity_game_engine::module::Module;
use fruity_game_engine::sync::Arc;
use fruity_game_engine::{export_function, typescript_import};
pub mod components {
    pub mod camera {
        use fruity_ecs::component::Component;
        use fruity_game_engine::any::FruityAny;
        use fruity_game_engine::resource::ResourceReference;
        use fruity_game_engine::{export_constructor, export_impl, export_struct};
        use fruity_graphic::math::Color;
        use fruity_graphic::resources::texture_resource::TextureResource;
        pub struct Camera {
            pub near: f32,
            pub far: f32,
            pub target: Option<ResourceReference<dyn TextureResource>>,
            pub background_color: Color,
        }
        impl ::fruity_game_engine::introspect::IntrospectFields for Camera {
            fn is_static(&self) -> ::fruity_game_engine::FruityResult<bool> {
                Ok(true)
            }
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("Camera".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            "near".to_string(),
                            "far".to_string(),
                            "target".to_string(),
                            "background_color".to_string(),
                        ]),
                    ),
                )
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                use ::fruity_game_engine::script_value::TryFromScriptValue;
                match name {
                    "near" => self.near = <f32>::from_script_value(value)?,
                    "far" => self.far = <f32>::from_script_value(value)?,
                    "target" => {
                        self
                            .target = <Option<
                            ResourceReference<dyn TextureResource>,
                        >>::from_script_value(value)?;
                    }
                    "background_color" => {
                        self.background_color = <Color>::from_script_value(value)?;
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                };
                ::fruity_game_engine::FruityResult::Ok(())
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::TryIntoScriptValue;
                match name {
                    "near" => <f32>::into_script_value(self.near.clone()),
                    "far" => <f32>::into_script_value(self.far.clone()),
                    "target" => {
                        <Option<
                            ResourceReference<dyn TextureResource>,
                        >>::into_script_value(self.target.clone())
                    }
                    "background_color" => {
                        <Color>::into_script_value(self.background_color.clone())
                    }
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl ::fruity_game_engine::script_value::TryIntoScriptValue for Camera {
            fn into_script_value(
                self,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                Ok(
                    ::fruity_game_engine::script_value::ScriptValue::Object(
                        Box::new(self),
                    ),
                )
            }
        }
        impl ::fruity_game_engine::script_value::TryFromScriptValue for Camera {
            fn from_script_value(
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<Self> {
                use std::ops::Deref;
                match value {
                    ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                        match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Err(
                                    ::fruity_game_engine::FruityError::InvalidArg({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Couldn\'t convert a {0} to {1}", value.deref()
                                                .get_type_name(), std::any::type_name::< Camera > ()
                                            ),
                                        );
                                        res
                                    }),
                                )
                            }
                        }
                    }
                    value => {
                        Err(
                            ::fruity_game_engine::FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Couldn\'t convert {0:?} to native object", value
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Camera {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field4_finish(
                    f,
                    "Camera",
                    "near",
                    &self.near,
                    "far",
                    &self.far,
                    "target",
                    &self.target,
                    "background_color",
                    &&self.background_color,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Camera {
            #[inline]
            fn clone(&self) -> Camera {
                Camera {
                    near: ::core::clone::Clone::clone(&self.near),
                    far: ::core::clone::Clone::clone(&self.far),
                    target: ::core::clone::Clone::clone(&self.target),
                    background_color: ::core::clone::Clone::clone(&self.background_color),
                }
            }
        }
        impl ::fruity_ecs::component::Component for Camera {
            fn duplicate(&self) -> Box<dyn ::fruity_ecs::component::Component> {
                Box::new(self.clone())
            }
            fn get_component_type_id(
                &self,
            ) -> fruity_game_engine::FruityResult<
                ::fruity_ecs::component::ComponentTypeId,
            > {
                Ok(
                    ::fruity_ecs::component::ComponentTypeId::Normal(
                        fruity_game_engine::script_value::ScriptObjectType::Rust(
                            std::any::TypeId::of::<Self>(),
                        ),
                    ),
                )
            }
            fn get_storage(&self) -> Box<dyn ::fruity_ecs::component::ComponentStorage> {
                Box::new(::fruity_ecs::component::VecComponentStorage::<Self>::new())
            }
        }
        impl ::fruity_ecs::serialization::Serialize for Camera {
            fn serialize(
                &self,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
            ) -> fruity_game_engine::FruityResult<
                fruity_game_engine::settings::Settings,
            > {
                let mut result = std::collections::HashMap::<
                    String,
                    fruity_game_engine::settings::Settings,
                >::new();
                result
                    .insert(
                        "near".to_string(),
                        self.near.serialize(resource_container)?,
                    );
                result
                    .insert("far".to_string(), self.far.serialize(resource_container)?);
                result
                    .insert(
                        "target".to_string(),
                        self.target.serialize(resource_container)?,
                    );
                result
                    .insert(
                        "background_color".to_string(),
                        self.background_color.serialize(resource_container)?,
                    );
                Ok(fruity_game_engine::settings::Settings::Object(result))
            }
        }
        impl ::fruity_ecs::serialization::Deserialize for Camera {
            fn get_identifier() -> String {
                "Camera".to_string()
            }
            fn deserialize(
                serialized: &fruity_game_engine::settings::Settings,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<
                    u64,
                    ::fruity_ecs::entity::EntityId,
                >,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::settings::Settings::Object(serialized)
                    = serialized {
                    let mut result = Self::default();
                    if serialized.contains_key(&"near".to_string()) {
                        result
                            .near = <f32 as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("near").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    if serialized.contains_key(&"far".to_string()) {
                        result
                            .far = <f32 as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("far").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    if serialized.contains_key(&"target".to_string()) {
                        result
                            .target = <Option<
                            ResourceReference<dyn TextureResource>,
                        > as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("target").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    if serialized.contains_key(&"background_color".to_string()) {
                        result
                            .background_color = <Color as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("background_color").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    Ok(result)
                } else {
                    Err(
                        fruity_game_engine::FruityError::GenericFailure({
                            let res = {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize a {0} from {1:?}", "Camera", &
                                        serialized
                                    ),
                                );
                                res
                            };
                            res
                        }),
                    )
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for Camera {
            fn get_type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl Camera {
            /// Returns a new Camera
            pub fn new() -> Camera {
                Self::default()
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for Camera {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn __napi__Camera(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
            cb: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_callback_info,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value {
            use ::fruity_game_engine::napi::NapiValue;
            use ::fruity_game_engine::script_value::TryFromScriptValue;
            use ::fruity_game_engine::script_value::TryIntoScriptValue;
            unsafe {
                let env = ::fruity_game_engine::napi::Env::from_raw(raw_env);
                ::fruity_game_engine::napi::bindgen_prelude::CallbackInfo::<
                    2usize,
                >::new(raw_env, cb, None)
                    .and_then(|cb| {
                        ::fruity_game_engine::napi::bindgen_prelude::within_runtime_if_available(move ||
                        {
                            let _ret = { Camera::new() };
                            let _ret = <Camera>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = ::fruity_game_engine::javascript::script_value_to_js_value(
                                    &env,
                                    _ret,
                                )
                                .map_err(|e| e.into_napi())?;
                            <::fruity_game_engine::napi::JsUnknown as ::fruity_game_engine::napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                raw_env,
                                _ret,
                            )
                        })
                    })
                    .unwrap_or_else(|e| {
                        ::fruity_game_engine::napi::bindgen_prelude::JsError::from(e)
                            .throw_into(raw_env);
                        std::ptr::null_mut::<
                            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value__,
                        >()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(dead_code)]
        pub unsafe fn Camera_js_function(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::Result<
            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value,
        > {
            struct TransmutedTypeId {
                t: u64,
            }
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    "Camera\0".as_ptr() as *const _,
                    7usize,
                    Some(__napi__Camera),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    ::fruity_game_engine::napi::sys::Status::napi_ok => {
                        let type_id_value = std::any::TypeId::of::<Camera>();
                        let type_id_value = unsafe {
                            std::mem::transmute::<
                                std::any::TypeId,
                                TransmutedTypeId,
                            >(type_id_value)
                        }
                            .t;
                        let mut type_id_ptr = std::ptr::null_mut();
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_int64(
                            raw_env,
                            type_id_value as i64,
                            &mut type_id_ptr,
                        );
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_set_named_property(
                            raw_env,
                            fn_ptr,
                            "__type_id".as_ptr() as *const _,
                            type_id_ptr,
                        );
                        Ok(())
                    }
                    _ => {
                        Err(
                            ::fruity_game_engine::napi::Error::new(
                                ::fruity_game_engine::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to register function `{0}`", "Camera"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            ::fruity_game_engine::napi::bindgen_prelude::register_js_function(
                "Camera\0",
                Camera_js_function,
                Some(__napi__Camera),
            );
            Ok(fn_ptr)
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub extern "C" fn __napi_register__Camera() {
            ::fruity_game_engine::napi::bindgen_prelude::register_module_export(
                None,
                "Camera\0",
                Camera_js_function,
            );
        }
        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = "__DATA,__mod_init_func"]
        pub static __napi_register__Camera___rust_ctor___ctor: unsafe extern "C" fn() = {
            unsafe extern "C" fn __napi_register__Camera___rust_ctor___ctor() {
                __napi_register__Camera()
            }
            __napi_register__Camera___rust_ctor___ctor
        };
        impl Default for Camera {
            fn default() -> Self {
                Self {
                    near: -1.0,
                    far: 1.0,
                    target: None,
                    background_color: Color::default(),
                }
            }
        }
    }
    pub mod rotate_2d {
        use fruity_ecs::component::Component;
        use fruity_game_engine::{
            any::FruityAny, export_constructor, export_impl, export_struct,
        };
        pub struct Rotate2D {
            pub angle: f32,
        }
        impl ::fruity_game_engine::introspect::IntrospectFields for Rotate2D {
            fn is_static(&self) -> ::fruity_game_engine::FruityResult<bool> {
                Ok(true)
            }
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("Rotate2D".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new(["angle".to_string()]),
                    ),
                )
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                use ::fruity_game_engine::script_value::TryFromScriptValue;
                match name {
                    "angle" => self.angle = <f32>::from_script_value(value)?,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                };
                ::fruity_game_engine::FruityResult::Ok(())
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::TryIntoScriptValue;
                match name {
                    "angle" => <f32>::into_script_value(self.angle.clone()),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl ::fruity_game_engine::script_value::TryIntoScriptValue for Rotate2D {
            fn into_script_value(
                self,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                Ok(
                    ::fruity_game_engine::script_value::ScriptValue::Object(
                        Box::new(self),
                    ),
                )
            }
        }
        impl ::fruity_game_engine::script_value::TryFromScriptValue for Rotate2D {
            fn from_script_value(
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<Self> {
                use std::ops::Deref;
                match value {
                    ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                        match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Err(
                                    ::fruity_game_engine::FruityError::InvalidArg({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Couldn\'t convert a {0} to {1}", value.deref()
                                                .get_type_name(), std::any::type_name::< Rotate2D > ()
                                            ),
                                        );
                                        res
                                    }),
                                )
                            }
                        }
                    }
                    value => {
                        Err(
                            ::fruity_game_engine::FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Couldn\'t convert {0:?} to native object", value
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Rotate2D {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Rotate2D",
                    "angle",
                    &&self.angle,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Rotate2D {
            #[inline]
            fn clone(&self) -> Rotate2D {
                Rotate2D {
                    angle: ::core::clone::Clone::clone(&self.angle),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Rotate2D {
            #[inline]
            fn default() -> Rotate2D {
                Rotate2D {
                    angle: ::core::default::Default::default(),
                }
            }
        }
        impl ::fruity_ecs::component::Component for Rotate2D {
            fn duplicate(&self) -> Box<dyn ::fruity_ecs::component::Component> {
                Box::new(self.clone())
            }
            fn get_component_type_id(
                &self,
            ) -> fruity_game_engine::FruityResult<
                ::fruity_ecs::component::ComponentTypeId,
            > {
                Ok(
                    ::fruity_ecs::component::ComponentTypeId::Normal(
                        fruity_game_engine::script_value::ScriptObjectType::Rust(
                            std::any::TypeId::of::<Self>(),
                        ),
                    ),
                )
            }
            fn get_storage(&self) -> Box<dyn ::fruity_ecs::component::ComponentStorage> {
                Box::new(::fruity_ecs::component::VecComponentStorage::<Self>::new())
            }
        }
        impl ::fruity_ecs::serialization::Serialize for Rotate2D {
            fn serialize(
                &self,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
            ) -> fruity_game_engine::FruityResult<
                fruity_game_engine::settings::Settings,
            > {
                let mut result = std::collections::HashMap::<
                    String,
                    fruity_game_engine::settings::Settings,
                >::new();
                result
                    .insert(
                        "angle".to_string(),
                        self.angle.serialize(resource_container)?,
                    );
                Ok(fruity_game_engine::settings::Settings::Object(result))
            }
        }
        impl ::fruity_ecs::serialization::Deserialize for Rotate2D {
            fn get_identifier() -> String {
                "Rotate2D".to_string()
            }
            fn deserialize(
                serialized: &fruity_game_engine::settings::Settings,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<
                    u64,
                    ::fruity_ecs::entity::EntityId,
                >,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::settings::Settings::Object(serialized)
                    = serialized {
                    let mut result = Self::default();
                    if serialized.contains_key(&"angle".to_string()) {
                        result
                            .angle = <f32 as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("angle").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    Ok(result)
                } else {
                    Err(
                        fruity_game_engine::FruityError::GenericFailure({
                            let res = {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize a {0} from {1:?}", "Rotate2D", &
                                        serialized
                                    ),
                                );
                                res
                            };
                            res
                        }),
                    )
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for Rotate2D {
            fn get_type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl Rotate2D {
            /// Returns a new Rotate2D
            pub fn new(angle: f32) -> Rotate2D {
                Self { angle }
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for Rotate2D {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn __napi__Rotate2D(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
            cb: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_callback_info,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value {
            use ::fruity_game_engine::napi::NapiValue;
            use ::fruity_game_engine::script_value::TryFromScriptValue;
            use ::fruity_game_engine::script_value::TryIntoScriptValue;
            unsafe {
                let env = ::fruity_game_engine::napi::Env::from_raw(raw_env);
                ::fruity_game_engine::napi::bindgen_prelude::CallbackInfo::<
                    2usize,
                >::new(raw_env, cb, None)
                    .and_then(|cb| {
                        let arg_0 = {
                            let arg = ::fruity_game_engine::javascript::js_value_to_script_value(
                                    &env,
                                    ::fruity_game_engine::napi::JsUnknown::from_raw(
                                        raw_env,
                                        cb.get_arg(0usize),
                                    )?,
                                )
                                .map_err(|e| e.into_napi())?;
                            <f32>::from_script_value(arg).map_err(|e| e.into_napi())?
                        };
                        ::fruity_game_engine::napi::bindgen_prelude::within_runtime_if_available(move ||
                        {
                            let _ret = { Rotate2D::new(arg_0) };
                            let _ret = <Rotate2D>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = ::fruity_game_engine::javascript::script_value_to_js_value(
                                    &env,
                                    _ret,
                                )
                                .map_err(|e| e.into_napi())?;
                            <::fruity_game_engine::napi::JsUnknown as ::fruity_game_engine::napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                raw_env,
                                _ret,
                            )
                        })
                    })
                    .unwrap_or_else(|e| {
                        ::fruity_game_engine::napi::bindgen_prelude::JsError::from(e)
                            .throw_into(raw_env);
                        std::ptr::null_mut::<
                            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value__,
                        >()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(dead_code)]
        pub unsafe fn Rotate2D_js_function(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::Result<
            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value,
        > {
            struct TransmutedTypeId {
                t: u64,
            }
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    "Rotate2D\0".as_ptr() as *const _,
                    9usize,
                    Some(__napi__Rotate2D),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    ::fruity_game_engine::napi::sys::Status::napi_ok => {
                        let type_id_value = std::any::TypeId::of::<Rotate2D>();
                        let type_id_value = unsafe {
                            std::mem::transmute::<
                                std::any::TypeId,
                                TransmutedTypeId,
                            >(type_id_value)
                        }
                            .t;
                        let mut type_id_ptr = std::ptr::null_mut();
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_int64(
                            raw_env,
                            type_id_value as i64,
                            &mut type_id_ptr,
                        );
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_set_named_property(
                            raw_env,
                            fn_ptr,
                            "__type_id".as_ptr() as *const _,
                            type_id_ptr,
                        );
                        Ok(())
                    }
                    _ => {
                        Err(
                            ::fruity_game_engine::napi::Error::new(
                                ::fruity_game_engine::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to register function `{0}`", "Rotate2D"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            ::fruity_game_engine::napi::bindgen_prelude::register_js_function(
                "Rotate2D\0",
                Rotate2D_js_function,
                Some(__napi__Rotate2D),
            );
            Ok(fn_ptr)
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub extern "C" fn __napi_register__Rotate2D() {
            ::fruity_game_engine::napi::bindgen_prelude::register_module_export(
                None,
                "Rotate2D\0",
                Rotate2D_js_function,
            );
        }
        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = "__DATA,__mod_init_func"]
        pub static __napi_register__Rotate2D___rust_ctor___ctor: unsafe extern "C" fn() = {
            unsafe extern "C" fn __napi_register__Rotate2D___rust_ctor___ctor() {
                __napi_register__Rotate2D()
            }
            __napi_register__Rotate2D___rust_ctor___ctor
        };
    }
    pub mod scale_2d {
        use fruity_ecs::component::Component;
        use fruity_game_engine::{
            any::FruityAny, export_constructor, export_impl, export_struct,
        };
        use fruity_graphic::math::vector2d::Vector2D;
        pub struct Scale2D {
            pub vec: Vector2D,
        }
        impl ::fruity_game_engine::introspect::IntrospectFields for Scale2D {
            fn is_static(&self) -> ::fruity_game_engine::FruityResult<bool> {
                Ok(true)
            }
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("Scale2D".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new(["vec".to_string()]),
                    ),
                )
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                use ::fruity_game_engine::script_value::TryFromScriptValue;
                match name {
                    "vec" => self.vec = <Vector2D>::from_script_value(value)?,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                };
                ::fruity_game_engine::FruityResult::Ok(())
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::TryIntoScriptValue;
                match name {
                    "vec" => <Vector2D>::into_script_value(self.vec.clone()),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl ::fruity_game_engine::script_value::TryIntoScriptValue for Scale2D {
            fn into_script_value(
                self,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                Ok(
                    ::fruity_game_engine::script_value::ScriptValue::Object(
                        Box::new(self),
                    ),
                )
            }
        }
        impl ::fruity_game_engine::script_value::TryFromScriptValue for Scale2D {
            fn from_script_value(
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<Self> {
                use std::ops::Deref;
                match value {
                    ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                        match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Err(
                                    ::fruity_game_engine::FruityError::InvalidArg({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Couldn\'t convert a {0} to {1}", value.deref()
                                                .get_type_name(), std::any::type_name::< Scale2D > ()
                                            ),
                                        );
                                        res
                                    }),
                                )
                            }
                        }
                    }
                    value => {
                        Err(
                            ::fruity_game_engine::FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Couldn\'t convert {0:?} to native object", value
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Scale2D {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Scale2D",
                    "vec",
                    &&self.vec,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Scale2D {
            #[inline]
            fn clone(&self) -> Scale2D {
                Scale2D {
                    vec: ::core::clone::Clone::clone(&self.vec),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Scale2D {
            #[inline]
            fn default() -> Scale2D {
                Scale2D {
                    vec: ::core::default::Default::default(),
                }
            }
        }
        impl ::fruity_ecs::component::Component for Scale2D {
            fn duplicate(&self) -> Box<dyn ::fruity_ecs::component::Component> {
                Box::new(self.clone())
            }
            fn get_component_type_id(
                &self,
            ) -> fruity_game_engine::FruityResult<
                ::fruity_ecs::component::ComponentTypeId,
            > {
                Ok(
                    ::fruity_ecs::component::ComponentTypeId::Normal(
                        fruity_game_engine::script_value::ScriptObjectType::Rust(
                            std::any::TypeId::of::<Self>(),
                        ),
                    ),
                )
            }
            fn get_storage(&self) -> Box<dyn ::fruity_ecs::component::ComponentStorage> {
                Box::new(::fruity_ecs::component::VecComponentStorage::<Self>::new())
            }
        }
        impl ::fruity_ecs::serialization::Serialize for Scale2D {
            fn serialize(
                &self,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
            ) -> fruity_game_engine::FruityResult<
                fruity_game_engine::settings::Settings,
            > {
                let mut result = std::collections::HashMap::<
                    String,
                    fruity_game_engine::settings::Settings,
                >::new();
                result
                    .insert("vec".to_string(), self.vec.serialize(resource_container)?);
                Ok(fruity_game_engine::settings::Settings::Object(result))
            }
        }
        impl ::fruity_ecs::serialization::Deserialize for Scale2D {
            fn get_identifier() -> String {
                "Scale2D".to_string()
            }
            fn deserialize(
                serialized: &fruity_game_engine::settings::Settings,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<
                    u64,
                    ::fruity_ecs::entity::EntityId,
                >,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::settings::Settings::Object(serialized)
                    = serialized {
                    let mut result = Self::default();
                    if serialized.contains_key(&"vec".to_string()) {
                        result
                            .vec = <Vector2D as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("vec").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    Ok(result)
                } else {
                    Err(
                        fruity_game_engine::FruityError::GenericFailure({
                            let res = {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize a {0} from {1:?}", "Scale2D", &
                                        serialized
                                    ),
                                );
                                res
                            };
                            res
                        }),
                    )
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for Scale2D {
            fn get_type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl Scale2D {
            /// Returns a new Camera
            pub fn new(vec: Vector2D) -> Scale2D {
                Self { vec }
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for Scale2D {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn __napi__Scale2D(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
            cb: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_callback_info,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value {
            use ::fruity_game_engine::napi::NapiValue;
            use ::fruity_game_engine::script_value::TryFromScriptValue;
            use ::fruity_game_engine::script_value::TryIntoScriptValue;
            unsafe {
                let env = ::fruity_game_engine::napi::Env::from_raw(raw_env);
                ::fruity_game_engine::napi::bindgen_prelude::CallbackInfo::<
                    2usize,
                >::new(raw_env, cb, None)
                    .and_then(|cb| {
                        let arg_0 = {
                            let arg = ::fruity_game_engine::javascript::js_value_to_script_value(
                                    &env,
                                    ::fruity_game_engine::napi::JsUnknown::from_raw(
                                        raw_env,
                                        cb.get_arg(0usize),
                                    )?,
                                )
                                .map_err(|e| e.into_napi())?;
                            <Vector2D>::from_script_value(arg)
                                .map_err(|e| e.into_napi())?
                        };
                        ::fruity_game_engine::napi::bindgen_prelude::within_runtime_if_available(move ||
                        {
                            let _ret = { Scale2D::new(arg_0) };
                            let _ret = <Scale2D>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = ::fruity_game_engine::javascript::script_value_to_js_value(
                                    &env,
                                    _ret,
                                )
                                .map_err(|e| e.into_napi())?;
                            <::fruity_game_engine::napi::JsUnknown as ::fruity_game_engine::napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                raw_env,
                                _ret,
                            )
                        })
                    })
                    .unwrap_or_else(|e| {
                        ::fruity_game_engine::napi::bindgen_prelude::JsError::from(e)
                            .throw_into(raw_env);
                        std::ptr::null_mut::<
                            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value__,
                        >()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(dead_code)]
        pub unsafe fn Scale2D_js_function(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::Result<
            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value,
        > {
            struct TransmutedTypeId {
                t: u64,
            }
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    "Scale2D\0".as_ptr() as *const _,
                    8usize,
                    Some(__napi__Scale2D),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    ::fruity_game_engine::napi::sys::Status::napi_ok => {
                        let type_id_value = std::any::TypeId::of::<Scale2D>();
                        let type_id_value = unsafe {
                            std::mem::transmute::<
                                std::any::TypeId,
                                TransmutedTypeId,
                            >(type_id_value)
                        }
                            .t;
                        let mut type_id_ptr = std::ptr::null_mut();
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_int64(
                            raw_env,
                            type_id_value as i64,
                            &mut type_id_ptr,
                        );
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_set_named_property(
                            raw_env,
                            fn_ptr,
                            "__type_id".as_ptr() as *const _,
                            type_id_ptr,
                        );
                        Ok(())
                    }
                    _ => {
                        Err(
                            ::fruity_game_engine::napi::Error::new(
                                ::fruity_game_engine::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to register function `{0}`", "Scale2D"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            ::fruity_game_engine::napi::bindgen_prelude::register_js_function(
                "Scale2D\0",
                Scale2D_js_function,
                Some(__napi__Scale2D),
            );
            Ok(fn_ptr)
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub extern "C" fn __napi_register__Scale2D() {
            ::fruity_game_engine::napi::bindgen_prelude::register_module_export(
                None,
                "Scale2D\0",
                Scale2D_js_function,
            );
        }
        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = "__DATA,__mod_init_func"]
        pub static __napi_register__Scale2D___rust_ctor___ctor: unsafe extern "C" fn() = {
            unsafe extern "C" fn __napi_register__Scale2D___rust_ctor___ctor() {
                __napi_register__Scale2D()
            }
            __napi_register__Scale2D___rust_ctor___ctor
        };
    }
    pub mod sprite {
        use fruity_ecs::component::Component;
        use fruity_game_engine::any::FruityAny;
        use fruity_game_engine::resource::ResourceReference;
        use fruity_game_engine::{export_constructor, export_impl, export_struct};
        use fruity_graphic::resources::material_resource::MaterialResource;
        use fruity_graphic::resources::texture_resource::TextureResource;
        pub struct Sprite {
            pub material: Option<ResourceReference<dyn MaterialResource>>,
            pub texture: Option<ResourceReference<dyn TextureResource>>,
            pub z_index: i32,
        }
        impl ::fruity_game_engine::introspect::IntrospectFields for Sprite {
            fn is_static(&self) -> ::fruity_game_engine::FruityResult<bool> {
                Ok(true)
            }
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("Sprite".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new([
                            "material".to_string(),
                            "texture".to_string(),
                            "z_index".to_string(),
                        ]),
                    ),
                )
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                use ::fruity_game_engine::script_value::TryFromScriptValue;
                match name {
                    "material" => {
                        self
                            .material = <Option<
                            ResourceReference<dyn MaterialResource>,
                        >>::from_script_value(value)?;
                    }
                    "texture" => {
                        self
                            .texture = <Option<
                            ResourceReference<dyn TextureResource>,
                        >>::from_script_value(value)?;
                    }
                    "z_index" => self.z_index = <i32>::from_script_value(value)?,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                };
                ::fruity_game_engine::FruityResult::Ok(())
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::TryIntoScriptValue;
                match name {
                    "material" => {
                        <Option<
                            ResourceReference<dyn MaterialResource>,
                        >>::into_script_value(self.material.clone())
                    }
                    "texture" => {
                        <Option<
                            ResourceReference<dyn TextureResource>,
                        >>::into_script_value(self.texture.clone())
                    }
                    "z_index" => <i32>::into_script_value(self.z_index.clone()),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl ::fruity_game_engine::script_value::TryIntoScriptValue for Sprite {
            fn into_script_value(
                self,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                Ok(
                    ::fruity_game_engine::script_value::ScriptValue::Object(
                        Box::new(self),
                    ),
                )
            }
        }
        impl ::fruity_game_engine::script_value::TryFromScriptValue for Sprite {
            fn from_script_value(
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<Self> {
                use std::ops::Deref;
                match value {
                    ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                        match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Err(
                                    ::fruity_game_engine::FruityError::InvalidArg({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Couldn\'t convert a {0} to {1}", value.deref()
                                                .get_type_name(), std::any::type_name::< Sprite > ()
                                            ),
                                        );
                                        res
                                    }),
                                )
                            }
                        }
                    }
                    value => {
                        Err(
                            ::fruity_game_engine::FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Couldn\'t convert {0:?} to native object", value
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Sprite {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field3_finish(
                    f,
                    "Sprite",
                    "material",
                    &self.material,
                    "texture",
                    &self.texture,
                    "z_index",
                    &&self.z_index,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Sprite {
            #[inline]
            fn clone(&self) -> Sprite {
                Sprite {
                    material: ::core::clone::Clone::clone(&self.material),
                    texture: ::core::clone::Clone::clone(&self.texture),
                    z_index: ::core::clone::Clone::clone(&self.z_index),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Sprite {
            #[inline]
            fn default() -> Sprite {
                Sprite {
                    material: ::core::default::Default::default(),
                    texture: ::core::default::Default::default(),
                    z_index: ::core::default::Default::default(),
                }
            }
        }
        impl ::fruity_ecs::component::Component for Sprite {
            fn duplicate(&self) -> Box<dyn ::fruity_ecs::component::Component> {
                Box::new(self.clone())
            }
            fn get_component_type_id(
                &self,
            ) -> fruity_game_engine::FruityResult<
                ::fruity_ecs::component::ComponentTypeId,
            > {
                Ok(
                    ::fruity_ecs::component::ComponentTypeId::Normal(
                        fruity_game_engine::script_value::ScriptObjectType::Rust(
                            std::any::TypeId::of::<Self>(),
                        ),
                    ),
                )
            }
            fn get_storage(&self) -> Box<dyn ::fruity_ecs::component::ComponentStorage> {
                Box::new(::fruity_ecs::component::VecComponentStorage::<Self>::new())
            }
        }
        impl ::fruity_ecs::serialization::Serialize for Sprite {
            fn serialize(
                &self,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
            ) -> fruity_game_engine::FruityResult<
                fruity_game_engine::settings::Settings,
            > {
                let mut result = std::collections::HashMap::<
                    String,
                    fruity_game_engine::settings::Settings,
                >::new();
                result
                    .insert(
                        "material".to_string(),
                        self.material.serialize(resource_container)?,
                    );
                result
                    .insert(
                        "texture".to_string(),
                        self.texture.serialize(resource_container)?,
                    );
                result
                    .insert(
                        "z_index".to_string(),
                        self.z_index.serialize(resource_container)?,
                    );
                Ok(fruity_game_engine::settings::Settings::Object(result))
            }
        }
        impl ::fruity_ecs::serialization::Deserialize for Sprite {
            fn get_identifier() -> String {
                "Sprite".to_string()
            }
            fn deserialize(
                serialized: &fruity_game_engine::settings::Settings,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<
                    u64,
                    ::fruity_ecs::entity::EntityId,
                >,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::settings::Settings::Object(serialized)
                    = serialized {
                    let mut result = Self::default();
                    if serialized.contains_key(&"material".to_string()) {
                        result
                            .material = <Option<
                            ResourceReference<dyn MaterialResource>,
                        > as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("material").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    if serialized.contains_key(&"texture".to_string()) {
                        result
                            .texture = <Option<
                            ResourceReference<dyn TextureResource>,
                        > as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("texture").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    if serialized.contains_key(&"z_index".to_string()) {
                        result
                            .z_index = <i32 as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("z_index").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    Ok(result)
                } else {
                    Err(
                        fruity_game_engine::FruityError::GenericFailure({
                            let res = {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize a {0} from {1:?}", "Sprite", &
                                        serialized
                                    ),
                                );
                                res
                            };
                            res
                        }),
                    )
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for Sprite {
            fn get_type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl Sprite {
            /// Returns a new Camera
            pub fn new(
                material: Option<ResourceReference<dyn MaterialResource>>,
                texture: Option<ResourceReference<dyn TextureResource>>,
                z_index: i32,
            ) -> Sprite {
                Self { material, texture, z_index }
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for Sprite {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn __napi__Sprite(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
            cb: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_callback_info,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value {
            use ::fruity_game_engine::napi::NapiValue;
            use ::fruity_game_engine::script_value::TryFromScriptValue;
            use ::fruity_game_engine::script_value::TryIntoScriptValue;
            unsafe {
                let env = ::fruity_game_engine::napi::Env::from_raw(raw_env);
                ::fruity_game_engine::napi::bindgen_prelude::CallbackInfo::<
                    2usize,
                >::new(raw_env, cb, None)
                    .and_then(|cb| {
                        let arg_0 = {
                            let arg = ::fruity_game_engine::javascript::js_value_to_script_value(
                                    &env,
                                    ::fruity_game_engine::napi::JsUnknown::from_raw(
                                        raw_env,
                                        cb.get_arg(0usize),
                                    )?,
                                )
                                .map_err(|e| e.into_napi())?;
                            <Option<
                                ResourceReference<dyn MaterialResource>,
                            >>::from_script_value(arg)
                                .map_err(|e| e.into_napi())?
                        };
                        let arg_1 = {
                            let arg = ::fruity_game_engine::javascript::js_value_to_script_value(
                                    &env,
                                    ::fruity_game_engine::napi::JsUnknown::from_raw(
                                        raw_env,
                                        cb.get_arg(1usize),
                                    )?,
                                )
                                .map_err(|e| e.into_napi())?;
                            <Option<
                                ResourceReference<dyn TextureResource>,
                            >>::from_script_value(arg)
                                .map_err(|e| e.into_napi())?
                        };
                        let arg_2 = {
                            let arg = ::fruity_game_engine::javascript::js_value_to_script_value(
                                    &env,
                                    ::fruity_game_engine::napi::JsUnknown::from_raw(
                                        raw_env,
                                        cb.get_arg(2usize),
                                    )?,
                                )
                                .map_err(|e| e.into_napi())?;
                            <i32>::from_script_value(arg).map_err(|e| e.into_napi())?
                        };
                        ::fruity_game_engine::napi::bindgen_prelude::within_runtime_if_available(move ||
                        {
                            let _ret = { Sprite::new(arg_0, arg_1, arg_2) };
                            let _ret = <Sprite>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = ::fruity_game_engine::javascript::script_value_to_js_value(
                                    &env,
                                    _ret,
                                )
                                .map_err(|e| e.into_napi())?;
                            <::fruity_game_engine::napi::JsUnknown as ::fruity_game_engine::napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                raw_env,
                                _ret,
                            )
                        })
                    })
                    .unwrap_or_else(|e| {
                        ::fruity_game_engine::napi::bindgen_prelude::JsError::from(e)
                            .throw_into(raw_env);
                        std::ptr::null_mut::<
                            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value__,
                        >()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(dead_code)]
        pub unsafe fn Sprite_js_function(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::Result<
            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value,
        > {
            struct TransmutedTypeId {
                t: u64,
            }
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    "Sprite\0".as_ptr() as *const _,
                    7usize,
                    Some(__napi__Sprite),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    ::fruity_game_engine::napi::sys::Status::napi_ok => {
                        let type_id_value = std::any::TypeId::of::<Sprite>();
                        let type_id_value = unsafe {
                            std::mem::transmute::<
                                std::any::TypeId,
                                TransmutedTypeId,
                            >(type_id_value)
                        }
                            .t;
                        let mut type_id_ptr = std::ptr::null_mut();
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_int64(
                            raw_env,
                            type_id_value as i64,
                            &mut type_id_ptr,
                        );
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_set_named_property(
                            raw_env,
                            fn_ptr,
                            "__type_id".as_ptr() as *const _,
                            type_id_ptr,
                        );
                        Ok(())
                    }
                    _ => {
                        Err(
                            ::fruity_game_engine::napi::Error::new(
                                ::fruity_game_engine::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!("Failed to register function `{0}`", "Sprite"),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            ::fruity_game_engine::napi::bindgen_prelude::register_js_function(
                "Sprite\0",
                Sprite_js_function,
                Some(__napi__Sprite),
            );
            Ok(fn_ptr)
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub extern "C" fn __napi_register__Sprite() {
            ::fruity_game_engine::napi::bindgen_prelude::register_module_export(
                None,
                "Sprite\0",
                Sprite_js_function,
            );
        }
        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = "__DATA,__mod_init_func"]
        pub static __napi_register__Sprite___rust_ctor___ctor: unsafe extern "C" fn() = {
            unsafe extern "C" fn __napi_register__Sprite___rust_ctor___ctor() {
                __napi_register__Sprite()
            }
            __napi_register__Sprite___rust_ctor___ctor
        };
    }
    pub mod transform_2d {
        use fruity_ecs::component::Component;
        use fruity_game_engine::{
            any::FruityAny, export_constructor, export_impl, export_struct,
        };
        use fruity_graphic::math::matrix3::Matrix3;
        pub struct Transform2D {
            pub transform: Matrix3,
        }
        impl ::fruity_game_engine::introspect::IntrospectFields for Transform2D {
            fn is_static(&self) -> ::fruity_game_engine::FruityResult<bool> {
                Ok(true)
            }
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("Transform2D".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new(["transform".to_string()]),
                    ),
                )
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                use ::fruity_game_engine::script_value::TryFromScriptValue;
                match name {
                    "transform" => self.transform = <Matrix3>::from_script_value(value)?,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                };
                ::fruity_game_engine::FruityResult::Ok(())
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::TryIntoScriptValue;
                match name {
                    "transform" => <Matrix3>::into_script_value(self.transform.clone()),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl ::fruity_game_engine::script_value::TryIntoScriptValue for Transform2D {
            fn into_script_value(
                self,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                Ok(
                    ::fruity_game_engine::script_value::ScriptValue::Object(
                        Box::new(self),
                    ),
                )
            }
        }
        impl ::fruity_game_engine::script_value::TryFromScriptValue for Transform2D {
            fn from_script_value(
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<Self> {
                use std::ops::Deref;
                match value {
                    ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                        match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Err(
                                    ::fruity_game_engine::FruityError::InvalidArg({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Couldn\'t convert a {0} to {1}", value.deref()
                                                .get_type_name(), std::any::type_name::< Transform2D > ()
                                            ),
                                        );
                                        res
                                    }),
                                )
                            }
                        }
                    }
                    value => {
                        Err(
                            ::fruity_game_engine::FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Couldn\'t convert {0:?} to native object", value
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Transform2D {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Transform2D",
                    "transform",
                    &&self.transform,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Transform2D {
            #[inline]
            fn clone(&self) -> Transform2D {
                Transform2D {
                    transform: ::core::clone::Clone::clone(&self.transform),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Transform2D {
            #[inline]
            fn default() -> Transform2D {
                Transform2D {
                    transform: ::core::default::Default::default(),
                }
            }
        }
        impl ::fruity_ecs::component::Component for Transform2D {
            fn duplicate(&self) -> Box<dyn ::fruity_ecs::component::Component> {
                Box::new(self.clone())
            }
            fn get_component_type_id(
                &self,
            ) -> fruity_game_engine::FruityResult<
                ::fruity_ecs::component::ComponentTypeId,
            > {
                Ok(
                    ::fruity_ecs::component::ComponentTypeId::Normal(
                        fruity_game_engine::script_value::ScriptObjectType::Rust(
                            std::any::TypeId::of::<Self>(),
                        ),
                    ),
                )
            }
            fn get_storage(&self) -> Box<dyn ::fruity_ecs::component::ComponentStorage> {
                Box::new(::fruity_ecs::component::VecComponentStorage::<Self>::new())
            }
        }
        impl ::fruity_ecs::serialization::Serialize for Transform2D {
            fn serialize(
                &self,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
            ) -> fruity_game_engine::FruityResult<
                fruity_game_engine::settings::Settings,
            > {
                let mut result = std::collections::HashMap::<
                    String,
                    fruity_game_engine::settings::Settings,
                >::new();
                result
                    .insert(
                        "transform".to_string(),
                        self.transform.serialize(resource_container)?,
                    );
                Ok(fruity_game_engine::settings::Settings::Object(result))
            }
        }
        impl ::fruity_ecs::serialization::Deserialize for Transform2D {
            fn get_identifier() -> String {
                "Transform2D".to_string()
            }
            fn deserialize(
                serialized: &fruity_game_engine::settings::Settings,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<
                    u64,
                    ::fruity_ecs::entity::EntityId,
                >,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::settings::Settings::Object(serialized)
                    = serialized {
                    let mut result = Self::default();
                    if serialized.contains_key(&"transform".to_string()) {
                        result
                            .transform = <Matrix3 as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("transform").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    Ok(result)
                } else {
                    Err(
                        fruity_game_engine::FruityError::GenericFailure({
                            let res = {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize a {0} from {1:?}", "Transform2D", &
                                        serialized
                                    ),
                                );
                                res
                            };
                            res
                        }),
                    )
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for Transform2D {
            fn get_type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl Transform2D {
            /// Returns a new Camera
            pub fn new() -> Transform2D {
                Self {
                    transform: Default::default(),
                }
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for Transform2D {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn __napi__Transform2D(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
            cb: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_callback_info,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value {
            use ::fruity_game_engine::napi::NapiValue;
            use ::fruity_game_engine::script_value::TryFromScriptValue;
            use ::fruity_game_engine::script_value::TryIntoScriptValue;
            unsafe {
                let env = ::fruity_game_engine::napi::Env::from_raw(raw_env);
                ::fruity_game_engine::napi::bindgen_prelude::CallbackInfo::<
                    2usize,
                >::new(raw_env, cb, None)
                    .and_then(|cb| {
                        ::fruity_game_engine::napi::bindgen_prelude::within_runtime_if_available(move ||
                        {
                            let _ret = { Transform2D::new() };
                            let _ret = <Transform2D>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = ::fruity_game_engine::javascript::script_value_to_js_value(
                                    &env,
                                    _ret,
                                )
                                .map_err(|e| e.into_napi())?;
                            <::fruity_game_engine::napi::JsUnknown as ::fruity_game_engine::napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                raw_env,
                                _ret,
                            )
                        })
                    })
                    .unwrap_or_else(|e| {
                        ::fruity_game_engine::napi::bindgen_prelude::JsError::from(e)
                            .throw_into(raw_env);
                        std::ptr::null_mut::<
                            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value__,
                        >()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(dead_code)]
        pub unsafe fn Transform2D_js_function(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::Result<
            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value,
        > {
            struct TransmutedTypeId {
                t: u64,
            }
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    "Transform2D\0".as_ptr() as *const _,
                    12usize,
                    Some(__napi__Transform2D),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    ::fruity_game_engine::napi::sys::Status::napi_ok => {
                        let type_id_value = std::any::TypeId::of::<Transform2D>();
                        let type_id_value = unsafe {
                            std::mem::transmute::<
                                std::any::TypeId,
                                TransmutedTypeId,
                            >(type_id_value)
                        }
                            .t;
                        let mut type_id_ptr = std::ptr::null_mut();
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_int64(
                            raw_env,
                            type_id_value as i64,
                            &mut type_id_ptr,
                        );
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_set_named_property(
                            raw_env,
                            fn_ptr,
                            "__type_id".as_ptr() as *const _,
                            type_id_ptr,
                        );
                        Ok(())
                    }
                    _ => {
                        Err(
                            ::fruity_game_engine::napi::Error::new(
                                ::fruity_game_engine::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to register function `{0}`", "Transform2D"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            ::fruity_game_engine::napi::bindgen_prelude::register_js_function(
                "Transform2D\0",
                Transform2D_js_function,
                Some(__napi__Transform2D),
            );
            Ok(fn_ptr)
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub extern "C" fn __napi_register__Transform2D() {
            ::fruity_game_engine::napi::bindgen_prelude::register_module_export(
                None,
                "Transform2D\0",
                Transform2D_js_function,
            );
        }
        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = "__DATA,__mod_init_func"]
        pub static __napi_register__Transform2D___rust_ctor___ctor: unsafe extern "C" fn() = {
            unsafe extern "C" fn __napi_register__Transform2D___rust_ctor___ctor() {
                __napi_register__Transform2D()
            }
            __napi_register__Transform2D___rust_ctor___ctor
        };
    }
    pub mod translate_2d {
        use fruity_ecs::component::Component;
        use fruity_game_engine::{
            any::FruityAny, export_constructor, export_impl, export_struct,
        };
        use fruity_graphic::math::vector2d::Vector2D;
        pub struct Translate2D {
            pub vec: Vector2D,
        }
        impl ::fruity_game_engine::introspect::IntrospectFields for Translate2D {
            fn is_static(&self) -> ::fruity_game_engine::FruityResult<bool> {
                Ok(true)
            }
            fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
                Ok("Translate2D".to_string())
            }
            fn get_field_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(
                    <[_]>::into_vec(
                        #[rustc_box]
                        ::alloc::boxed::Box::new(["vec".to_string()]),
                    ),
                )
            }
            fn set_field_value(
                &mut self,
                name: &str,
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<()> {
                use ::fruity_game_engine::script_value::TryFromScriptValue;
                match name {
                    "vec" => self.vec = <Vector2D>::from_script_value(value)?,
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                };
                ::fruity_game_engine::FruityResult::Ok(())
            }
            fn get_field_value(
                &self,
                name: &str,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                use ::fruity_game_engine::script_value::TryIntoScriptValue;
                match name {
                    "vec" => <Vector2D>::into_script_value(self.vec.clone()),
                    _ => {
                        ::core::panicking::panic(
                            "internal error: entered unreachable code",
                        )
                    }
                }
            }
        }
        impl ::fruity_game_engine::script_value::TryIntoScriptValue for Translate2D {
            fn into_script_value(
                self,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                Ok(
                    ::fruity_game_engine::script_value::ScriptValue::Object(
                        Box::new(self),
                    ),
                )
            }
        }
        impl ::fruity_game_engine::script_value::TryFromScriptValue for Translate2D {
            fn from_script_value(
                value: ::fruity_game_engine::script_value::ScriptValue,
            ) -> ::fruity_game_engine::FruityResult<Self> {
                use std::ops::Deref;
                match value {
                    ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                        match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Err(
                                    ::fruity_game_engine::FruityError::InvalidArg({
                                        let res = ::alloc::fmt::format(
                                            format_args!(
                                                "Couldn\'t convert a {0} to {1}", value.deref()
                                                .get_type_name(), std::any::type_name::< Translate2D > ()
                                            ),
                                        );
                                        res
                                    }),
                                )
                            }
                        }
                    }
                    value => {
                        Err(
                            ::fruity_game_engine::FruityError::InvalidArg({
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Couldn\'t convert {0:?} to native object", value
                                    ),
                                );
                                res
                            }),
                        )
                    }
                }
            }
        }
        #[automatically_derived]
        impl ::core::fmt::Debug for Translate2D {
            fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                ::core::fmt::Formatter::debug_struct_field1_finish(
                    f,
                    "Translate2D",
                    "vec",
                    &&self.vec,
                )
            }
        }
        #[automatically_derived]
        impl ::core::clone::Clone for Translate2D {
            #[inline]
            fn clone(&self) -> Translate2D {
                Translate2D {
                    vec: ::core::clone::Clone::clone(&self.vec),
                }
            }
        }
        #[automatically_derived]
        impl ::core::default::Default for Translate2D {
            #[inline]
            fn default() -> Translate2D {
                Translate2D {
                    vec: ::core::default::Default::default(),
                }
            }
        }
        impl ::fruity_ecs::component::Component for Translate2D {
            fn duplicate(&self) -> Box<dyn ::fruity_ecs::component::Component> {
                Box::new(self.clone())
            }
            fn get_component_type_id(
                &self,
            ) -> fruity_game_engine::FruityResult<
                ::fruity_ecs::component::ComponentTypeId,
            > {
                Ok(
                    ::fruity_ecs::component::ComponentTypeId::Normal(
                        fruity_game_engine::script_value::ScriptObjectType::Rust(
                            std::any::TypeId::of::<Self>(),
                        ),
                    ),
                )
            }
            fn get_storage(&self) -> Box<dyn ::fruity_ecs::component::ComponentStorage> {
                Box::new(::fruity_ecs::component::VecComponentStorage::<Self>::new())
            }
        }
        impl ::fruity_ecs::serialization::Serialize for Translate2D {
            fn serialize(
                &self,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
            ) -> fruity_game_engine::FruityResult<
                fruity_game_engine::settings::Settings,
            > {
                let mut result = std::collections::HashMap::<
                    String,
                    fruity_game_engine::settings::Settings,
                >::new();
                result
                    .insert("vec".to_string(), self.vec.serialize(resource_container)?);
                Ok(fruity_game_engine::settings::Settings::Object(result))
            }
        }
        impl ::fruity_ecs::serialization::Deserialize for Translate2D {
            fn get_identifier() -> String {
                "Translate2D".to_string()
            }
            fn deserialize(
                serialized: &fruity_game_engine::settings::Settings,
                resource_container: &fruity_game_engine::resource::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<
                    u64,
                    ::fruity_ecs::entity::EntityId,
                >,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::settings::Settings::Object(serialized)
                    = serialized {
                    let mut result = Self::default();
                    if serialized.contains_key(&"vec".to_string()) {
                        result
                            .vec = <Vector2D as ::fruity_ecs::serialization::Deserialize>::deserialize(
                            serialized.get("vec").unwrap(),
                            resource_container,
                            local_id_to_entity_id,
                        )?;
                    }
                    Ok(result)
                } else {
                    Err(
                        fruity_game_engine::FruityError::GenericFailure({
                            let res = {
                                let res = ::alloc::fmt::format(
                                    format_args!(
                                        "Failed to deserialize a {0} from {1:?}", "Translate2D", &
                                        serialized
                                    ),
                                );
                                res
                            };
                            res
                        }),
                    )
                }
            }
        }
        impl ::fruity_game_engine::any::FruityAny for Translate2D {
            fn get_type_name(&self) -> &'static str {
                std::any::type_name::<Self>()
            }
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }
            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }
            fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_mut(
                &mut self,
            ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
                self
            }
            fn as_fruity_any_box(
                self: Box<Self>,
            ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
                self
            }
        }
        impl Translate2D {
            /// Returns a new Camera
            pub fn new(vec: Vector2D) -> Translate2D {
                Self { vec }
            }
        }
        impl ::fruity_game_engine::introspect::IntrospectMethods for Translate2D {
            fn get_const_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_const_method(
                &self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
            fn get_mut_method_names(
                &self,
            ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
                Ok(::alloc::vec::Vec::new())
            }
            fn call_mut_method(
                &mut self,
                name: &str,
                __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
            ) -> ::fruity_game_engine::FruityResult<
                ::fruity_game_engine::script_value::ScriptValue,
            > {
                ::core::panicking::panic("internal error: entered unreachable code")
            }
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn __napi__Translate2D(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
            cb: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_callback_info,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value {
            use ::fruity_game_engine::napi::NapiValue;
            use ::fruity_game_engine::script_value::TryFromScriptValue;
            use ::fruity_game_engine::script_value::TryIntoScriptValue;
            unsafe {
                let env = ::fruity_game_engine::napi::Env::from_raw(raw_env);
                ::fruity_game_engine::napi::bindgen_prelude::CallbackInfo::<
                    2usize,
                >::new(raw_env, cb, None)
                    .and_then(|cb| {
                        let arg_0 = {
                            let arg = ::fruity_game_engine::javascript::js_value_to_script_value(
                                    &env,
                                    ::fruity_game_engine::napi::JsUnknown::from_raw(
                                        raw_env,
                                        cb.get_arg(0usize),
                                    )?,
                                )
                                .map_err(|e| e.into_napi())?;
                            <Vector2D>::from_script_value(arg)
                                .map_err(|e| e.into_napi())?
                        };
                        ::fruity_game_engine::napi::bindgen_prelude::within_runtime_if_available(move ||
                        {
                            let _ret = { Translate2D::new(arg_0) };
                            let _ret = <Translate2D>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = ::fruity_game_engine::javascript::script_value_to_js_value(
                                    &env,
                                    _ret,
                                )
                                .map_err(|e| e.into_napi())?;
                            <::fruity_game_engine::napi::JsUnknown as ::fruity_game_engine::napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                                raw_env,
                                _ret,
                            )
                        })
                    })
                    .unwrap_or_else(|e| {
                        ::fruity_game_engine::napi::bindgen_prelude::JsError::from(e)
                            .throw_into(raw_env);
                        std::ptr::null_mut::<
                            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value__,
                        >()
                    })
            }
        }
        #[doc(hidden)]
        #[allow(dead_code)]
        pub unsafe fn Translate2D_js_function(
            raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
        ) -> ::fruity_game_engine::napi::bindgen_prelude::Result<
            ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value,
        > {
            struct TransmutedTypeId {
                t: u64,
            }
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    "Translate2D\0".as_ptr() as *const _,
                    12usize,
                    Some(__napi__Translate2D),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    ::fruity_game_engine::napi::sys::Status::napi_ok => {
                        let type_id_value = std::any::TypeId::of::<Translate2D>();
                        let type_id_value = unsafe {
                            std::mem::transmute::<
                                std::any::TypeId,
                                TransmutedTypeId,
                            >(type_id_value)
                        }
                            .t;
                        let mut type_id_ptr = std::ptr::null_mut();
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_int64(
                            raw_env,
                            type_id_value as i64,
                            &mut type_id_ptr,
                        );
                        ::fruity_game_engine::napi::bindgen_prelude::sys::napi_set_named_property(
                            raw_env,
                            fn_ptr,
                            "__type_id".as_ptr() as *const _,
                            type_id_ptr,
                        );
                        Ok(())
                    }
                    _ => {
                        Err(
                            ::fruity_game_engine::napi::Error::new(
                                ::fruity_game_engine::napi::Status::from(c),
                                {
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Failed to register function `{0}`", "Translate2D"
                                        ),
                                    );
                                    res
                                },
                            ),
                        )
                    }
                }
            }?;
            ::fruity_game_engine::napi::bindgen_prelude::register_js_function(
                "Translate2D\0",
                Translate2D_js_function,
                Some(__napi__Translate2D),
            );
            Ok(fn_ptr)
        }
        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub extern "C" fn __napi_register__Translate2D() {
            ::fruity_game_engine::napi::bindgen_prelude::register_module_export(
                None,
                "Translate2D\0",
                Translate2D_js_function,
            );
        }
        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = "__DATA,__mod_init_func"]
        pub static __napi_register__Translate2D___rust_ctor___ctor: unsafe extern "C" fn() = {
            unsafe extern "C" fn __napi_register__Translate2D___rust_ctor___ctor() {
                __napi_register__Translate2D()
            }
            __napi_register__Translate2D___rust_ctor___ctor
        };
    }
}
pub mod graphic_2d_service {
    use adjacent_pair_iterator::AdjacentPairIterator;
    use fruity_game_engine::any::FruityAny;
    use fruity_game_engine::export;
    use fruity_game_engine::export_impl;
    use fruity_game_engine::export_struct;
    use fruity_game_engine::resource::ResourceContainer;
    use fruity_game_engine::resource::ResourceReference;
    use fruity_game_engine::FruityError;
    use fruity_game_engine::FruityResult;
    use fruity_graphic::graphic_service::GraphicService;
    use fruity_graphic::graphic_service::MaterialParam;
    use fruity_graphic::math::matrix3::Matrix3;
    use fruity_graphic::math::vector2d::Vector2D;
    use fruity_graphic::math::Color;
    use fruity_graphic::resources::material_resource::MaterialResource;
    use fruity_graphic::resources::mesh_resource::MeshResource;
    use maplit::hashmap;
    use std::collections::HashMap;
    use std::f32::consts::PI;
    use std::ops::Range;
    pub struct Graphic2dService {
        graphic_service: ResourceReference<dyn GraphicService>,
        resource_container: ResourceContainer,
        draw_line_material: ResourceReference<dyn MaterialResource>,
        draw_dotted_line_material: ResourceReference<dyn MaterialResource>,
        draw_rect_material: ResourceReference<dyn MaterialResource>,
        draw_arc_material: ResourceReference<dyn MaterialResource>,
    }
    impl ::fruity_game_engine::introspect::IntrospectFields for Graphic2dService {
        fn is_static(&self) -> ::fruity_game_engine::FruityResult<bool> {
            Ok(true)
        }
        fn get_class_name(&self) -> ::fruity_game_engine::FruityResult<String> {
            Ok("Graphic2dService".to_string())
        }
        fn get_field_names(&self) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn set_field_value(
            &mut self,
            name: &str,
            value: ::fruity_game_engine::script_value::ScriptValue,
        ) -> ::fruity_game_engine::FruityResult<()> {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
        fn get_field_value(
            &self,
            name: &str,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    impl ::fruity_game_engine::script_value::TryIntoScriptValue for Graphic2dService {
        fn into_script_value(
            self,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            Ok(::fruity_game_engine::script_value::ScriptValue::Object(Box::new(self)))
        }
    }
    impl ::fruity_game_engine::script_value::TryFromScriptValue for Graphic2dService {
        fn from_script_value(
            value: ::fruity_game_engine::script_value::ScriptValue,
        ) -> ::fruity_game_engine::FruityResult<Self> {
            use std::ops::Deref;
            match value {
                ::fruity_game_engine::script_value::ScriptValue::Object(value) => {
                    match value.downcast::<Self>() {
                        Ok(value) => Ok(*value),
                        Err(value) => {
                            Err(
                                ::fruity_game_engine::FruityError::InvalidArg({
                                    let res = ::alloc::fmt::format(
                                        format_args!(
                                            "Couldn\'t convert a {0} to {1}", value.deref()
                                            .get_type_name(), std::any::type_name::< Graphic2dService >
                                            ()
                                        ),
                                    );
                                    res
                                }),
                            )
                        }
                    }
                }
                value => {
                    Err(
                        ::fruity_game_engine::FruityError::InvalidArg({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Couldn\'t convert {0:?} to native object", value
                                ),
                            );
                            res
                        }),
                    )
                }
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for Graphic2dService {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "graphic_service",
                "resource_container",
                "draw_line_material",
                "draw_dotted_line_material",
                "draw_rect_material",
                "draw_arc_material",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.graphic_service,
                &self.resource_container,
                &self.draw_line_material,
                &self.draw_dotted_line_material,
                &self.draw_rect_material,
                &&self.draw_arc_material,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "Graphic2dService",
                names,
                values,
            )
        }
    }
    impl ::fruity_game_engine::any::FruityAny for Graphic2dService {
        fn get_type_name(&self) -> &'static str {
            std::any::type_name::<Self>()
        }
        fn as_any_ref(&self) -> &dyn std::any::Any {
            self
        }
        fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
            self
        }
        fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
            self
        }
        fn as_fruity_any_ref(&self) -> &dyn ::fruity_game_engine::any::FruityAny {
            self
        }
        fn as_fruity_any_mut(
            &mut self,
        ) -> &mut dyn ::fruity_game_engine::any::FruityAny {
            self
        }
        fn as_fruity_any_box(
            self: Box<Self>,
        ) -> Box<dyn ::fruity_game_engine::any::FruityAny> {
            self
        }
    }
    impl Graphic2dService {
        pub fn new(resource_container: ResourceContainer) -> FruityResult<Self> {
            let graphic_service = resource_container.require::<dyn GraphicService>();
            let draw_line_material = resource_container
                .get::<dyn MaterialResource>("Materials/Draw Line")
                .ok_or(
                    FruityError::GenericFailure({
                        let res = ::alloc::fmt::format(
                            format_args!("Missing shader {0}", "Materials/Draw Line"),
                        );
                        res
                    }),
                )?;
            let draw_dotted_line_material = resource_container
                .get::<dyn MaterialResource>("Materials/Draw Dotted Line")
                .ok_or(
                    FruityError::GenericFailure({
                        let res = ::alloc::fmt::format(
                            format_args!(
                                "Missing shader {0}", "Materials/Draw Dotted Line"
                            ),
                        );
                        res
                    }),
                )?;
            let draw_rect_material = resource_container
                .get::<dyn MaterialResource>("Materials/Draw Rect")
                .ok_or(
                    FruityError::GenericFailure({
                        let res = ::alloc::fmt::format(
                            format_args!("Missing shader {0}", "Materials/Draw Rect"),
                        );
                        res
                    }),
                )?;
            let draw_arc_material = resource_container
                .get::<dyn MaterialResource>("Materials/Draw Arc")
                .ok_or(
                    FruityError::GenericFailure({
                        let res = ::alloc::fmt::format(
                            format_args!("Missing shader {0}", "Materials/Draw Arc"),
                        );
                        res
                    }),
                )?;
            Ok(Self {
                graphic_service,
                resource_container,
                draw_line_material,
                draw_dotted_line_material,
                draw_rect_material,
                draw_arc_material,
            })
        }
        pub fn draw_quad(
            &self,
            identifier: u64,
            material: ResourceReference<dyn MaterialResource>,
            params: HashMap<String, MaterialParam>,
            z_index: i32,
        ) {
            let graphic_service = self.graphic_service.read();
            let mesh = self
                .resource_container
                .get::<dyn MeshResource>("Meshes/Quad")
                .unwrap();
            graphic_service
                .draw_mesh(identifier, mesh.clone(), material, params, z_index)
        }
        pub fn draw_line(
            &self,
            pos1: Vector2D,
            pos2: Vector2D,
            width: u32,
            color: Color,
            z_index: i32,
            transform: Matrix3,
        ) {
            self.draw_quad(
                0,
                self.draw_line_material.clone(),
                {
                    let _cap = <[()]>::len(&[(), (), (), (), ()]);
                    let mut _map = ::std::collections::HashMap::with_capacity(_cap);
                    let _ = _map
                        .insert(
                            "transform".to_string(),
                            MaterialParam::Matrix4(transform.into()),
                        );
                    let _ = _map
                        .insert("pos1".to_string(), MaterialParam::Vector2D(pos1));
                    let _ = _map
                        .insert("pos2".to_string(), MaterialParam::Vector2D(pos2));
                    let _ = _map.insert("width".to_string(), MaterialParam::Uint(width));
                    let _ = _map
                        .insert("color".to_string(), MaterialParam::Color(color));
                    _map
                },
                z_index,
            );
        }
        pub fn draw_polyline(
            &self,
            points: Vec<Vector2D>,
            width: u32,
            color: Color,
            z_index: i32,
            transform: Matrix3,
        ) {
            points
                .adjacent_pairs()
                .for_each(|(pos1, pos2)| {
                    self.draw_line(pos1, pos2, width, color, z_index, transform)
                });
        }
        pub fn draw_dotted_line(
            &self,
            pos1: Vector2D,
            pos2: Vector2D,
            width: u32,
            color: Color,
            z_index: i32,
            transform: Matrix3,
        ) {
            self.draw_quad(
                0,
                self.draw_dotted_line_material.clone(),
                {
                    let _cap = <[()]>::len(&[(), (), (), (), ()]);
                    let mut _map = ::std::collections::HashMap::with_capacity(_cap);
                    let _ = _map
                        .insert(
                            "transform".to_string(),
                            MaterialParam::Matrix4(transform.into()),
                        );
                    let _ = _map
                        .insert("pos1".to_string(), MaterialParam::Vector2D(pos1));
                    let _ = _map
                        .insert("pos2".to_string(), MaterialParam::Vector2D(pos2));
                    let _ = _map.insert("width".to_string(), MaterialParam::Uint(width));
                    let _ = _map
                        .insert("color".to_string(), MaterialParam::Color(color));
                    _map
                },
                z_index,
            );
        }
        pub fn draw_rect(
            &self,
            bottom_left: Vector2D,
            top_right: Vector2D,
            width: u32,
            fill_color: Color,
            border_color: Color,
            z_index: i32,
            transform: Matrix3,
        ) {
            self.draw_quad(
                0,
                self.draw_rect_material.clone(),
                {
                    let _cap = <[()]>::len(&[(), (), (), (), (), ()]);
                    let mut _map = ::std::collections::HashMap::with_capacity(_cap);
                    let _ = _map
                        .insert(
                            "transform".to_string(),
                            MaterialParam::Matrix4(transform.into()),
                        );
                    let _ = _map
                        .insert(
                            "bottom_left".to_string(),
                            MaterialParam::Vector2D(bottom_left),
                        );
                    let _ = _map
                        .insert(
                            "top_right".to_string(),
                            MaterialParam::Vector2D(top_right),
                        );
                    let _ = _map.insert("width".to_string(), MaterialParam::Uint(width));
                    let _ = _map
                        .insert(
                            "fill_color".to_string(),
                            MaterialParam::Color(fill_color),
                        );
                    let _ = _map
                        .insert(
                            "border_color".to_string(),
                            MaterialParam::Color(border_color),
                        );
                    _map
                },
                z_index,
            );
        }
        pub fn draw_arc(
            &self,
            center: Vector2D,
            radius: f32,
            angle_range: Range<f32>,
            width: u32,
            fill_color: Color,
            border_color: Color,
            z_index: i32,
            transform: Matrix3,
        ) {
            let angle_range = normalize_angle_range(angle_range);
            self.draw_quad(
                0,
                self.draw_arc_material.clone(),
                {
                    let _cap = <[()]>::len(&[(), (), (), (), (), (), (), ()]);
                    let mut _map = ::std::collections::HashMap::with_capacity(_cap);
                    let _ = _map
                        .insert(
                            "transform".to_string(),
                            MaterialParam::Matrix4(transform.into()),
                        );
                    let _ = _map
                        .insert("center".to_string(), MaterialParam::Vector2D(center));
                    let _ = _map
                        .insert("radius".to_string(), MaterialParam::Float(radius));
                    let _ = _map
                        .insert(
                            "fill_color".to_string(),
                            MaterialParam::Color(fill_color),
                        );
                    let _ = _map
                        .insert(
                            "border_color".to_string(),
                            MaterialParam::Color(border_color),
                        );
                    let _ = _map.insert("width".to_string(), MaterialParam::Uint(width));
                    let _ = _map
                        .insert(
                            "angle_start".to_string(),
                            MaterialParam::Float(angle_range.start),
                        );
                    let _ = _map
                        .insert(
                            "angle_end".to_string(),
                            MaterialParam::Float(angle_range.end),
                        );
                    _map
                },
                z_index,
            );
        }
        pub fn draw_circle(
            &self,
            center: Vector2D,
            radius: f32,
            width: u32,
            fill_color: Color,
            border_color: Color,
            z_index: i32,
            transform: Matrix3,
        ) {
            self.draw_arc(
                center,
                radius,
                0.0..(2.0 * PI),
                width,
                fill_color,
                border_color,
                z_index,
                transform,
            );
        }
    }
    impl ::fruity_game_engine::introspect::IntrospectMethods for Graphic2dService {
        fn get_const_method_names(
            &self,
        ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(
                <[_]>::into_vec(
                    #[rustc_box]
                    ::alloc::boxed::Box::new([
                        "draw_quad".to_string(),
                        "draw_line".to_string(),
                        "draw_polyline".to_string(),
                        "draw_dotted_line".to_string(),
                        "draw_rect".to_string(),
                        "draw_arc".to_string(),
                        "draw_circle".to_string(),
                    ]),
                ),
            )
        }
        fn call_const_method(
            &self,
            name: &str,
            __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            use ::fruity_game_engine::script_value::TryIntoScriptValue;
            match name {
                "draw_quad" => {
                    let mut __caster = ::fruity_game_engine::utils::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<u64>()?;
                    let __arg_1 = __caster
                        .cast_next::<ResourceReference<dyn MaterialResource>>()?;
                    let __arg_2 = __caster
                        .cast_next::<HashMap<String, MaterialParam>>()?;
                    let __arg_3 = __caster.cast_next::<i32>()?;
                    self.draw_quad(__arg_0, __arg_1, __arg_2, __arg_3)
                        .into_script_value()
                }
                "draw_line" => {
                    let mut __caster = ::fruity_game_engine::utils::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<Vector2D>()?;
                    let __arg_1 = __caster.cast_next::<Vector2D>()?;
                    let __arg_2 = __caster.cast_next::<u32>()?;
                    let __arg_3 = __caster.cast_next::<Color>()?;
                    let __arg_4 = __caster.cast_next::<i32>()?;
                    let __arg_5 = __caster.cast_next::<Matrix3>()?;
                    self.draw_line(__arg_0, __arg_1, __arg_2, __arg_3, __arg_4, __arg_5)
                        .into_script_value()
                }
                "draw_polyline" => {
                    let mut __caster = ::fruity_game_engine::utils::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<Vec<Vector2D>>()?;
                    let __arg_1 = __caster.cast_next::<u32>()?;
                    let __arg_2 = __caster.cast_next::<Color>()?;
                    let __arg_3 = __caster.cast_next::<i32>()?;
                    let __arg_4 = __caster.cast_next::<Matrix3>()?;
                    self.draw_polyline(__arg_0, __arg_1, __arg_2, __arg_3, __arg_4)
                        .into_script_value()
                }
                "draw_dotted_line" => {
                    let mut __caster = ::fruity_game_engine::utils::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<Vector2D>()?;
                    let __arg_1 = __caster.cast_next::<Vector2D>()?;
                    let __arg_2 = __caster.cast_next::<u32>()?;
                    let __arg_3 = __caster.cast_next::<Color>()?;
                    let __arg_4 = __caster.cast_next::<i32>()?;
                    let __arg_5 = __caster.cast_next::<Matrix3>()?;
                    self.draw_dotted_line(
                            __arg_0,
                            __arg_1,
                            __arg_2,
                            __arg_3,
                            __arg_4,
                            __arg_5,
                        )
                        .into_script_value()
                }
                "draw_rect" => {
                    let mut __caster = ::fruity_game_engine::utils::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<Vector2D>()?;
                    let __arg_1 = __caster.cast_next::<Vector2D>()?;
                    let __arg_2 = __caster.cast_next::<u32>()?;
                    let __arg_3 = __caster.cast_next::<Color>()?;
                    let __arg_4 = __caster.cast_next::<Color>()?;
                    let __arg_5 = __caster.cast_next::<i32>()?;
                    let __arg_6 = __caster.cast_next::<Matrix3>()?;
                    self.draw_rect(
                            __arg_0,
                            __arg_1,
                            __arg_2,
                            __arg_3,
                            __arg_4,
                            __arg_5,
                            __arg_6,
                        )
                        .into_script_value()
                }
                "draw_arc" => {
                    let mut __caster = ::fruity_game_engine::utils::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<Vector2D>()?;
                    let __arg_1 = __caster.cast_next::<f32>()?;
                    let __arg_2 = __caster.cast_next::<Range<f32>>()?;
                    let __arg_3 = __caster.cast_next::<u32>()?;
                    let __arg_4 = __caster.cast_next::<Color>()?;
                    let __arg_5 = __caster.cast_next::<Color>()?;
                    let __arg_6 = __caster.cast_next::<i32>()?;
                    let __arg_7 = __caster.cast_next::<Matrix3>()?;
                    self.draw_arc(
                            __arg_0,
                            __arg_1,
                            __arg_2,
                            __arg_3,
                            __arg_4,
                            __arg_5,
                            __arg_6,
                            __arg_7,
                        )
                        .into_script_value()
                }
                "draw_circle" => {
                    let mut __caster = ::fruity_game_engine::utils::ArgumentCaster::new(
                        __args,
                    );
                    let __arg_0 = __caster.cast_next::<Vector2D>()?;
                    let __arg_1 = __caster.cast_next::<f32>()?;
                    let __arg_2 = __caster.cast_next::<u32>()?;
                    let __arg_3 = __caster.cast_next::<Color>()?;
                    let __arg_4 = __caster.cast_next::<Color>()?;
                    let __arg_5 = __caster.cast_next::<i32>()?;
                    let __arg_6 = __caster.cast_next::<Matrix3>()?;
                    self.draw_circle(
                            __arg_0,
                            __arg_1,
                            __arg_2,
                            __arg_3,
                            __arg_4,
                            __arg_5,
                            __arg_6,
                        )
                        .into_script_value()
                }
                _ => ::core::panicking::panic("internal error: entered unreachable code"),
            }
        }
        fn get_mut_method_names(
            &self,
        ) -> ::fruity_game_engine::FruityResult<Vec<String>> {
            Ok(::alloc::vec::Vec::new())
        }
        fn call_mut_method(
            &mut self,
            name: &str,
            __args: Vec<::fruity_game_engine::script_value::ScriptValue>,
        ) -> ::fruity_game_engine::FruityResult<
            ::fruity_game_engine::script_value::ScriptValue,
        > {
            ::core::panicking::panic("internal error: entered unreachable code")
        }
    }
    /// Take a radian angle and normalize it between [-PI, PI[
    ///
    /// # Arguments
    /// * `angle` - The input angle
    ///
    pub fn normalize_angle(angle: f32) -> f32 {
        if angle < -PI {
            normalize_angle(angle + 2.0 * PI)
        } else if angle >= PI {
            normalize_angle(angle - 2.0 * PI)
        } else {
            angle
        }
    }
    /// Take a radian angle range and normalize each born between [-PI, PI[
    /// If the range length is 2PI, returns simply -PI..PI
    ///
    /// # Arguments
    /// * `range` - The input range
    ///
    pub fn normalize_angle_range(range: Range<f32>) -> Range<f32> {
        if range.start == range.end {
            return 0.0..0.0;
        }
        let angle1 = normalize_angle(range.start);
        let angle2 = normalize_angle(range.end);
        let start = f32::min(angle1, angle2);
        let end = f32::max(angle1, angle2);
        if start == end { -PI..PI } else { start..end }
    }
}
pub mod systems {
    pub mod draw_camera {
        use crate::Camera;
        use crate::Transform2D;
        use fruity_ecs::query::Query;
        use fruity_ecs::query::With;
        use fruity_game_engine::inject::Ref;
        use fruity_game_engine::FruityResult;
        use fruity_graphic::graphic_service::GraphicService;
        use fruity_graphic::math::matrix4::Matrix4;
        use fruity_graphic::math::vector2d::Vector2D;
        pub fn draw_camera(
            graphic_service: Ref<dyn GraphicService>,
            query: Query<(With<Transform2D>, With<Camera>)>,
        ) -> FruityResult<()> {
            query
                .for_each(|(transform, camera)| {
                    let bottom_left = transform.transform * Vector2D::new(-0.5, -0.5);
                    let top_right = transform.transform * Vector2D::new(0.5, 0.5);
                    let view_proj = Matrix4::from_rect(
                        bottom_left.x,
                        top_right.x,
                        bottom_left.y,
                        top_right.y,
                        camera.near,
                        camera.far,
                    );
                    {
                        let graphic_service = graphic_service.read();
                        graphic_service
                            .render_scene(
                                view_proj,
                                camera.background_color,
                                camera.target.clone(),
                            );
                    }
                    Ok(())
                })
        }
    }
    pub mod draw_sprite {
        use crate::Graphic2dService;
        use crate::Sprite;
        use crate::Transform2D;
        use fruity_ecs::query::Query;
        use fruity_ecs::query::With;
        use fruity_ecs::query::WithId;
        use fruity_game_engine::inject::Ref;
        use fruity_game_engine::FruityResult;
        use fruity_graphic::graphic_service::MaterialParam;
        use maplit::hashmap;
        pub fn draw_sprite(
            graphic_2d_service: Ref<Graphic2dService>,
            query: Query<(WithId, With<Transform2D>, With<Sprite>)>,
        ) -> FruityResult<()> {
            query
                .for_each(|(entity_id, transform, sprite)| {
                    let graphic_2d_service = graphic_2d_service.read();
                    if let Some(material) = &sprite.material {
                        graphic_2d_service
                            .draw_quad(
                                entity_id.0,
                                material.clone(),
                                {
                                    let _cap = <[()]>::len(&[()]);
                                    let mut _map = ::std::collections::HashMap::with_capacity(
                                        _cap,
                                    );
                                    let _ = _map
                                        .insert(
                                            "transform".to_string(),
                                            MaterialParam::Matrix4(transform.transform.into()),
                                        );
                                    _map
                                },
                                sprite.z_index,
                            );
                    }
                    Ok(())
                })
        }
    }
    pub mod update_transform_2d {
        use crate::Rotate2D;
        use crate::Scale2D;
        use crate::Transform2D;
        use crate::Translate2D;
        use fruity_ecs::query::Query;
        use fruity_ecs::query::WithMut;
        use fruity_ecs::query::WithOptional;
        use fruity_game_engine::FruityResult;
        use fruity_graphic::math::matrix3::Matrix3;
        pub fn update_transform_2d(
            query: Query<
                (
                    WithMut<Transform2D>,
                    WithOptional<Translate2D>,
                    WithOptional<Rotate2D>,
                    WithOptional<Scale2D>,
                ),
            >,
        ) -> FruityResult<()> {
            query
                .for_each(|(mut transform, translate_2d, rotate_2d, scale_2d)| {
                    transform.transform = Matrix3::new_identity();
                    if let Some(translate_2d) = translate_2d {
                        transform
                            .transform = transform.transform
                            * Matrix3::new_translation(translate_2d.vec);
                    }
                    if let Some(rotate_2d) = rotate_2d {
                        transform
                            .transform = transform.transform
                            * Matrix3::new_rotation(rotate_2d.angle);
                    }
                    if let Some(scale_2d) = scale_2d {
                        transform
                            .transform = transform.transform
                            * Matrix3::new_scaling(scale_2d.vec);
                    }
                    Ok(())
                })
        }
    }
}
/// Returns the module, ready to be registered into the fruity_game_engine
pub fn create_fruity_graphic_2d_module() -> Module {
    Module {
        name: "fruity_graphic_2d".to_string(),
        dependencies: <[_]>::into_vec(
            #[rustc_box]
            ::alloc::boxed::Box::new([
                "fruity_ecs".to_string(),
                "fruity_graphic".to_string(),
                "fruity_windows".to_string(),
            ]),
        ),
        setup: Some(
            Arc::new(|world, _settings| {
                let resource_container = world.get_resource_container();
                let graphic_2d_service = Graphic2dService::new(
                    resource_container.clone(),
                )?;
                resource_container
                    .add::<
                        Graphic2dService,
                    >("graphic_2d_service", Box::new(graphic_2d_service));
                let serialization_service = resource_container
                    .require::<SerializationService>();
                let mut serialization_service = serialization_service.write();
                serialization_service.register_component::<Transform2D>();
                serialization_service.register_component::<Translate2D>();
                serialization_service.register_component::<Rotate2D>();
                serialization_service.register_component::<Scale2D>();
                serialization_service.register_component::<Sprite>();
                serialization_service.register_component::<Camera>();
                let system_service = resource_container.require::<SystemService>();
                let mut system_service = system_service.write();
                system_service
                    .add_system(
                        "update_transform_2d",
                        &update_transform_2d as &'static (dyn Fn(_) -> _ + Send + Sync),
                        Some(SystemParams {
                            pool_index: Some(95),
                            ignore_pause: Some(true),
                            ..Default::default()
                        }),
                    );
                system_service
                    .add_system(
                        "draw_sprite",
                        &draw_sprite as &'static (dyn Fn(_, _) -> _ + Send + Sync),
                        Some(SystemParams {
                            pool_index: Some(98),
                            ignore_pause: Some(true),
                            ..Default::default()
                        }),
                    );
                system_service
                    .add_system(
                        "draw_camera",
                        &draw_camera as &'static (dyn Fn(_, _) -> _ + Send + Sync),
                        Some(SystemParams {
                            pool_index: Some(99),
                            ignore_pause: Some(true),
                            ..Default::default()
                        }),
                    );
                Ok(())
            }),
        ),
        ..Default::default()
    }
}
#[doc(hidden)]
#[allow(non_snake_case)]
extern "C" fn __napi__createFruityGraphic2DModule(
    raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
    cb: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_callback_info,
) -> ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value {
    use ::fruity_game_engine::napi::NapiValue;
    use ::fruity_game_engine::script_value::TryFromScriptValue;
    use ::fruity_game_engine::script_value::TryIntoScriptValue;
    unsafe {
        let env = ::fruity_game_engine::napi::Env::from_raw(raw_env);
        ::fruity_game_engine::napi::bindgen_prelude::CallbackInfo::<
            2usize,
        >::new(raw_env, cb, None)
            .and_then(|cb| {
                ::fruity_game_engine::napi::bindgen_prelude::within_runtime_if_available(move ||
                {
                    let _ret = { create_fruity_graphic_2d_module() };
                    let _ret = <Module>::into_script_value(_ret)
                        .map_err(|e| e.into_napi())?;
                    let _ret = ::fruity_game_engine::javascript::script_value_to_js_value(
                            &env,
                            _ret,
                        )
                        .map_err(|e| e.into_napi())?;
                    <::fruity_game_engine::napi::JsUnknown as ::fruity_game_engine::napi::bindgen_prelude::ToNapiValue>::to_napi_value(
                        raw_env,
                        _ret,
                    )
                })
            })
            .unwrap_or_else(|e| {
                ::fruity_game_engine::napi::bindgen_prelude::JsError::from(e)
                    .throw_into(raw_env);
                std::ptr::null_mut::<
                    ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value__,
                >()
            })
    }
}
#[doc(hidden)]
#[allow(dead_code)]
pub unsafe fn createFruityGraphic2DModule_js_function(
    raw_env: ::fruity_game_engine::napi::bindgen_prelude::sys::napi_env,
) -> ::fruity_game_engine::napi::bindgen_prelude::Result<
    ::fruity_game_engine::napi::bindgen_prelude::sys::napi_value,
> {
    struct TransmutedTypeId {
        t: u64,
    }
    let mut fn_ptr = std::ptr::null_mut();
    {
        let c = ::fruity_game_engine::napi::bindgen_prelude::sys::napi_create_function(
            raw_env,
            "createFruityGraphic2DModule\0".as_ptr() as *const _,
            28usize,
            Some(__napi__createFruityGraphic2DModule),
            std::ptr::null_mut(),
            &mut fn_ptr,
        );
        match c {
            ::fruity_game_engine::napi::sys::Status::napi_ok => Ok(()),
            _ => {
                Err(
                    ::fruity_game_engine::napi::Error::new(
                        ::fruity_game_engine::napi::Status::from(c),
                        {
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "Failed to register function `{0}`",
                                    "createFruityGraphic2DModule"
                                ),
                            );
                            res
                        },
                    ),
                )
            }
        }
    }?;
    ::fruity_game_engine::napi::bindgen_prelude::register_js_function(
        "createFruityGraphic2DModule\0",
        createFruityGraphic2DModule_js_function,
        Some(__napi__createFruityGraphic2DModule),
    );
    Ok(fn_ptr)
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub extern "C" fn __napi_register__createFruityGraphic2DModule() {
    ::fruity_game_engine::napi::bindgen_prelude::register_module_export(
        None,
        "createFruityGraphic2DModule\0",
        createFruityGraphic2DModule_js_function,
    );
}
#[used]
#[allow(non_upper_case_globals)]
#[allow(non_snake_case)]
#[doc(hidden)]
#[no_mangle]
#[link_section = "__DATA,__mod_init_func"]
pub static __napi_register__createFruityGraphic2DModule___rust_ctor___ctor: unsafe extern "C" fn() = {
    unsafe extern "C" fn __napi_register__createFruityGraphic2DModule___rust_ctor___ctor() {
        __napi_register__createFruityGraphic2DModule()
    }
    __napi_register__createFruityGraphic2DModule___rust_ctor___ctor
};
