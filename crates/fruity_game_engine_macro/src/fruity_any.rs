use crate::current_crate;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn intern_derive_fruity_any(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let current_crate = current_crate();
    let ident_string = ident.to_string();

    let output = quote! {
        impl #impl_generics #current_crate::any::FruityAny for #ident #ty_generics #where_clause {
            fn get_type_name(&self) -> &'static str {
                #ident_string
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

            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }

            fn as_fruity_any_ref(&self) -> &dyn #current_crate::any::FruityAny {
                self
            }

            fn as_fruity_any_mut(&mut self) -> &mut dyn #current_crate::any::FruityAny {
                self
            }

            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn #current_crate::any::FruityAny> {
                self
            }

            fn as_fruity_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn #current_crate::any::FruityAny> {
                self
            }
        }
    };

    output.into()
}
