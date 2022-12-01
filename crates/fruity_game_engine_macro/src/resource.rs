use crate::utils::current_crate;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn intern_derive_resource(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let current_crate = current_crate();

    let output = quote! {
        impl #impl_generics #current_crate::resource::Resource for #ident #ty_generics #where_clause {
            fn as_resource_box(self: Box<Self>) -> Box<dyn #current_crate::resource::Resource> {
                self
            }

            fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
              self
            }
        }
    };

    output.into()
}
