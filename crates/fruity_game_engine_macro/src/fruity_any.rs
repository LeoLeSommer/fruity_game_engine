use crate::fruity_crate;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn intern_derive_fruity_any(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let fruity_crate = fruity_crate();

    let output = quote! {
        impl #impl_generics #fruity_crate::any::FruityAny for #ident #ty_generics #where_clause {
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

            fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
                self
            }

            fn as_fruity_any_ref(&self) -> &dyn #fruity_crate::any::FruityAny {
                self
            }

            fn as_fruity_any_mut(&mut self) -> &mut dyn #fruity_crate::any::FruityAny {
                self
            }

            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn #fruity_crate::any::FruityAny> {
                self
            }

            fn as_fruity_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn #fruity_crate::any::FruityAny + Send + Sync> {
                self
            }
        }
    };

    output.into()
}
