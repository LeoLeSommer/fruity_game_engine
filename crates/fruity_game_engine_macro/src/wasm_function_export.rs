use crate::utils::fruity_crate;
use convert_case::{Case, Casing};
use fruity_game_engine_code_parser::FruityExportFn;
use proc_macro2::Span;
use quote::quote;
use syn::__private::TokenStream2;

pub(crate) fn wasm_function_export(
    exported_fn: FruityExportFn,
    case: Option<Case>,
) -> TokenStream2 {
    let fruity_crate = fruity_crate();

    let fn_identifier = exported_fn.name;
    let mut exported_name = exported_fn
        .name_overwrite
        .map(|name_overwrite| name_overwrite.to_string())
        .unwrap_or(quote! {#fn_identifier}.to_string());

    if let Some(case) = case {
        exported_name = exported_name.to_case(case);
    }

    let return_ty = match exported_fn.return_ty {
        syn::ReturnType::Default => Box::new(syn::Type::Tuple(syn::TypeTuple {
            paren_token: syn::token::Paren {
                span: Span::call_site(),
            },
            elems: syn::punctuated::Punctuated::new(),
        })),
        syn::ReturnType::Type(_, ty) => ty,
    };

    let wasm_func_ident = syn::Ident::new(&format!("__wasm__{}", exported_name), Span::call_site());

    let args_names = exported_fn
        .args
        .iter()
        .enumerate()
        .map(|(index, _)| syn::Ident::new(&format!("arg_{}", index), Span::call_site()));

    let function_args = exported_fn.args.iter().enumerate().map(|(index, _)| {
        let arg_name = syn::Ident::new(&format!("arg_{}", index), Span::call_site());

        quote! {
            #arg_name: #fruity_crate::wasm_bindgen::JsValue
        }
    });

    let args_converters = exported_fn.args.iter().enumerate()
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
