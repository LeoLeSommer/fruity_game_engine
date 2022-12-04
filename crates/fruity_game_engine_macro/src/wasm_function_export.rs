use crate::utils::current_crate;
use crate::utils::fruity_crate;
use proc_macro2::Span;
use quote::quote;
use syn::Signature;
use syn::__private::TokenStream2;

pub(crate) fn wasm_function_export(
    sig_input: Signature,
    fn_identifier: TokenStream2,
    exported_name: String,
) -> TokenStream2 {
    let fruity_crate = fruity_crate();
    let current_crate = current_crate();

    let return_ty = match sig_input.output {
        syn::ReturnType::Default => Box::new(syn::Type::Tuple(syn::TypeTuple {
            paren_token: syn::token::Paren {
                span: Span::call_site(),
            },
            elems: syn::punctuated::Punctuated::new(),
        })),
        syn::ReturnType::Type(_, ty) => ty,
    };

    let wasm_func_ident = syn::Ident::new(
        &format!("__wasm_{}__{}", current_crate, sig_input.ident),
        Span::call_site(),
    );

    let args_names = sig_input
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(input) => Some(input),
        })
        .enumerate()
        .map(|(index, _)| syn::Ident::new(&format!("arg_{}", index), Span::call_site()));

    let function_args = sig_input
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(input) => Some(input),
        })
        .enumerate()
        .map(|(index, _)| {
            let arg_name = syn::Ident::new(&format!("arg_{}", index), Span::call_site());

            quote! {
                #arg_name: #fruity_crate::wasm_bindgen::JsValue
            }
        });

    let args_converters = sig_input
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(input) => Some(input),
        })
        .enumerate()
        .map(|(index, input)| {
            let arg_name = syn::Ident::new(&format!("arg_{}", index), Span::call_site());
            let ty = input.ty.clone();

            quote! {
                let #arg_name = {
                    let arg = #fruity_crate::javascript::wasm::js_value_to_script_value(#arg_name).unwrap();
                    <#ty>::from_script_value(arg).unwrap()
                };
            }
        });

    quote! {
        #[#fruity_crate::wasm_bindgen::prelude::wasm_bindgen(js_name = #exported_name)]
        #[allow(missing_docs)]
        pub fn #wasm_func_ident(
            #(#function_args),*
        ) -> Result<#fruity_crate::wasm_bindgen::JsValue, #fruity_crate::wasm_bindgen::JsError> {
            use #fruity_crate::script_value::convert::TryFromScriptValue;
            use #fruity_crate::script_value::convert::TryIntoScriptValue;


            let _ret = {
                #(#args_converters)*

                let _ret = #fn_identifier(#(#args_names),*);
                <#return_ty>::into_script_value(_ret).unwrap()
            };

            #fruity_crate::javascript::wasm::script_value_to_js_value(_ret)
                .map_err(|err| #fruity_crate::wasm_bindgen::JsError::from(err))
        }
    }
}
