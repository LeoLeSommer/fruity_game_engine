extern crate quote;
extern crate syn;

use convert::intern_derive_try_from_script_value;
use convert_case::Case;
use fruity_any::intern_derive_fruity_any;
use introspect::{intern_export_impl, intern_export_struct};
use napi_function_export::napi_function_export;
use proc_macro::{self, TokenStream};
use quote::quote;
use resource::intern_derive_resource;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, ItemImpl, ItemStruct};
use utils::current_crate;

#[cfg(feature = "napi-module")]
use syn::ItemFn;

mod convert;
mod fruity_any;
mod introspect;
mod napi_function_export;
mod parse;
mod resource;
mod utils;

#[proc_macro_derive(FruityAny)]
pub fn derive_fruity_any(input: TokenStream) -> TokenStream {
    intern_derive_fruity_any(input)
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    intern_derive_resource(input)
}

#[proc_macro_derive(TryFromScriptValue)]
pub fn derive_try_from_script_value(input: TokenStream) -> TokenStream {
    intern_derive_try_from_script_value(input)
}

#[proc_macro_attribute]
#[cfg(not(feature = "napi-module"))]
pub fn fruity_module_exports(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.clone().into();

    let output = quote! {
        #[allow(dead_code)]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
#[cfg(feature = "napi-module")]
pub fn fruity_module_exports(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_2 = input.clone();
    let fn_input: ItemFn = parse_macro_input!(input_2);
    let fn_ident = fn_input.sig.ident;
    let current_crate = current_crate();
    let input: TokenStream2 = input.clone().into();

    let output = quote! {
        #input

        #[#current_crate::napi::bindgen_prelude::ctor]
        fn __napi__explicit_module_register() {
            unsafe fn register(raw_env: #current_crate::napi::sys::napi_env, raw_exports: #current_crate::napi::sys::napi_value) -> #current_crate::napi::Result<()> {
                use #current_crate::napi::{Env, JsObject, NapiValue};

                let env = Env::from_raw(raw_env);
                let exports = JsObject::from_raw_unchecked(raw_env, raw_exports);
                let export_js = #current_crate::javascript::ExportJavascript::new(exports, env);

                let result = #fn_ident(export_js);
                result.map_err(|e| #current_crate::FruityError::into_napi(e))
            }

            #current_crate::napi::bindgen_prelude::register_module_exports(register)
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
#[cfg(not(feature = "napi-module"))]
pub fn export_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.clone().into();

    let output = quote! {
        #[allow(dead_code)]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
#[cfg(feature = "napi-module")]
pub fn export_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let fn_input: ItemFn = parse_macro_input!(input2);
    let napi_function_export = napi_function_export(fn_input, Case::Camel);

    let input: TokenStream2 = input.clone().into();
    let output = quote! {
        #input
        #napi_function_export
    };

    output.into()
}

#[proc_macro_attribute]
pub fn export_struct(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let struct_input: ItemStruct = parse_macro_input!(input2);

    let export_struct = intern_export_struct(struct_input);

    let input: TokenStream2 = input.clone().into();
    let output = quote! {
        #input
        #export_struct
    };

    output.into()
}

#[proc_macro_attribute]
pub fn export_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let impl_input: ItemImpl = parse_macro_input!(input2);

    let export_struct = intern_export_impl(impl_input);

    let input: TokenStream2 = input.clone().into();
    let output = quote! {
        #input
        #export_struct
    };

    output.into()
}
