use crate::utils::fruity_crate;
use convert_case::{Case, Casing};
use proc_macro2::Span;
use quote::quote;
use syn::{ItemFn, __private::TokenStream2};

pub(crate) fn napi_value_export(fn_input: ItemFn) -> TokenStream2 {
    let fruity_crate = fruity_crate();

    let ident = fn_input.sig.ident;
    let ty = match fn_input.sig.output {
        syn::ReturnType::Default => Box::new(syn::Type::Tuple(syn::TypeTuple {
            paren_token: syn::token::Paren {
                span: Span::call_site(),
            },
            elems: syn::punctuated::Punctuated::new(),
        })),
        syn::ReturnType::Type(_, ty) => ty,
    };

    let script_identifier = ident.to_string().to_case(Case::Camel);
    let script_identifier_c = format!("{}\0", script_identifier);

    let napi_register_default_callback = syn::Ident::new(
        &format!("__register__const____napi_register__{}_callback__", ident),
        Span::call_site(),
    );
    let napi_register_ident =
        syn::Ident::new(&format!("__napi_register__{}", ident), Span::call_site());
    let napi_register_ctor_ident = syn::Ident::new(
        &format!("__napi_register__{}___rust_ctor___ctor", ident),
        Span::call_site(),
    );

    quote! {
        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        unsafe fn #napi_register_default_callback(
            raw_env: #fruity_crate::napi::sys::napi_env,
        ) -> #fruity_crate::napi::Result<#fruity_crate::napi::sys::napi_value> {
            use #fruity_crate::script_value::convert::TryIntoScriptValue;

            let env = #fruity_crate::napi::Env::from_raw(raw_env);
            let ret = <#ty>::into_script_value(#ident())
                .map_err(|e| e.into_napi())?;
            let ret = #fruity_crate::javascript::script_value_to_js_value(&env, ret)
                .map_err(|e| e.into_napi())?;

            <#fruity_crate::napi::JsUnknown as #fruity_crate::napi::bindgen_prelude::ToNapiValue>::to_napi_value(raw_env, ret)
        }

        #[allow(non_snake_case)]
        #[allow(clippy::all)]
        extern "C" fn #napi_register_ident() {
            #fruity_crate::napi::bindgen_prelude::register_module_export(
                None,
                #script_identifier_c,
                #napi_register_default_callback,
            );
        }
        #[used]
        #[allow(non_upper_case_globals)]
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
