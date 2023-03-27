use std::{cell::RefCell, collections::HashMap, ffi::CString, ops::Deref};

use convert_case::{Case, Casing};
use napi::{check_status, Env, JsError, JsUnknown, NapiRaw, NapiValue, PropertyAttributes};

use crate::{
    javascript::{js_value_to_script_value, script_value_to_js_value},
    script_value::ScriptObject,
};

#[derive(Default)]
pub struct NapiClassConstructors(RefCell<HashMap<String, napi::sys::napi_ref>>);

// Safe cause the class constructor is always called in the javascript thread
unsafe impl Send for NapiClassConstructors {}

// Safe cause the class constructor is always called in the javascript thread
unsafe impl Sync for NapiClassConstructors {}

impl NapiClassConstructors {
    pub unsafe fn instantiate(
        &self,
        env: napi_sys::napi_env,
        script_object: Box<dyn ScriptObject>,
    ) -> napi::Result<napi_sys::napi_value> {
        if script_object.is_static().map_err(|err| err.into_napi())? {
            self.instantiate_static(env, script_object)
        } else {
            self.instantiate_dynamic(env, script_object)
        }
    }

    unsafe fn instantiate_static(
        &self,
        env: napi_sys::napi_env,
        script_object: Box<dyn ScriptObject>,
    ) -> napi::Result<napi_sys::napi_value> {
        // Get the existing class if exists, otherwise create it
        let class_name = script_object
            .get_class_name()
            .map_err(|err| err.into_napi())?;

        let ctor_ref = if let Some(ctor_ref) = self.0.borrow().get(&class_name) {
            Some(*ctor_ref)
        } else {
            None
        };

        let ctor_ref = if let Some(ctor_ref) = ctor_ref {
            ctor_ref
        } else {
            self.register_class(env, script_object.deref())?
        };

        let mut ctor = std::ptr::null_mut();
        napi::check_status!(
            napi::sys::napi_get_reference_value(env, ctor_ref, &mut ctor),
            "Failed to get constructor reference of class `{}`",
            &class_name,
        )?;

        // Instantiate object
        let mut result = std::ptr::null_mut();
        napi::check_status!(
            napi_sys::napi_new_instance(env, ctor, 0, std::ptr::null_mut(), &mut result),
            "Failed to construct class `{}`",
            &class_name
        )?;

        // Wrap the rust value inside the object
        let mut object_ref = std::ptr::null_mut();
        let wrapped_value = Box::leak(Box::new(script_object));
        napi::check_status!(
            napi_sys::napi_wrap(
                env,
                result,
                wrapped_value as *mut _ as *mut std::ffi::c_void,
                Some(raw_finalize_wrapped_object_unchecked),
                std::ptr::null_mut(),
                &mut object_ref,
            ),
            "Failed to wrap native object of class `{}`",
            &class_name
        )?;

        Ok(result)
    }

    unsafe fn instantiate_dynamic(
        &self,
        env: napi_sys::napi_env,
        script_object: Box<dyn ScriptObject>,
    ) -> napi::Result<napi_sys::napi_value> {
        let class_name = script_object
            .get_class_name()
            .map_err(|err| err.into_napi())?;

        // Instantiate object
        let mut result = std::ptr::null_mut();
        check_status!(
            napi_sys::napi_create_object(env, &mut result),
            "Failed to create napi Object"
        )?;

        // Get member names
        let field_names = script_object
            .get_field_names()
            .map_err(|err| err.into_napi())?
            .into_iter()
            .map(|field_name| CString::new(field_name.to_case(Case::Camel)).unwrap())
            .collect::<Vec<_>>();

        let const_method_names = script_object
            .get_const_method_names()
            .map_err(|err| err.into_napi())?
            .into_iter()
            .map(|field_name| CString::new(field_name.to_case(Case::Camel)).unwrap())
            .collect::<Vec<_>>();

        let mut_method_names = script_object
            .get_mut_method_names()
            .map_err(|err| err.into_napi())?
            .into_iter()
            .map(|field_name| CString::new(field_name.to_case(Case::Camel)).unwrap())
            .collect::<Vec<_>>();

        // Add members definitions
        let mut properties = Vec::<napi_sys::napi_property_descriptor>::with_capacity(
            field_names.len() + const_method_names.len() + mut_method_names.len(),
        );

        field_names.iter().for_each(|field_name| {
            properties.push(napi_sys::napi_property_descriptor {
                utf8name: field_name.as_ptr(),
                name: std::ptr::null_mut(),
                method: None,
                getter: Some(generic_getter),
                setter: Some(generic_setter),
                value: std::ptr::null_mut(),
                attributes: (PropertyAttributes::Default
                    | PropertyAttributes::Writable
                    | PropertyAttributes::Enumerable)
                    .bits(),
                data: field_name.as_ptr() as *mut std::ffi::c_void,
            })
        });

        const_method_names.iter().for_each(|method_name| {
            properties.push(napi_sys::napi_property_descriptor {
                utf8name: method_name.as_ptr(),
                name: std::ptr::null_mut(),
                method: Some(generic_const_method),
                getter: None,
                setter: None,
                value: std::ptr::null_mut(),
                attributes: (PropertyAttributes::Default
                    | PropertyAttributes::Writable
                    | PropertyAttributes::Enumerable)
                    .bits(),
                data: method_name.as_ptr() as *mut std::ffi::c_void,
            })
        });

        mut_method_names.iter().for_each(|method_name| {
            properties.push(napi_sys::napi_property_descriptor {
                utf8name: method_name.as_ptr(),
                name: std::ptr::null_mut(),
                method: Some(generic_mut_method),
                getter: None,
                setter: None,
                value: std::ptr::null_mut(),
                attributes: (PropertyAttributes::Default
                    | PropertyAttributes::Writable
                    | PropertyAttributes::Enumerable)
                    .bits(),
                data: method_name.as_ptr() as *mut std::ffi::c_void,
            })
        });

        check_status!(napi_sys::napi_define_properties(
            env,
            result,
            properties.len(),
            properties.as_ptr(),
        ))?;

        // Wrap the rust value inside the object
        let mut object_ref = std::ptr::null_mut();
        let wrapped_value = Box::leak(Box::new(script_object));
        napi::check_status!(
            napi_sys::napi_wrap(
                env,
                result,
                wrapped_value as *mut _ as *mut std::ffi::c_void,
                Some(raw_finalize_wrapped_object_unchecked),
                std::ptr::null_mut(),
                &mut object_ref,
            ),
            "Failed to wrap native object of object `{}`",
            &class_name
        )?;

        // Add a finalize to store the member names
        let mut maybe_ref = std::ptr::null_mut();
        let wrap_context = Box::leak(Box::new(FinalizeMemberNamesData {
            _field_names: field_names,
            _const_method_names: const_method_names,
            _mut_method_names: mut_method_names,
        }));
        check_status!(unsafe {
            napi_sys::napi_add_finalizer(
                env,
                result,
                wrap_context as *mut _ as *mut std::ffi::c_void,
                Some(raw_finalize_member_names_unchecked),
                std::ptr::null_mut(),
                &mut maybe_ref,
            )
        })?;

        Ok(result)
    }

    unsafe fn register_class(
        &self,
        env: napi_sys::napi_env,
        script_object: &dyn ScriptObject,
    ) -> napi::Result<napi::sys::napi_ref> {
        let class_name = script_object
            .get_class_name()
            .map_err(|err| err.into_napi())?;
        let js_class_name = std::ffi::CStr::from_bytes_with_nul_unchecked(class_name.as_bytes());
        let mut class_ptr = std::ptr::null_mut();
        let mut ctor_ref = std::ptr::null_mut();

        // Add members definitions
        let field_names = script_object
            .get_field_names()
            .map_err(|err| err.into_napi())?
            .into_iter()
            .map(|field_name| CString::new(field_name.to_case(Case::Camel)).unwrap())
            .collect::<Vec<_>>();

        let const_method_names = script_object
            .get_const_method_names()
            .map_err(|err| err.into_napi())?
            .into_iter()
            .map(|field_name| CString::new(field_name.to_case(Case::Camel)).unwrap())
            .collect::<Vec<_>>();

        let mut_method_names = script_object
            .get_mut_method_names()
            .map_err(|err| err.into_napi())?
            .into_iter()
            .map(|field_name| CString::new(field_name.to_case(Case::Camel)).unwrap())
            .collect::<Vec<_>>();

        let mut properties = Vec::<napi_sys::napi_property_descriptor>::with_capacity(
            field_names.len() + const_method_names.len() + mut_method_names.len(),
        );

        field_names.iter().for_each(|field_name| {
            properties.push(napi_sys::napi_property_descriptor {
                utf8name: field_name.as_ptr(),
                name: std::ptr::null_mut(),
                method: None,
                getter: Some(generic_getter),
                setter: Some(generic_setter),
                value: std::ptr::null_mut(),
                attributes: (PropertyAttributes::Default
                    | PropertyAttributes::Writable
                    | PropertyAttributes::Enumerable)
                    .bits(),
                data: field_name.as_ptr() as *mut std::ffi::c_void,
            })
        });

        const_method_names.iter().for_each(|method_name| {
            properties.push(napi_sys::napi_property_descriptor {
                utf8name: method_name.as_ptr(),
                name: std::ptr::null_mut(),
                method: Some(generic_const_method),
                getter: None,
                setter: None,
                value: std::ptr::null_mut(),
                attributes: (PropertyAttributes::Default
                    | PropertyAttributes::Writable
                    | PropertyAttributes::Enumerable)
                    .bits(),
                data: method_name.as_ptr() as *mut std::ffi::c_void,
            })
        });

        mut_method_names.iter().for_each(|method_name| {
            properties.push(napi_sys::napi_property_descriptor {
                utf8name: method_name.as_ptr(),
                name: std::ptr::null_mut(),
                method: Some(generic_mut_method),
                getter: None,
                setter: None,
                value: std::ptr::null_mut(),
                attributes: (PropertyAttributes::Default
                    | PropertyAttributes::Writable
                    | PropertyAttributes::Enumerable)
                    .bits(),
                data: method_name.as_ptr() as *mut std::ffi::c_void,
            })
        });

        // Create class
        napi::check_status!(
            napi_sys::napi_define_class(
                env,
                js_class_name.as_ptr(),
                class_name.len() - 1,
                Some(raw_constructor),
                std::ptr::null_mut(),
                properties.len(),
                properties.as_ptr(),
                &mut class_ptr,
            ),
            "Failed to register class `{}`",
            &class_name,
        )?;

        napi::check_status!(
            napi_sys::napi_create_reference(env, class_ptr, 1, &mut ctor_ref,),
            "Failed to create constructor reference of class `{}`",
            &class_name,
        )?;

        // Add a finalize to store the member names
        let mut maybe_ref = std::ptr::null_mut();
        let wrap_context = Box::leak(Box::new(FinalizeMemberNamesData {
            _field_names: field_names,
            _const_method_names: const_method_names,
            _mut_method_names: mut_method_names,
        }));
        check_status!(unsafe {
            napi_sys::napi_add_finalizer(
                env,
                class_ptr,
                wrap_context as *mut _ as *mut std::ffi::c_void,
                Some(raw_finalize_member_names_unchecked),
                std::ptr::null_mut(),
                &mut maybe_ref,
            )
        })?;

        // Store the class for the next instantiations
        self.0.borrow_mut().insert(class_name.clone(), ctor_ref);

        Ok(ctor_ref)
    }
}

/// # Safety
///
/// called when node wrapper objects destroyed
#[doc(hidden)]
pub unsafe extern "C" fn raw_constructor(
    env: napi_sys::napi_env,
    _callback_info: napi_sys::napi_callback_info,
) -> napi_sys::napi_value {
    let mut undefined = std::ptr::null_mut();
    unsafe { napi_sys::napi_get_undefined(env, &mut undefined) };
    undefined
}

/// # Safety
///
/// called when node wrapper objects destroyed
#[doc(hidden)]
pub unsafe extern "C" fn raw_finalize_wrapped_object_unchecked(
    _env: napi_sys::napi_env,
    finalize_data: *mut std::ffi::c_void,
    _finalize_hint: *mut std::ffi::c_void,
) {
    let data = *unsafe { Box::from_raw(finalize_data as *mut Box<dyn ScriptObject>) };
    drop(data);
}

struct FinalizeMemberNamesData {
    _field_names: Vec<CString>,
    _const_method_names: Vec<CString>,
    _mut_method_names: Vec<CString>,
}

/// # Safety
///
/// called when node wrapper objects destroyed
#[doc(hidden)]
pub unsafe extern "C" fn raw_finalize_member_names_unchecked(
    _env: napi_sys::napi_env,
    finalize_data: *mut std::ffi::c_void,
    _finalize_hint: *mut std::ffi::c_void,
) {
    let data = *unsafe { Box::from_raw(finalize_data as *mut FinalizeMemberNamesData) };
    drop(data);
}

unsafe extern "C" fn generic_getter(
    raw_env: napi_sys::napi_env,
    callback_info: napi_sys::napi_callback_info,
) -> napi_sys::napi_value {
    unsafe fn generic_getter(
        raw_env: napi_sys::napi_env,
        callback_info: napi_sys::napi_callback_info,
    ) -> napi::Result<napi_sys::napi_value> {
        // Get the callback infos
        let mut this = std::ptr::null_mut();
        let mut args = [std::ptr::null_mut(); 0];
        let mut argc = 0;
        let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

        check_status!(napi_sys::napi_get_cb_info(
            raw_env,
            callback_info,
            &mut argc,
            args.as_mut_ptr(),
            &mut this,
            &mut data_ptr,
        ))?;

        let data_ptr = std::ffi::CStr::from_ptr(data_ptr as *mut std::ffi::c_char);
        let field_name = data_ptr.to_str().unwrap().to_string();

        // Initialize javascript utils
        let env = Env::from_raw(raw_env);

        // Get the wrapped object
        let mut wrapped = std::ptr::null_mut();
        check_status!(
            unsafe { napi_sys::napi_unwrap(raw_env, this, &mut wrapped) },
            "Unwrap value [{}] from class failed",
            std::any::type_name::<Box<dyn ScriptObject>>(),
        )?;
        let wrapped = wrapped as *mut Box<dyn ScriptObject>;
        let wrapped = wrapped.as_mut().unwrap();

        // Execute the getter
        let result = wrapped
            .get_field_value(&field_name.to_case(Case::Snake))
            .map_err(|e| e.into_napi())?;

        // Returns the result
        let result = script_value_to_js_value(&env, result).map_err(|e| e.into_napi())?;
        Ok(result.raw())
    }

    generic_getter(raw_env, callback_info).unwrap_or_else(|e| {
        unsafe { JsError::from(e).throw_into(raw_env) };
        std::ptr::null_mut()
    })
}

unsafe extern "C" fn generic_setter(
    raw_env: napi_sys::napi_env,
    callback_info: napi_sys::napi_callback_info,
) -> napi_sys::napi_value {
    unsafe fn generic_setter(
        raw_env: napi_sys::napi_env,
        callback_info: napi_sys::napi_callback_info,
    ) -> napi::Result<napi_sys::napi_value> {
        // Get the callback infos
        let mut this = std::ptr::null_mut();
        let mut args = [std::ptr::null_mut(); 1];
        let mut argc = 1;
        let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

        check_status!(napi_sys::napi_get_cb_info(
            raw_env,
            callback_info,
            &mut argc,
            args.as_mut_ptr(),
            &mut this,
            &mut data_ptr,
        ))?;

        let data_ptr = std::ffi::CStr::from_ptr(data_ptr as *mut std::ffi::c_char);
        let field_name = data_ptr.to_str().unwrap().to_string();

        // Initialize javascript utils
        let env = Env::from_raw(raw_env);

        // Get the wrapped object
        let mut wrapped = std::ptr::null_mut();
        check_status!(
            unsafe { napi_sys::napi_unwrap(raw_env, this, &mut wrapped) },
            "Unwrap value [{}] from class failed",
            std::any::type_name::<Box<dyn ScriptObject>>(),
        )?;
        let wrapped = wrapped as *mut Box<dyn ScriptObject>;
        let wrapped = wrapped.as_mut().unwrap();

        // Execute the setter
        let arg = JsUnknown::from_raw(raw_env, args[0])?;
        let arg = js_value_to_script_value(&env, arg).map_err(|e| e.into_napi())?;
        wrapped
            .set_field_value(&field_name.to_case(Case::Snake), arg)
            .map_err(|e| e.into_napi())?;

        // Returns the result
        let result = env.get_undefined()?;
        Ok(result.raw())
    }

    generic_setter(raw_env, callback_info).unwrap_or_else(|e| {
        unsafe { JsError::from(e).throw_into(raw_env) };
        std::ptr::null_mut()
    })
}

unsafe extern "C" fn generic_const_method(
    raw_env: napi_sys::napi_env,
    callback_info: napi_sys::napi_callback_info,
) -> napi_sys::napi_value {
    unsafe fn generic_const_method(
        raw_env: napi_sys::napi_env,
        callback_info: napi_sys::napi_callback_info,
    ) -> napi::Result<napi_sys::napi_value> {
        // Get the callback infos
        let mut this = std::ptr::null_mut();
        let mut args = [std::ptr::null_mut(); 6];
        let mut argc = 6;
        let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

        check_status!(napi_sys::napi_get_cb_info(
            raw_env,
            callback_info,
            &mut argc,
            args.as_mut_ptr(),
            &mut this,
            &mut data_ptr,
        ))?;

        let data_ptr = std::ffi::CStr::from_ptr(data_ptr as *mut std::ffi::c_char);
        let method_name = data_ptr.to_str().unwrap().to_string();

        // Initialize javascript utils
        let env = Env::from_raw(raw_env);

        // Get the wrapped object
        let mut wrapped = std::ptr::null_mut();
        check_status!(
            unsafe { napi_sys::napi_unwrap(raw_env, this, &mut wrapped) },
            "Unwrap value [{}] from class failed",
            std::any::type_name::<Box<dyn ScriptObject>>(),
        )?;
        let wrapped = wrapped as *mut Box<dyn ScriptObject>;
        let wrapped = wrapped.as_mut().unwrap();

        // Execute the setter
        let args = args
            .iter()
            .map(|arg| {
                let arg = JsUnknown::from_raw(raw_env, *arg)?;
                js_value_to_script_value(&env, arg).map_err(|e| e.into_napi())
            })
            .try_collect::<Vec<_>>()?;

        let result = wrapped
            .call_const_method(&method_name.to_case(Case::Snake), args)
            .map_err(|e| e.into_napi())?;

        // Returns the result
        let result = script_value_to_js_value(&env, result).map_err(|e| e.into_napi())?;
        Ok(result.raw())
    }

    generic_const_method(raw_env, callback_info).unwrap_or_else(|e| {
        unsafe { JsError::from(e).throw_into(raw_env) };
        std::ptr::null_mut()
    })
}

unsafe extern "C" fn generic_mut_method(
    raw_env: napi_sys::napi_env,
    callback_info: napi_sys::napi_callback_info,
) -> napi_sys::napi_value {
    unsafe fn generic_mut_method(
        raw_env: napi_sys::napi_env,
        callback_info: napi_sys::napi_callback_info,
    ) -> napi::Result<napi_sys::napi_value> {
        // Get the callback infos
        let mut this = std::ptr::null_mut();
        let mut args = [std::ptr::null_mut(); 6];
        let mut argc = 6;
        let mut data_ptr: *mut std::ffi::c_void = std::ptr::null_mut();

        check_status!(napi_sys::napi_get_cb_info(
            raw_env,
            callback_info,
            &mut argc,
            args.as_mut_ptr(),
            &mut this,
            &mut data_ptr,
        ))?;

        let data_ptr = std::ffi::CStr::from_ptr(data_ptr as *mut std::ffi::c_char);
        let method_name = data_ptr.to_str().unwrap().to_string();

        // Initialize javascript utils
        let env = Env::from_raw(raw_env);

        // Get the wrapped object
        let mut wrapped = std::ptr::null_mut();
        check_status!(
            unsafe { napi_sys::napi_unwrap(raw_env, this, &mut wrapped) },
            "Unwrap value [{}] from class failed",
            std::any::type_name::<Box<dyn ScriptObject>>(),
        )?;
        let wrapped = wrapped as *mut Box<dyn ScriptObject>;
        let wrapped = wrapped.as_mut().unwrap();

        // Execute the setter
        let args = args
            .iter()
            .map(|arg| {
                let arg = JsUnknown::from_raw(raw_env, *arg)?;
                js_value_to_script_value(&env, arg).map_err(|e| e.into_napi())
            })
            .try_collect::<Vec<_>>()?;

        let result = wrapped
            .call_mut_method(&method_name.to_case(Case::Snake), args)
            .map_err(|e| e.into_napi())?;

        // Returns the result
        let result = script_value_to_js_value(&env, result).map_err(|e| e.into_napi())?;
        Ok(result.raw())
    }

    generic_mut_method(raw_env, callback_info).unwrap_or_else(|e| {
        unsafe { JsError::from(e).throw_into(raw_env) };
        std::ptr::null_mut()
    })
}
