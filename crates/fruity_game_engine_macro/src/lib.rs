extern crate quote;
extern crate syn;

use convert::intern_derive_try_from_script_value;
use fruity_any::intern_derive_fruity_any;
use introspect::{intern_export_impl, intern_export_struct};
use proc_macro::{self, TokenStream};
use quote::quote;
use resource::intern_derive_resource;
use syn::__private::TokenStream2;
use syn::{parse_macro_input, ItemImpl, ItemStruct};
use utils::current_crate;

#[cfg(any(feature = "napi-module", feature = "wasm-module"))]
use syn::ItemFn;

#[cfg(any(feature = "napi-module", feature = "wasm-module"))]
use convert_case::{Case, Casing};

#[cfg(feature = "wasm-module")]
use wasm_function_export::wasm_function_export;

#[cfg(feature = "napi-module")]
use napi_function_export::napi_function_export;

#[cfg(feature = "napi-module")]
use napi_value_export::napi_value_export;

#[cfg(feature = "napi-module")]
mod napi_function_export;

#[cfg(feature = "napi-module")]
mod napi_value_export;

#[cfg(feature = "wasm-module")]
mod wasm_function_export;

mod convert;
mod fruity_any;
mod introspect;
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
pub fn export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn export_constructor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}

#[proc_macro_attribute]
pub fn export_function(_attr: TokenStream, input: TokenStream) -> TokenStream {
    #[cfg(not(feature = "napi-module"))]
    let napi_function_export = quote! {};

    #[cfg(feature = "napi-module")]
    let napi_function_export = {
        let input = input.clone();
        let fn_input: ItemFn = parse_macro_input!(input);
        let ident = fn_input.sig.ident.clone();
        napi_function_export(
            fn_input.sig.clone(),
            quote! {#ident},
            fn_input.sig.ident.to_string().to_case(Case::Camel),
        )
    };

    #[cfg(not(feature = "wasm-module"))]
    let wasm_function_export = quote! {};

    #[cfg(feature = "wasm-module")]
    let wasm_function_export = {
        let input = input.clone();
        let fn_input: ItemFn = parse_macro_input!(input);
        let ident = fn_input.sig.ident.clone();
        wasm_function_export(
            fn_input.sig.clone(),
            quote! {#ident},
            fn_input.sig.ident.to_string().to_case(Case::Camel),
        )
    };

    let input: TokenStream2 = input.clone().into();
    let output = quote! {
        #input
        #napi_function_export
        #wasm_function_export
    };

    output.into()
}

#[proc_macro_attribute]
#[cfg(not(any(feature = "napi-module", feature = "wasm-module")))]
pub fn export_value(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input: TokenStream2 = input.clone().into();

    let output = quote! {
        #[allow(dead_code)]
        #input
    };

    output.into()
}

#[proc_macro_attribute]
#[cfg(feature = "napi-module")]
pub fn export_value(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input2 = input.clone();
    let fn_input: ItemFn = parse_macro_input!(input2);

    let napi_value_export = napi_value_export(fn_input);

    let input: TokenStream2 = input.clone().into();
    let output = quote! {
        #input
        #napi_value_export
    };

    output.into()
}

#[proc_macro_attribute]
#[cfg(not(any(feature = "napi-module", feature = "wasm-module")))]
pub fn export_value(attr: TokenStream, input: TokenStream) -> TokenStream {
    export_function(attr, input)
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
