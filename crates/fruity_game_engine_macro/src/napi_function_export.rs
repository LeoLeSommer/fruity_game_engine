use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::quote;
use syn::ItemFn;
use syn::__private::TokenStream2;

pub(crate) fn napi_function_export(fn_input: ItemFn, case: Case) -> TokenStream2 {
    let fn_ident = fn_input.sig.ident.clone();
    let fn_identifier = fn_input.sig.ident.to_string().to_case(case);
    let fn_identifier_c = format!("{}\0", fn_identifier);
    let fn_identifier_size = fn_identifier_c.len();
    let return_ty = match fn_input.sig.output {
        syn::ReturnType::Default => Box::new(syn::Type::Tuple(syn::TypeTuple {
            paren_token: syn::token::Paren {
                span: Span::call_site(),
            },
            elems: syn::punctuated::Punctuated::new(),
        })),
        syn::ReturnType::Type(_, ty) => ty,
    };

    let napi_func_ident = syn::Ident::new(&format!("__napi__{}", fn_identifier), Span::call_site());
    let napi_js_func_ident =
        syn::Ident::new(&format!("{}_js_function", fn_identifier), Span::call_site());
    let napi_register_ident = syn::Ident::new(
        &format!("__napi_register__{}", fn_identifier),
        Span::call_site(),
    );
    let napi_register_ctor_ident = syn::Ident::new(
        &format!("__napi_register__{}___rust_ctor___ctor", fn_identifier),
        Span::call_site(),
    );

    let args_names = fn_input
        .sig
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(input) => Some(input),
        })
        .enumerate()
        .map(|(index, _)| syn::Ident::new(&format!("arg_{}", index), Span::call_site()));

    let args_converters = fn_input
        .sig
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
                    let arg = crate::javascript::js_value_to_script_value(
                        &env,
                        napi::JsUnknown::from_raw(raw_env, cb.get_arg(#arg_index))?,
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
            raw_env: napi::bindgen_prelude::sys::napi_env,
            cb: napi::bindgen_prelude::sys::napi_callback_info,
        ) -> napi::bindgen_prelude::sys::napi_value {
            unsafe {
                let env = napi::Env::from_raw(raw_env);
                napi::bindgen_prelude::CallbackInfo::<2usize>::new(raw_env, cb, None)
                    .and_then(|cb| {
                        #(#args_converters)*

                        napi::bindgen_prelude::within_runtime_if_available(move || {
                            let _ret = { #fn_ident(#(#args_names),*) };
                            let _ret = <#return_ty>::into_script_value(_ret)
                                .map_err(|e| e.into_napi())?;
                            let _ret = crate::javascript::script_value_to_js_value(&env, _ret)
                                .map_err(|e| e.into_napi())?;

                            <JsUnknown as napi::bindgen_prelude::ToNapiValue>::to_napi_value(raw_env, _ret)
                        })
                    })
                    .unwrap_or_else(|e| {
                        napi::bindgen_prelude::JsError::from(e).throw_into(raw_env);
                        std::ptr::null_mut::<napi::bindgen_prelude::sys::napi_value__>()
                    })
            }
        }

        #[doc(hidden)]
        #[allow(dead_code)]
        unsafe fn #napi_js_func_ident(
            raw_env: napi::bindgen_prelude::sys::napi_env,
        ) -> napi::bindgen_prelude::Result<napi::bindgen_prelude::sys::napi_value> {
            let mut fn_ptr = std::ptr::null_mut();
            {
                let c = napi::bindgen_prelude::sys::napi_create_function(
                    raw_env,
                    #fn_identifier_c.as_ptr() as *const _,
                    #fn_identifier_size,
                    Some(#napi_func_ident),
                    std::ptr::null_mut(),
                    &mut fn_ptr,
                );
                match c {
                    ::napi::sys::Status::napi_ok => Ok(()),
                    _ => Err(::napi::Error::new(
                        ::napi::Status::from(c),
                        format!("Failed to register function `{}`", #fn_identifier),
                    )),
                }
            }?;
            napi::bindgen_prelude::register_js_function(
                #fn_identifier_c,
                #napi_js_func_ident,
                Some(#napi_func_ident),
            );
            Ok(fn_ptr)
        }

        #[doc(hidden)]
        #[allow(non_snake_case)]
        extern "C" fn #napi_register_ident() {
            napi::bindgen_prelude::register_module_export(
                None,
                #fn_identifier_c,
                #napi_js_func_ident,
            );
        }

        #[used]
        #[allow(non_upper_case_globals)]
        #[allow(non_snake_case)]
        #[doc(hidden)]
        #[link_section = "__DATA,__mod_init_func"]
        static #napi_register_ctor_ident: unsafe extern "C" fn() = {
            unsafe extern "C" fn #napi_register_ctor_ident() {
                #napi_register_ident()
            }
            #napi_register_ctor_ident
        };
    }
}
