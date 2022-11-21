use proc_macro2::Span;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{AttrStyle, Attribute, Fields, FnArg, ImplItemMethod, Index, ReturnType};

pub struct ParsedArg {
    pub name: TokenStream2,
    pub ty: TokenStream2,
}

pub enum ParsedReceiver {
    None,
    Const,
    Mut,
}

pub struct ParsedMethod {
    pub attrs: Vec<Attribute>,
    pub name: TokenStream2,
    pub _receiver: ParsedReceiver,
    pub args: Vec<ParsedArg>,
    pub return_ty: Option<TokenStream2>,
}

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

    ParsedMethod {
        attrs,
        name: quote! { #name },
        _receiver: receiver,
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
