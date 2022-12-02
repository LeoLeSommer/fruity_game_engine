extern crate quote;
extern crate syn;

use convert::intern_derive_try_from_script_value;
use convert_case::Case;
use convert_case::Casing;
use fruity_any::intern_derive_fruity_any;
use introspect::{intern_export_impl, intern_export_struct};
use napi_function_export::napi_function_export;
use napi_value_export::napi_value_export;
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
mod napi_value_export;
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
    let ident = fn_input.sig.ident.clone();

    let napi_function_export = napi_function_export(
        fn_input.sig.clone(),
        quote! {#ident},
        fn_input.sig.ident.to_string().to_case(Case::Camel),
    );

    let input: TokenStream2 = input.clone().into();
    let output = quote! {
        #input
        #napi_function_export
    };

    output.into()
}

#[proc_macro_attribute]
#[cfg(not(feature = "napi-module"))]
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
