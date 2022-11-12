use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(FruityAny)]
pub fn derive_fruity_any(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let fruity_game_engine_crate = if crate_name == "fruity_game_engine" {
        quote! { crate }
    } else {
        quote! { fruity_game_engine }
    };

    let output = quote! {
        impl #impl_generics #fruity_game_engine_crate::any::FruityAny for #ident #ty_generics #where_clause {
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }

            fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
                self
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    let fruity_game_engine_crate = if crate_name == "fruity_game_engine" {
        quote! { crate }
    } else {
        quote! { fruity_game_engine }
    };

    let output = quote! {
        impl #impl_generics #fruity_game_engine_crate::resource::Resource for #ident #ty_generics #where_clause {
            fn as_resource_box(self: Box<Self>) -> Box<dyn #fruity_game_engine_crate::resource::Resource> {
                self
            }
        }
    };

    output.into()
}
