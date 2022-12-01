use proc_macro::{TokenStream};
use proc_macro2::Span;
use syn::{__private::TokenStream2, Stmt};
use quote::quote;
use crate::{parse::{ParsedField, ParsedMethod, ParsedReceiver, parse_struct_fields, parse_impl_method}, current_crate};

pub(crate) fn impl_introspect_object(input: TokenStream) -> TokenStream2 {
    let current_crate = current_crate();

    let input_2: TokenStream2 = input.into();
    let braced_input = quote! {
        {
            #input_2
        }
    };

    let item = syn::parse2::<syn::Block>(braced_input.into()).unwrap();

    // Parse the block
    let mut struct_name = syn::Ident::new("unknown", Span::call_site());
            let struct_name_as_string = struct_name.to_string();
            let mut fields: Vec<ParsedField> = Vec::new();

    let mut methods: Vec<ParsedMethod> = Vec::new();
    item.stmts.into_iter().for_each(|stmt| match stmt {
        Stmt::Item(item) => match item {
            syn::Item::Struct(item_struct) => {
                struct_name = item_struct.ident.clone();
                let mut structure_fields = parse_struct_fields(&item_struct.fields);
                fields.append(&mut structure_fields);
            }
            syn::Item::Impl(item_impl) => {
                let mut impl_methods = item_impl
                    .items
                    .into_iter()
                    .filter_map(|item| {
                        if let syn::ImplItem::Method(method) = item {
                            Some(method)
                        } else {
                            None
                        }
                    })
                    .map(|item| parse_impl_method(&item))
                    .collect::<Vec<_>>();

                methods.append(&mut impl_methods);
            }
            _ => {}
        },
        _ => {}
    });

    // Prepare the infos
    let exported_fields = fields
        .clone()
        .into_iter()
        .filter(|field| field.public)
        .collect::<Vec<_>>();

    let exported_const_methods = methods
        .clone()
        .into_iter()
        .filter_map(|method| {
            method.attrs.iter()
                .find(|attr| attr.ident.to_string() == "export")
                .map(|export_attr| (method.clone(), export_attr.clone()))
        })
        .filter(|(method, _)| matches!(method.receiver, ParsedReceiver::Const))
        .collect::<Vec<_>>();

    let exported_mut_methods = methods
        .clone()
        .into_iter()
        .filter_map(|method| {
            method.attrs.iter()
                .find(|attr| attr.ident.to_string() == "export")
                .map(|export_attr| (method.clone(), export_attr.clone()))
        })
        .filter(|(method, _)| matches!(method.receiver, ParsedReceiver::Mut))
        .collect::<Vec<_>>();

    // Implement the IntrospectObject functions
    let impl_get_class_name = quote! {
        fn get_class_name(&self) -> #current_crate::FruityResult<String> {
            Ok(#struct_name_as_string.to_string())
        }
    };

    let impl_get_field_names = {
        let fields_names = exported_fields
            .iter()
            .map(|field| field.name.to_string());

        quote! {
            fn get_field_names(&self) -> #current_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#fields_names.to_string(),)*])
            }
        }
    };

    let impl_set_field_value = {
        let fields_setters = exported_fields
            .iter()
            .map(|field| {
                let name = field.name.clone();
                let name_as_string = field.name.to_string();
                let ty = field.ty.clone();
                
                quote! {
                    #name_as_string => self.#name = <#ty>::from_script_value(value)?,
                }
            });


        if fields_setters.len() > 0 {
            quote! {
                fn set_field_value(&mut self, name: &str, value: #current_crate::script_value::ScriptValue) -> #current_crate::FruityResult<()> {
                    use #current_crate::script_value::convert::TryFromScriptValue;

                    match name {
                        #(#fields_setters)*
                        _ => unreachable!(),
                    };
            
                    #current_crate::FruityResult::Ok(())
                }
            }
        } else {
            quote! {
                fn set_field_value(&mut self, name: &str, value: #current_crate::script_value::ScriptValue) -> #current_crate::FruityResult<()> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_field_value = {
        let fields_getters = exported_fields
            .iter()
            .map(|field| {
                let name = field.name.clone();
                let name_as_string = field.name.to_string();
                let ty = field.ty.clone();
                
                quote! {
                    #name_as_string => <#ty>::into_script_value(self.#name.clone()),
                }
            });


        if fields_getters.len() > 0 {
            quote! {
                fn get_field_value(&self, name: &str) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    use #current_crate::script_value::convert::TryIntoScriptValue;
                
                    match name {
                        #(#fields_getters)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn get_field_value(&self, name: &str) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_const_method_names = {
        let method_names = exported_const_methods
            .iter()
            .map(|(method, export_attr)| {
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
                
                export_function_name
            });

        quote! {
            fn get_const_method_names(&self) -> #current_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#method_names.to_string(),)*])
            }
        }
    };

    let impl_call_const_method = {
        let method_callers = exported_const_methods
            .iter()
            .map(|(method, export_attr)| {
                let name = method.name.clone();
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
            
                let type_cast = match method.args.len() {
                    0 => None,
                    _ => {
                        let args_cast = method.args.iter().map(|arg| {
                            let name = arg.name.clone();
                            let ty = arg.ty.clone();
    
                            quote! {
                                let #name = __caster.cast_next::<#ty>()?;
                            }
                        }).collect::<Vec<_>>();
    
                        Some(
                            quote! {
                                let mut __caster = #current_crate::utils::introspect::ArgumentCaster::new(__args);
                                #(#args_cast)*
                            }
                        )
                    }
                };
                let arg_names = method.args.iter().map(|arg| arg.name.clone());
                
                quote! {
                    #export_function_name => {
                        #type_cast
                        self.#name(#(#arg_names),*).into_script_value()
                    },
                }
            });

        if method_callers.len() > 0 {
            quote! {
                fn call_const_method(&self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    use #current_crate::script_value::convert::TryIntoScriptValue;
    
                    match name {
                        #(#method_callers)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn call_const_method(&self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_mut_method_names = {
        let method_names = exported_mut_methods
            .iter()
            .map(|(method, export_attr)| {
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
                
                export_function_name
            });

        quote! {
            fn get_mut_method_names(&self) -> #current_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#method_names.to_string(),)*])
            }
        }
    };

    let impl_call_mut_method = {
        let method_callers = exported_mut_methods
            .iter()
            .map(|(method, export_attr)| {
                let name = method.name.clone();
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
            
                let type_cast = match method.args.len() {
                    0 => None,
                    _ => {
                        let args_cast = method.args.iter().map(|arg| {
                            let name = arg.name.clone();
                            let ty = arg.ty.clone();
    
                            quote! {
                                let #name = __caster.cast_next::<#ty>()?;
                            }
                        }).collect::<Vec<_>>();
    
                        Some(
                            quote! {
                                let mut __caster = #current_crate::utils::introspect::ArgumentCaster::new(__args);
                                #(#args_cast)*
                            }
                        )
                    }
                };
                let arg_names = method.args.iter().map(|arg| arg.name.clone());
                
                quote! {
                    #export_function_name => {
                        #type_cast
                        self.#name(#(#arg_names),*).into_script_value()
                    },
                }
            })
            .collect::<Vec<_>>();

        if method_callers.len() > 0 {
            quote! {
                fn call_mut_method(&mut self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    use #current_crate::script_value::convert::TryIntoScriptValue;
    
                    match name {
                        #(#method_callers)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn call_mut_method(&mut self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    quote!{
        impl #current_crate::introspect::IntrospectObject for #struct_name
        {
            #impl_get_class_name
            #impl_get_field_names
            #impl_set_field_value
            #impl_get_field_value
            #impl_get_const_method_names
            #impl_call_const_method
            #impl_get_mut_method_names
            #impl_call_mut_method
        }
    }
}