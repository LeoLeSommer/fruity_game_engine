use crate::utils::fruity_crate;
use proc_macro2::Span;
use quote::quote;
use syn::Signature;
use syn::__private::TokenStream2;

pub(crate) fn napi_function_export(
    sig_input: Signature,
    fn_identifier: TokenStream2,
    exported_name: String,
) -> TokenStream2 {
    let fruity_crate = fruity_crate();

    let fn_identifier_c = format!("{}\0", exported_name);
    let fn_identifier_size = fn_identifier_c.len();
    let return_ty = match sig_input.output {
        syn::ReturnType::Default => Box::new(syn::Type::Tuple(syn::TypeTuple {
            paren_token: syn::token::Paren {
                span: Span::call_site(),
            },
            elems: syn::punctuated::Punctuated::new(),
        })),
        syn::ReturnType::Type(_, ty) => ty,
    };

    let napi_func_ident = syn::Ident::new(&format!("__napi__{}", exported_name), Span::call_site());
    let napi_js_func_ident =
        syn::Ident::new(&format!("{}_js_function", exported_name), Span::call_site());
    let napi_register_ident = syn::Ident::new(
        &format!("__napi_register__{}", exported_name),
        Span::call_site(),
    );
    let napi_register_ctor_ident = syn::Ident::new(
        &format!("__napi_register__{}___rust_ctor___ctor", exported_name),
        Span::call_site(),
    );

    let args_names = sig_input
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(input) => Some(input),
        })
        .enumerate()
        .map(|(index, _)| syn::Ident::new(&format!("arg_{}", index), Span::call_site()));

    let args_converters = sig_input
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(input) => Some(input),
        })
        .enumerate()
        .map(|(index, input)| {
            let arg_name = syn::Ident::new(&format!("arg_{}", index), Span::call_site());
            let arg_index = index;
            let ty = input.ty.clone();

            quote! {
                let #arg_name = {
                    let arg = #fruity_crate::javascript::js_value_to_script_value(
                        &env,
                        #fruity_crate::napi::JsUnknown::from_raw(raw_env, cb.get_arg(#arg_index))?,
                    )
                    .map_err(|e| e.into_napi())?;
                    <#ty>::from_script_value(arg).map_err(|e| e.into_napi())?
                };
            }
        });

    quote! {
        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn #napi_func_ident(
            raw_env: #fruity_crate::napi::bindgen_prelude::sys::napi_env,
            cb: #fruity_crate::napi::bindgen_prelude::sys::napi_callback_info,
        ) -> #fruity_crate::napi::bindgen_prelude::sys::napi_value {
            use #fruity_crate::napi::NapiValue;
            use #fruity_crate::script_value::convert::TryFromScriptValue;
            use #fruity_crate::script_value::convert::TryIntoScriptValue;

            unsafe {
                let env = #fruity_crate::napi::Env::from_raw(raw_env);
                #fruity_crate::napi::bindgen_prelude::CallbackInfo::<2usize>::new(raw_env, cb, None)
                    .and_then(|cb| {
                        #(#args_converters)*

                        #fruity_crate::napi::bindgen_prelude::within_runtime_if_available(move || {
                            let _ret = { #fn_identifier(#(#args_names),*) };
                            let _ret = <#return_ty>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = #fruity_crate::javascript::script_value_to_js_value(&env, _ret)
                                .map_err(|e| e.into_napi())?;

                            <#fruity_crate::napi::JsUnknown as #fruity_crate::napi::bindgen_prelude::ToNapiValue>::to_napi_value(raw_env, _ret)
                        })
                    })
                    .unwrap_or_else(|e| {
                        #fruity_crate::napi::bindgen_prelude::JsError::from(e).throw_into(raw_env);
                        std::ptr::null_mut::<#fruity_crate::napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }

        #[doc(hidden)]
        #[allow(dead_code)]
        pub unsafe fn #napi_js_func_ident(
            raw_env: #fruity_crate::napi::bindgen_prelude::sys::napi_env,
        ) -> #fruity_crate::napi::bindgen_prelude::Result<#fruity_crate::napi::bindgen_prelude::sys::napi_value> {
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = #fruity_crate::napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    #fn_identifier_c.as_ptr() as *const _,
                    #fn_identifier_size,
                    Some(#napi_func_ident),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    #fruity_crate::napi::sys::Status::napi_ok => Ok(()),
                    _ => Err(#fruity_crate::napi::Error::new(
                        #fruity_crate::napi::Status::from(c),
                        format!("Failed to register function `{}`", #exported_name),
                    )),
                }
            }?;
            #fruity_crate::napi::bindgen_prelude::register_js_function(
                #fn_identifier_c,
                #napi_js_func_ident,
                Some(#napi_func_ident),
            );
            Ok(fn_ptr)
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        pub extern "C" fn #napi_register_ident() {
            #fruity_crate::napi::bindgen_prelude::register_module_export(
                None,
                #fn_identifier_c,
                #napi_js_func_ident,
            );
        }

        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[no_mangle]
        #[link_section = "__DATA,__mod_init_func"]
        pub static #napi_register_ctor_ident: unsafe extern "C" fn() = {
            unsafe extern "C" fn #napi_register_ctor_ident() {
                #napi_register_ident()
            }
            #napi_register_ctor_ident
        };
    }
}
