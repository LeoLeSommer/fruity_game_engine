use crate::{
    any::FruityAny,
    introspect::{IntrospectFields, IntrospectMethods},
    javascript::napi_script::class_constructors::NapiClassConstructors,
    profile_scope,
    script_value::{ScriptObject, ScriptValue, TryFromScriptValue, TryIntoScriptValue},
    FruityError, FruityResult,
};
use convert_case::{Case, Casing};
use futures::{executor::block_on, FutureExt};
use lazy_static::lazy_static;
use napi::{
    bindgen_prelude::{FromNapiValue, Promise, ToNapiValue},
    check_status,
    threadsafe_function::{ErrorStrategy, ThreadSafeCallContext, ThreadsafeFunction},
    Env, JsBigInt, JsFunction, JsNumber, JsObject, JsString, JsUnknown, NapiRaw, NapiValue, Ref,
    Task, ValueType,
};
use send_wrapper::SendWrapper;
use std::{
    fmt::Debug, future::Future, marker::PhantomData, ops::Deref, pin::Pin, rc::Rc, sync::Arc,
    thread, vec,
};
use tokio::runtime::Builder;

mod class_constructors;

lazy_static! {
    static ref NAPI_CLASS_CONSTRUCTORS: NapiClassConstructors = NapiClassConstructors::default();
}

/// Create a napi js value from a script value
pub fn script_value_to_js_value(env: &Env, value: ScriptValue) -> FruityResult<JsUnknown> {
    profile_scope!("script_value_to_js_value");

    Ok(match value.into_script_value()? {
        ScriptValue::I8(value) => env
            .create_int32(value as i32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::I16(value) => env
            .create_int32(value as i32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::I32(value) => env
            .create_int32(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::I64(value) => env
            .create_int64(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::ISize(value) => env
            .create_int32(value as i32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U8(value) => env
            .create_uint32(value as u32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U16(value) => env
            .create_uint32(value as u32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U32(value) => env
            .create_uint32(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::U64(value) => env
            .create_bigint_from_u64(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown()
            .map_err(|e| FruityError::from_napi(e))?,
        ScriptValue::USize(value) => env
            .create_uint32(value as u32)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::F32(value) => env
            .create_double(value as f64)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::F64(value) => env
            .create_double(value as f64)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Bool(value) => env
            .get_boolean(value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::String(value) => env
            .create_string(&value)
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Array(value) => {
            profile_scope!("script_value_to_js_value_array");
            let mut js_array = env
                .create_empty_array()
                .map_err(|e| FruityError::from_napi(e))?;

            for (index, elem) in value.into_iter().enumerate() {
                js_array
                    .set_element(index as u32, script_value_to_js_value(env, elem)?)
                    .map_err(|e| FruityError::from_napi(e))?;
            }

            js_array.into_unknown()
        }
        ScriptValue::Null => env
            .get_null()
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Undefined => env
            .get_undefined()
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown(),
        ScriptValue::Future(future) => {
            profile_scope!("script_value_to_js_value_future");
            let (js_deferred, js_promise) = env
                .create_deferred()
                .map_err(|e| FruityError::from_napi(e))?;

            thread::spawn(|| {
                block_on(async {
                    match future.await {
                        Ok(result) => js_deferred.resolve(|env| {
                            script_value_to_js_value(&env, result).map_err(|err| err.into_napi())
                        }),
                        Err(err) => js_deferred.reject(err.into_napi()),
                    }
                })
            });

            js_promise.into_unknown()
        }
        ScriptValue::Callback(callback) => {
            profile_scope!("script_value_to_js_value_closure");
            env.create_function_from_closure("unknown", move |ctx| {
                let args = ctx
                    .get_all()
                    .into_iter()
                    .map(|elem| js_value_to_script_value(ctx.env, elem))
                    .try_collect::<Vec<_>>()
                    .map_err(|e| e.into_napi())?;

                let result = callback(args).map_err(|e| e.into_napi())?;
                script_value_to_js_value(ctx.env, result).map_err(|e| e.into_napi())
            })
            .map_err(|e| FruityError::from_napi(e))?
            .into_unknown()
        }
        ScriptValue::Object(value) => {
            profile_scope!("script_value_to_js_value_object");
            match value.downcast::<JsIntrospectObject>() {
                // First case, it's a native js value
                Ok(value) => {
                    let js_object: JsObject = value.reference.inner();
                    js_object.into_unknown()
                }
                // Second case, we wrap the object into a js object
                Err(value) => unsafe {
                    let raw = NAPI_CLASS_CONSTRUCTORS
                        .instantiate(env.raw(), value)
                        .map_err(|err| FruityError::from_napi(err))?;
                    JsUnknown::from_raw(env.raw(), raw)
                        .map_err(|err| FruityError::from_napi(err))?
                },
            }
        }
    })
}

/// Create a script value from a napi js value
pub fn js_value_to_script_value(env: &Env, value: JsUnknown) -> FruityResult<ScriptValue> {
    profile_scope!("js_value_to_script_value");

    Ok(
        match value.get_type().map_err(|e| FruityError::from_napi(e))? {
            ValueType::Undefined => ScriptValue::Undefined,
            ValueType::Null => ScriptValue::Null,
            ValueType::Boolean => ScriptValue::Bool(
                value
                    .coerce_to_bool()
                    .map_err(|e| FruityError::from_napi(e))?
                    .get_value()
                    .map_err(|e| FruityError::from_napi(e))?,
            ),
            ValueType::Number => ScriptValue::F64(
                value
                    .coerce_to_number()
                    .map_err(|e| FruityError::from_napi(e))?
                    .get_double()
                    .map_err(|e| FruityError::from_napi(e))?,
            ),
            ValueType::String => ScriptValue::String(
                value
                    .coerce_to_string()
                    .map_err(|e| FruityError::from_napi(e))?
                    .into_utf8()
                    .map_err(|e| FruityError::from_napi(e))?
                    .as_str()
                    .map_err(|e| FruityError::from_napi(e))?
                    .to_string(),
            ),
            ValueType::Symbol => unimplemented!(),
            ValueType::Object => {
                let js_object = value
                    .coerce_to_object()
                    .map_err(|e| FruityError::from_napi(e))?;

                if js_object
                    .is_array()
                    .map_err(|e| FruityError::from_napi(e))?
                {
                    profile_scope!("js_value_to_script_value_array");
                    // First case, the object is a plain javascript array
                    ScriptValue::Array(
                        (0..js_object
                            .get_array_length()
                            .map_err(|e| FruityError::from_napi(e))?)
                            .map(|index| {
                                js_value_to_script_value(
                                    env,
                                    js_object
                                        .get_element(index)
                                        .map_err(|e| FruityError::from_napi(e))?,
                                )
                            })
                            .try_collect::<Vec<_>>()?,
                    )
                } else if js_object
                    .is_promise()
                    .map_err(|e| FruityError::from_napi(e))?
                {
                    profile_scope!("js_value_to_script_value_array");
                    // Convert js value to promise
                    let promise = Promise::<ScriptValue>::from_unknown(js_object.into_unknown())
                        .map_err(|e| FruityError::from_napi(e))?;

                    // Convert promise to future
                    let future = Box::pin(async move {
                        let res = promise.await.map_err(|e| FruityError::from_napi(e));
                        res
                    })
                        as Pin<Box<dyn Send + Future<Output = FruityResult<ScriptValue>>>>;

                    ScriptValue::Future(future)
                } else {
                    profile_scope!("js_value_to_script_value_object");

                    // Get the wrapped object
                    let mut wrapped = std::ptr::null_mut();
                    let unwrap_result = check_status!(
                        unsafe { napi_sys::napi_unwrap(env.raw(), js_object.raw(), &mut wrapped) },
                        "Unwrap value [{}] from class failed",
                        std::any::type_name::<Box<dyn ScriptObject>>(),
                    );

                    match unwrap_result {
                        Ok(_) => {
                            let wrapped = wrapped as *mut Box<dyn ScriptObject>;
                            let wrapped = unsafe { Box::from_raw(wrapped) };

                            // Second case, a value is wrapped into the object
                            ScriptValue::Object(*wrapped)
                        }
                        Err(_) => {
                            // Third case, the object is a plain javascript object
                            ScriptValue::Object(Box::new(JsIntrospectObject {
                                reference: SendWrapper::new(JsSharedRef::new(env, js_object)?),
                            }))
                        }
                    }
                }
            }
            ValueType::Function => {
                profile_scope!("js_value_to_script_value_function");
                let js_func = JsFunction::try_from(value).map_err(|e| FruityError::from_napi(e))?;
                let js_func = JsSharedRef::new(env, js_func)?;

                let thread_safe_func: ThreadsafeFunction<Vec<ScriptValue>, ErrorStrategy::Fatal> =
                    js_func
                        .inner()
                        .create_threadsafe_function(
                            0,
                            |ctx: ThreadSafeCallContext<Vec<ScriptValue>>| Ok(ctx.value),
                        )
                        .map_err(|e| FruityError::from_napi(e))?;
                let thread_safe_func = ThreadsafeFunctionSync(thread_safe_func);

                let js_send_wrapper = SendWrapper::new((env.clone(), js_func));
                ScriptValue::Callback(Box::new(move |args| {
                    // Case the js function is called in the js thread, we call it directly
                    // Otherwise, we call the function in the js thread and wait for the result in our thread
                    let result = if js_send_wrapper.valid() {
                        let (env, js_func) = js_send_wrapper.deref();

                        // Get the js func from the reference
                        let js_func = js_func.inner();

                        // Convert all the others args as a JsUnknown
                        let args = args
                            .into_iter()
                            .map(|elem| script_value_to_js_value(&env, elem))
                            .try_collect::<Vec<_>>()?;

                        // Call the function
                        let result = js_func
                            .call(None, &args)
                            .map_err(|e| FruityError::from_napi(e))?;

                        // Return the result
                        let result = js_value_to_script_value(&env, result)?;
                        Ok(result)
                    } else {
                        let result: ScriptValue = Builder::new_current_thread()
                            .enable_all()
                            .build()
                            .unwrap()
                            .block_on(thread_safe_func.call_async(args))
                            .map_err(|e| FruityError::from_napi(e))?;

                        Ok(result)
                    };

                    result
                }))
            }
            ValueType::External => unimplemented!(),
            ValueType::BigInt => ScriptValue::I64(
                unsafe { value.cast::<JsBigInt>() }
                    .get_i64()
                    .map_err(|e| FruityError::from_napi(e))?
                    .0,
            ),
            ValueType::Unknown => unimplemented!(),
        },
    )
}

struct ThreadsafeFunctionSync(ThreadsafeFunction<Vec<ScriptValue>, ErrorStrategy::Fatal>);

unsafe impl Send for ThreadsafeFunctionSync {}
unsafe impl Sync for ThreadsafeFunctionSync {}

impl Deref for ThreadsafeFunctionSync {
    type Target = ThreadsafeFunction<Vec<ScriptValue>, ErrorStrategy::Fatal>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct RefWrapper {
    reference: Ref<()>,
    env: Env,
}

impl Drop for RefWrapper {
    fn drop(&mut self) {
        self.reference.unref(self.env.clone()).unwrap();
    }
}

struct JsSharedRef<T>
where
    T: NapiRaw,
{
    reference: Rc<RefWrapper>,
    env: Env,
    phantom: PhantomData<T>,
}

impl<T> Debug for JsSharedRef<T>
where
    T: NapiRaw,
{
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<T> Clone for JsSharedRef<T>
where
    T: NapiRaw,
{
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
            env: self.env.clone(),
            phantom: Default::default(),
        }
    }
}

impl<T> JsSharedRef<T>
where
    T: NapiRaw + NapiValue,
{
    pub fn new(env: &Env, value: T) -> FruityResult<Self> {
        Ok(Self {
            reference: Rc::new(RefWrapper {
                reference: env
                    .create_reference(value)
                    .map_err(|e| FruityError::from_napi(e))?,
                env: env.clone(),
            }),
            env: env.clone(),
            phantom: Default::default(),
        })
    }

    pub fn inner(&self) -> T {
        self.env
            .get_reference_value::<T>(&self.reference.reference)
            .unwrap()
    }
}

/// A structure to store a javascript object that can be stored in a ScriptValue
#[derive(FruityAny, Clone)]
pub struct JsIntrospectObject {
    reference: SendWrapper<JsSharedRef<JsObject>>,
}

impl Debug for JsIntrospectObject {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl IntrospectFields for JsIntrospectObject {
    fn is_static(&self) -> FruityResult<bool> {
        Ok(false)
    }

    fn get_class_name(&self) -> FruityResult<String> {
        // Get the js func object the reference
        let js_object = self.reference.inner();

        let constructor: JsFunction = js_object
            .get_named_property("constructor")
            .map_err(|e| FruityError::from_napi(e))?;
        let constructor = constructor
            .coerce_to_object()
            .map_err(|e| FruityError::from_napi(e))?;
        let name: JsString = constructor
            .get_named_property("name")
            .map_err(|e| FruityError::from_napi(e))?;
        Ok(name
            .into_utf8()
            .map_err(|e| FruityError::from_napi(e))?
            .as_str()
            .map_err(|e| FruityError::from_napi(e))?
            .to_string())
    }

    fn get_field_names(&self) -> FruityResult<Vec<String>> {
        // Get the js func object the reference
        let js_object = self.reference.inner();

        let properties = js_object
            .get_property_names()
            .map_err(|e| FruityError::from_napi(e))?;
        let len = properties
            .get_named_property::<JsNumber>("length")
            .map_err(|e| FruityError::from_napi(e))?
            .get_uint32()
            .map_err(|e| FruityError::from_napi(e))?;

        (0..len)
            .map(|index| {
                let key = properties
                    .get_element::<JsString>(index)
                    .map_err(|e| FruityError::from_napi(e))?;
                let key = key
                    .into_utf8()
                    .map_err(|e| FruityError::from_napi(e))?
                    .as_str()
                    .map_err(|e| FruityError::from_napi(e))?
                    .to_string();

                Ok(key.to_case(Case::Snake))
            })
            .try_collect()
    }

    fn set_field_value(&mut self, name: &str, value: ScriptValue) -> FruityResult<()> {
        // Get the js func object the reference
        let mut js_object = self.reference.inner();

        let value = script_value_to_js_value(&self.reference.env, value)?;
        js_object
            .set_named_property(&name.to_case(Case::Camel), value)
            .map_err(|e| FruityError::from_napi(e))?;

        Ok(())
    }

    fn get_field_value(&self, name: &str) -> FruityResult<ScriptValue> {
        // Get the js func object the reference
        let js_object = self.reference.inner();

        let value = js_object
            .get_named_property(&name.to_case(Case::Camel))
            .map_err(|e| FruityError::from_napi(e))?;
        js_value_to_script_value(&self.reference.env, value)
    }
}

impl IntrospectMethods for JsIntrospectObject {
    fn get_const_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_const_method(&self, _name: &str, _args: Vec<ScriptValue>) -> FruityResult<ScriptValue> {
        unreachable!()
    }

    fn get_mut_method_names(&self) -> FruityResult<Vec<String>> {
        Ok(vec![])
    }

    fn call_mut_method(
        &mut self,
        _name: &str,
        _args: Vec<ScriptValue>,
    ) -> FruityResult<ScriptValue> {
        unreachable!()
    }
}

impl TryIntoScriptValue for JsIntrospectObject {
    fn into_script_value(self) -> FruityResult<ScriptValue> {
        Ok(ScriptValue::Object(Box::new(self)))
    }
}

impl TryFromScriptValue for JsIntrospectObject {
    fn from_script_value(value: ScriptValue) -> FruityResult<Self> {
        match value {
            ScriptValue::Object(value) => match value.downcast::<Self>() {
                Ok(value) => Ok(*value),
                Err(value) => Err(FruityError::InvalidArg(format!(
                    "Couldn't convert a {} to {}",
                    value.deref().get_type_name(),
                    std::any::type_name::<Self>()
                ))),
            },
            value => Err(FruityError::InvalidArg(format!(
                "Couldn't convert {:?} to native object",
                value
            ))),
        }
    }
}

impl FruityError {
    /// Convert a js error to a fruity_game_engine error
    pub fn from_napi(err: napi::Error) -> Self {
        match err.status {
            napi::Status::Ok => FruityError::Ok(err.reason.to_string()),
            napi::Status::InvalidArg => FruityError::InvalidArg(err.reason.to_string()),
            napi::Status::ObjectExpected => FruityError::ObjectExpected(err.reason.to_string()),
            napi::Status::StringExpected => FruityError::StringExpected(err.reason.to_string()),
            napi::Status::NameExpected => FruityError::NameExpected(err.reason.to_string()),
            napi::Status::FunctionExpected => FruityError::FunctionExpected(err.reason.to_string()),
            napi::Status::NumberExpected => FruityError::NumberExpected(err.reason.to_string()),
            napi::Status::BooleanExpected => FruityError::BooleanExpected(err.reason.to_string()),
            napi::Status::ArrayExpected => FruityError::ArrayExpected(err.reason.to_string()),
            napi::Status::GenericFailure => FruityError::GenericFailure(err.reason.to_string()),
            napi::Status::PendingException => FruityError::PendingException(err.reason.to_string()),
            napi::Status::Cancelled => FruityError::Cancelled(err.reason.to_string()),
            napi::Status::EscapeCalledTwice => {
                FruityError::EscapeCalledTwice(err.reason.to_string())
            }
            napi::Status::HandleScopeMismatch => {
                FruityError::HandleScopeMismatch(err.reason.to_string())
            }
            napi::Status::CallbackScopeMismatch => {
                FruityError::CallbackScopeMismatch(err.reason.to_string())
            }
            napi::Status::QueueFull => FruityError::QueueFull(err.reason.to_string()),
            napi::Status::Closing => FruityError::Closing(err.reason.to_string()),
            napi::Status::BigintExpected => FruityError::BigintExpected(err.reason.to_string()),
            napi::Status::DateExpected => FruityError::DateExpected(err.reason.to_string()),
            napi::Status::ArrayBufferExpected => {
                FruityError::ArrayBufferExpected(err.reason.to_string())
            }
            napi::Status::DetachableArraybufferExpected => {
                FruityError::DetachableArraybufferExpected(err.reason.to_string())
            }
            napi::Status::WouldDeadlock => FruityError::WouldDeadlock(err.reason.to_string()),
            napi::Status::NoExternalBuffersAllowed => {
                FruityError::NoExternalBuffersAllowed(err.reason.to_string())
            }
            napi::Status::Unknown => FruityError::Unknown(err.reason.to_string()),
        }
    }

    /// Convert a fruity_game_engine error to a js error
    pub fn into_napi(self) -> napi::Error {
        match self {
            FruityError::Ok(message) => napi::Error::new(napi::Status::Ok, message),
            FruityError::InvalidArg(message) => napi::Error::new(napi::Status::InvalidArg, message),
            FruityError::ObjectExpected(message) => {
                napi::Error::new(napi::Status::ObjectExpected, message)
            }
            FruityError::StringExpected(message) => {
                napi::Error::new(napi::Status::StringExpected, message)
            }
            FruityError::NameExpected(message) => {
                napi::Error::new(napi::Status::NameExpected, message)
            }
            FruityError::FunctionExpected(message) => {
                napi::Error::new(napi::Status::FunctionExpected, message)
            }
            FruityError::NumberExpected(message) => {
                napi::Error::new(napi::Status::NumberExpected, message)
            }
            FruityError::BooleanExpected(message) => {
                napi::Error::new(napi::Status::BooleanExpected, message)
            }
            FruityError::ArrayExpected(message) => {
                napi::Error::new(napi::Status::ArrayExpected, message)
            }
            FruityError::GenericFailure(message) => {
                napi::Error::new(napi::Status::GenericFailure, message)
            }
            FruityError::PendingException(message) => {
                napi::Error::new(napi::Status::PendingException, message)
            }
            FruityError::Cancelled(message) => napi::Error::new(napi::Status::Cancelled, message),
            FruityError::EscapeCalledTwice(message) => {
                napi::Error::new(napi::Status::EscapeCalledTwice, message)
            }
            FruityError::HandleScopeMismatch(message) => {
                napi::Error::new(napi::Status::HandleScopeMismatch, message)
            }
            FruityError::CallbackScopeMismatch(message) => {
                napi::Error::new(napi::Status::CallbackScopeMismatch, message)
            }
            FruityError::QueueFull(message) => napi::Error::new(napi::Status::QueueFull, message),
            FruityError::Closing(message) => napi::Error::new(napi::Status::Closing, message),
            FruityError::BigintExpected(message) => {
                napi::Error::new(napi::Status::BigintExpected, message)
            }
            FruityError::DateExpected(message) => {
                napi::Error::new(napi::Status::DateExpected, message)
            }
            FruityError::ArrayBufferExpected(message) => {
                napi::Error::new(napi::Status::ArrayBufferExpected, message)
            }
            FruityError::DetachableArraybufferExpected(message) => {
                napi::Error::new(napi::Status::DetachableArraybufferExpected, message)
            }
            FruityError::WouldDeadlock(message) => {
                napi::Error::new(napi::Status::WouldDeadlock, message)
            }
            FruityError::NoExternalBuffersAllowed(message) => {
                napi::Error::new(napi::Status::NoExternalBuffersAllowed, message)
            }
            FruityError::Unknown(message) => napi::Error::new(napi::Status::Unknown, message),
        }
    }
}

impl ToNapiValue for ScriptValue {
    unsafe fn to_napi_value(
        env_raw: napi_sys::napi_env,
        script_value: Self,
    ) -> napi::Result<napi_sys::napi_value> {
        let env = Env::from_raw(env_raw);
        let napi_value =
            script_value_to_js_value(&env, script_value).map_err(|err| err.into_napi())?;

        Ok(napi_value.raw())
    }
}

impl FromNapiValue for ScriptValue {
    unsafe fn from_napi_value(
        env_raw: napi_sys::napi_env,
        napi_value: napi_sys::napi_value,
    ) -> napi::Result<Self> {
        let env = Env::from_raw(env_raw);
        let napi_value = JsUnknown::from_raw(env_raw, napi_value)?;

        // TODO: Find a cleaner way to do this
        // No type means that napi value is just nothing, should be undefined but for an obscure reason
        // it happen when thread-safe functions returns nothing
        let has_value = napi_value.get_type().is_ok();
        if has_value {
            let script_value =
                js_value_to_script_value(&env, napi_value).map_err(|err| err.into_napi())?;

            Ok(script_value)
        } else {
            Ok(ScriptValue::Undefined)
        }
    }
}
