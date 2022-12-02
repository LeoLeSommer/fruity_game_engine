use itertools::Itertools;
use proc_macro2::{Ident, Span};
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use std::collections::HashMap;
use syn::__private::TokenStream2;
use syn::{AttrStyle, Fields, FnArg, ImplItemMethod, Index, ReturnType};

#[derive(Clone, Debug)]
pub struct ParsedArg {
    pub name: TokenStream2,
    pub ty: TokenStream2,
}

#[derive(Clone, Debug)]
pub enum ParsedReceiver {
    None,
    Const,
    Mut,
}

#[derive(Clone, Debug)]
pub struct Attribute {
    pub ident: Ident,
    pub params: AttributeParameters,
}

pub type AttributeParameters = HashMap<String, TokenStream>;

#[derive(Clone, Debug)]
pub struct ParsedMethod {
    pub item_impl_method: ImplItemMethod,
    pub attrs: Vec<Attribute>,
    pub name: TokenStream2,
    pub receiver: ParsedReceiver,
    pub args: Vec<ParsedArg>,
    pub return_ty: Option<TokenStream2>,
}

#[derive(Clone, Debug)]
pub struct ParsedField {
    pub public: bool,
    pub name: TokenStream2,
    pub ty: TokenStream2,
}

pub fn parse_impl_method(method: &ImplItemMethod) -> ParsedMethod {
    let attrs = method.attrs.clone();
    let signature = method.sig.clone();

    let name = signature.clone().ident;
    let receiver = match signature.receiver() {
        Some(FnArg::Receiver(receiver)) => match receiver.reference {
            Some(_) => match receiver.mutability {
                Some(_) => ParsedReceiver::Mut,
                None => ParsedReceiver::Const,
            },
            None => ParsedReceiver::None,
        },
        _ => ParsedReceiver::None,
    };

    let args = signature
        .inputs
        .iter()
        .filter_map(|input| match input {
            FnArg::Receiver(_) => None,
            FnArg::Typed(pat_type) => Some(pat_type),
        })
        .enumerate()
        .map(|(index, input)| {
            let ty = input.ty.clone();
            let name_ident = syn::Ident::new(&format!("__arg_{}", index), Span::call_site());

            ParsedArg {
                name: quote! { #name_ident },
                ty: quote! { #ty },
            }
        })
        .collect::<Vec<_>>();

    let return_ty = match signature.output {
        ReturnType::Default => None,
        ReturnType::Type(_, return_ty) => Some(quote! { #return_ty }),
    };

    let attrs = attrs
        .into_iter()
        .filter_map(|attr| {
            attr.clone().path.get_ident().map(|attr_ident| Attribute {
                ident: attr_ident.clone(),
                params: attr
                    .clone()
                    .tokens
                    .into_iter()
                    .filter_map(|e| match e {
                        TokenTree::Group(group) => Some(group),
                        _ => None,
                    })
                    .find(|_| true)
                    .map(|e| e.stream())
                    .map(|e| {
                        e.into_iter()
                            .enumerate()
                            .group_by(|(index, _)| index / 3)
                            .into_iter()
                            .map(|(_, metas)| {
                                metas
                                    .into_iter()
                                    .map(|(_, metas)| metas.into())
                                    .collect::<Vec<TokenStream>>()
                            })
                            .filter(|metas| metas.len() == 3)
                            .filter(|metas| metas[1].to_string() == "=")
                            .map(|metas| (metas[0].to_string(), metas[2].clone()))
                            .collect::<HashMap<String, TokenStream>>()
                    })
                    .unwrap_or(HashMap::new()),
            })
        })
        .collect_vec();

    /*let attrs = attr_params
    .*/

    ParsedMethod {
        item_impl_method: method.clone(),
        attrs,
        name: quote! { #name },
        receiver: receiver,
        args: args,
        return_ty,
    }
}

pub fn parse_struct_fields(fields: &Fields) -> Vec<ParsedField> {
    match fields {
        Fields::Named(ref fields) => fields
            .named
            .iter()
            .filter(|field| {
                matches!(
                    field.attrs.iter().find(|attr| {
                        matches!(attr.style, AttrStyle::Outer)
                            && attr.path.segments.len() == 1
                            && attr.path.segments[0].ident.to_string() == "native_only"
                    }),
                    None
                )
            })
            .map(|field| {
                let ty = &field.ty;
                match &field.ident {
                    Some(ident) => ParsedField {
                        public: if let syn::Visibility::Public(_) = field.vis {
                            true
                        } else {
                            false
                        },
                        name: quote! { #ident },
                        ty: quote! { #ty },
                    },
                    None => unimplemented!(),
                }
            })
            .collect(),
        Fields::Unnamed(ref fields) => {
            // For tuple struct, field name are numbers
            fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| {
                    let ty = &field.ty;
                    let index = Index::from(index);

                    ParsedField {
                        public: if let syn::Visibility::Public(_) = field.vis {
                            true
                        } else {
                            false
                        },
                        name: quote! { #index },
                        ty: quote! { #ty },
                    }
                })
                .collect()
        }
        Fields::Unit => {
            unimplemented!()
        }
    }
}
