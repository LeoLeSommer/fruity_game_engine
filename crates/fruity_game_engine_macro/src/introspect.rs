use crate::fruity_crate;
use convert_case::{Case, Casing};
use fruity_game_engine_code_parser::{parse_attrs_items, FruityExportFn};
use fruity_game_engine_code_parser::{
    parse_enum_item, parse_impl_item, parse_struct_item, FruityExportClassFieldName,
    FruityExportReceiver,
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_quote, Attribute, Item};
use syn::{ItemEnum, ItemImpl, ItemStruct, __private::TokenStream2};

#[cfg(feature = "wasm-platform")]
use crate::wasm_function_export::wasm_function_export;

#[cfg(not(feature = "wasm-platform"))]
use crate::napi_function_export;

pub fn intern_export_struct(attr: TokenStream, item: ItemStruct) -> TokenStream2 {
    let fruity_crate = fruity_crate();

    // Parse the attrs
    let attr: TokenStream2 = attr.into();
    let attr: Attribute = parse_quote! {
        #[export_struct(#attr)]
    };

    let parsed_attr = parse_attrs_items(&vec![attr]);

    // Parse the block
    let exported_struct = parse_struct_item(item.clone());

    // Prepare the infos
    let exported_fields = exported_struct
        .fields
        .clone()
        .into_iter()
        .filter(|field| field.public)
        .collect::<Vec<_>>();

    let struct_name = item.ident.clone();
    let struct_name_as_string = struct_name.to_string();

    // Implement the IntrospectFields functions
    let impl_get_class_name = quote! {
        fn get_class_name(&self) -> #fruity_crate::FruityResult<String> {
            Ok(#struct_name_as_string.to_string())
        }
    };

    let impl_get_field_names = {
        let fields_names = exported_fields.iter().map(|field| match &field.name {
            FruityExportClassFieldName::Named(name) => name.to_string(),
            FruityExportClassFieldName::Unnamed(name) => name.to_string(),
        });

        quote! {
            fn get_field_names(&self) -> #fruity_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#fields_names.to_string(),)*])
            }
        }
    };

    let impl_set_field_value = {
        let fields_setters = exported_fields.iter().map(|field| match &field.name {
            FruityExportClassFieldName::Named(name) => {
                let name_as_string = name.to_string();
                let ty = field.ty.clone();

                quote! {
                    #name_as_string => self.#name = <#ty>::from_script_value(value)?,
                }
            }
            FruityExportClassFieldName::Unnamed(name) => {
                let name_as_string = name.to_string();
                let name = syn::Index::from(*name);
                let ty = field.ty.clone();

                quote! {
                    #name_as_string => self.#name = <#ty>::from_script_value(value)?,
                }
            }
        });

        if fields_setters.len() > 0 {
            quote! {
                fn set_field_value(&mut self, name: &str, value: #fruity_crate::script_value::ScriptValue) -> #fruity_crate::FruityResult<()> {
                    use #fruity_crate::script_value::TryFromScriptValue;

                    match name {
                        #(#fields_setters)*
                        _ => unreachable!(),
                    };

                    #fruity_crate::FruityResult::Ok(())
                }
            }
        } else {
            quote! {
                fn set_field_value(&mut self, name: &str, value: #fruity_crate::script_value::ScriptValue) -> #fruity_crate::FruityResult<()> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_field_value = {
        let fields_getters = exported_fields.iter().map(|field| match &field.name {
            FruityExportClassFieldName::Named(name) => {
                let name_as_string = name.to_string();
                let ty = field.ty.clone();

                quote! {
                    #name_as_string => <#ty>::into_script_value(self.#name.clone()),
                }
            }
            FruityExportClassFieldName::Unnamed(name) => {
                let name_as_string = name.to_string();
                let name = syn::Index::from(*name);
                let ty = field.ty.clone();

                quote! {
                    #name_as_string => <#ty>::into_script_value(self.#name.clone()),
                }
            }
        });

        if fields_getters.len() > 0 {
            quote! {
                fn get_field_value(&self, name: &str) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                    use #fruity_crate::script_value::TryIntoScriptValue;

                    match name {
                        #(#fields_getters)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn get_field_value(&self, name: &str) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    let impl_from_script_value = if parsed_attr.from_raw_js_object {
        let fields_initializer = exported_fields.iter().map(|field| match &field.name {
            FruityExportClassFieldName::Named(name) => {
                let name_as_string = name.to_string();
                let ty = field.ty.clone();

                quote! {
                    #name: <#ty>::from_script_value(value.get_field_value(#name_as_string)?)?,
                }
            }
            FruityExportClassFieldName::Unnamed(name) => {
                let name_as_string = name.to_string();
                let ty = field.ty.clone();

                quote! {
                    #name: <#ty>::from_script_value(value.get_field_value(#name_as_string)?)?,
                }
            }
        });

        quote! {
            impl #fruity_crate::script_value::TryFromScriptValue for #struct_name
            {
                fn from_script_value(value: #fruity_crate::script_value::ScriptValue) -> #fruity_crate::FruityResult<Self> {
                    use std::ops::Deref;

                    match value {
                        #fruity_crate::script_value::ScriptValue::Object(value) => match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => {
                                Ok(Self {
                                    #(#fields_initializer)*
                                })
                            },
                        },
                        value => Err(#fruity_crate::FruityError::InvalidArg(format!(
                            "Couldn't convert {:?} to native object",
                            value
                        ))),
                    }
                }
            }
        }
    } else {
        quote! {
            impl #fruity_crate::script_value::TryFromScriptValue for #struct_name
            {
                fn from_script_value(value: #fruity_crate::script_value::ScriptValue) -> #fruity_crate::FruityResult<Self> {
                    use std::ops::Deref;

                    match value {
                        #fruity_crate::script_value::ScriptValue::Object(value) => match value.downcast::<Self>() {
                            Ok(value) => Ok(*value),
                            Err(value) => Err(#fruity_crate::FruityError::InvalidArg(format!(
                                "Couldn't convert a {} to {}",
                                value.deref().get_type_name(),
                                std::any::type_name::<#struct_name>()
                            ))),
                        },
                        value => Err(#fruity_crate::FruityError::InvalidArg(format!(
                            "Couldn't convert {:?} to native object",
                            value
                        ))),
                    }
                }
            }
        }
    };

    quote! {
        impl #fruity_crate::introspect::IntrospectFields for #struct_name
        {
            fn is_static(&self) -> #fruity_crate::FruityResult<bool> {
                Ok(true)
            }

            #impl_get_class_name
            #impl_get_field_names
            #impl_set_field_value
            #impl_get_field_value
        }

        impl #fruity_crate::script_value::TryIntoScriptValue for #struct_name {
            fn into_script_value(self) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                Ok(#fruity_crate::script_value::ScriptValue::Object(Box::new(self)))
            }
        }

        #impl_from_script_value
    }
}

pub(crate) fn intern_export_impl(item: ItemImpl) -> TokenStream2 {
    let fruity_crate = fruity_crate();

    // Parse the block
    let exported_struct = parse_impl_item(item.clone());
    let struct_name = item.self_ty.clone();

    // Prepare the infos
    let exported_const_methods = exported_struct
        .methods
        .clone()
        .into_iter()
        .filter(|method| matches!(method.receiver, FruityExportReceiver::Const))
        .collect::<Vec<_>>();

    let exported_mut_methods = exported_struct
        .methods
        .clone()
        .into_iter()
        .filter(|method| matches!(method.receiver, FruityExportReceiver::Mut))
        .collect::<Vec<_>>();

    // Implement the IntrospectMethods functions
    let impl_get_const_method_names = {
        let method_names = exported_const_methods.iter().map(|method| {
            method
                .name_overwrite
                .clone()
                .unwrap_or(method.name.clone())
                .to_string()
        });

        quote! {
            fn get_const_method_names(&self) -> #fruity_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#method_names.to_string(),)*])
            }
        }
    };

    let impl_call_const_method = {
        let method_callers = exported_const_methods.iter().map(|method| {
            let name = method.name.clone();
            let export_function_name = method
                .name_overwrite
                .clone()
                .unwrap_or(method.name.clone())
                .to_string();

            let type_cast = match method.args.len() {
                0 => None,
                _ => {
                    let args_cast = method
                        .args
                        .iter()
                        .enumerate()
                        .map(|(index, arg)| {
                            let ident =
                                syn::Ident::new(&format!("__arg_{}", index), Span::call_site());
                            let ty = arg.ty.clone();

                            quote! {
                                let #ident = __caster.cast_next::<#ty>()?;
                            }
                        })
                        .collect::<Vec<_>>();

                    Some(quote! {
                        let mut __caster = #fruity_crate::utils::ArgumentCaster::new(__args);
                        #(#args_cast)*
                    })
                }
            };
            let arg_names = method.args.iter().enumerate().map(|(index, _arg)| {
                syn::Ident::new(&format!("__arg_{}", index), Span::call_site())
            });

            quote! {
                #export_function_name => {
                    #type_cast
                    self.#name(#(#arg_names),*).into_script_value()
                },
            }
        });

        if method_callers.len() > 0 {
            quote! {
                fn call_const_method(&self, name: &str, __args: Vec<#fruity_crate::script_value::ScriptValue>) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                    use #fruity_crate::script_value::TryIntoScriptValue;

                    match name {
                        #(#method_callers)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn call_const_method(&self, name: &str, __args: Vec<#fruity_crate::script_value::ScriptValue>) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_mut_method_names = {
        let method_names = exported_mut_methods.iter().map(|method| {
            method
                .name_overwrite
                .clone()
                .unwrap_or(method.name.clone())
                .to_string()
        });

        quote! {
            fn get_mut_method_names(&self) -> #fruity_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#method_names.to_string(),)*])
            }
        }
    };

    let impl_call_mut_method = {
        let method_callers = exported_mut_methods
            .iter()
            .map(|method| {
                let name = method.name.clone();
                let export_function_name = method
                    .name_overwrite
                    .clone()
                    .unwrap_or(method.name.clone())
                    .to_string();

                let type_cast = match method.args.len() {
                    0 => None,
                    _ => {
                        let args_cast = method
                            .args
                            .iter()
                            .enumerate()
                            .map(|(index, arg)| {
                                let ident =
                                    syn::Ident::new(&format!("__arg_{}", index), Span::call_site());
                                let ty = arg.ty.clone();

                                quote! {
                                    let #ident = __caster.cast_next::<#ty>()?;
                                }
                            })
                            .collect::<Vec<_>>();

                        Some(quote! {
                            let mut __caster = #fruity_crate::utils::ArgumentCaster::new(__args);
                            #(#args_cast)*
                        })
                    }
                };
                let arg_names = method.args.iter().enumerate().map(|(index, _arg)| {
                    syn::Ident::new(&format!("__arg_{}", index), Span::call_site())
                });

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
                fn call_mut_method(&mut self, name: &str, __args: Vec<#fruity_crate::script_value::ScriptValue>) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                    use #fruity_crate::script_value::TryIntoScriptValue;

                    match name {
                        #(#method_callers)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn call_mut_method(&mut self, name: &str, __args: Vec<#fruity_crate::script_value::ScriptValue>) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    #[cfg(feature = "wasm-platform")]
    let napi_constructor_bindings = quote! {};

    #[cfg(not(feature = "wasm-platform"))]
    let napi_constructor_bindings = {
        let napi_function_exports = exported_struct.clone().constructor.map(|constructor| {
            let ident = constructor.name.get_ident().unwrap();

            let constructor_binding = napi_function_export(
                FruityExportFn {
                    name:
                        syn::Path {
                            leading_colon: None,
                            segments: syn::punctuated::Punctuated::<
                                syn::PathSegment,
                                syn::token::Colon2,
                            >::from_iter(vec![
                                syn::PathSegment::from(exported_struct.name.clone()),
                                syn::PathSegment::from(ident.clone()),
                            ]),
                        },
                    name_overwrite: Some(exported_struct.name.clone()),
                    attrs: constructor.attrs,
                    args: constructor.args,
                    return_ty: constructor.return_ty,
                    is_async: constructor.is_async,
                    typescript_overwrite: constructor.typescript_overwrite,
                },
                None,
            );

            let constructor_get_type_ident = syn::Ident::new(
                &format!("{}_getType", exported_struct.name.to_string()),
                Span::call_site(),
            );

            let constructor_get_type_function = quote! {
                pub fn #constructor_get_type_ident() -> String {
                    struct TransmutedTypeId {
                        t: u64,
                    }

                    let type_id_value = std::any::TypeId::of::<#struct_name>();
                    let type_id_value = unsafe {
                        std::mem::transmute::<
                            std::any::TypeId,
                            TransmutedTypeId,
                        >(type_id_value)
                    }
                        .t;

                    type_id_value.to_string()
                }
            };

            let constructor_get_type_binding = napi_function_export(
                FruityExportFn {
                    name:
                        syn::Path {
                            leading_colon: None,
                            segments: syn::punctuated::Punctuated::<
                                syn::PathSegment,
                                syn::token::Colon2,
                            >::from_iter(vec![
                                syn::PathSegment::from(constructor_get_type_ident.clone()),
                            ]),
                        },
                    name_overwrite: None,
                    attrs: vec![],
                    args: vec![],
                    return_ty: syn::ReturnType::Type(
                        Default::default(),
                        Box::new(syn::Type::Path(syn::TypePath {
                            qself: None,
                            path: syn::Path {
                                leading_colon: None,
                                segments: syn::punctuated::Punctuated::<
                                    syn::PathSegment,
                                    syn::token::Colon2,
                                >::from_iter(vec![
                                    syn::PathSegment::from(syn::Ident::new(
                                        "String",
                                        Span::call_site(),
                                    )),
                                ]),
                            },
                        })),
                    ),
                    is_async: false,
                    typescript_overwrite: None,
                },
                None,
            );

            quote! {
                #constructor_binding
                #constructor_get_type_function
                #constructor_get_type_binding
            }
        });

        quote! {
            #napi_function_exports
        }
    };

    #[cfg(not(feature = "wasm-platform"))]
    let wasm_constructor_bindings = quote! {};

    #[cfg(feature = "wasm-platform")]
    let wasm_constructor_bindings = {
        let wasm_function_exports = exported_struct.clone().constructor.map(|constructor| {
            let ident = constructor.name.get_ident().unwrap();

            let constructor_binding = wasm_function_export(
                FruityExportFn {
                    name:
                        syn::Path {
                            leading_colon: None,
                            segments: syn::punctuated::Punctuated::<
                                syn::PathSegment,
                                syn::token::Colon2,
                            >::from_iter(vec![
                                syn::PathSegment::from(exported_struct.name.clone()),
                                syn::PathSegment::from(ident.clone()),
                            ]),
                        },
                    name_overwrite: Some(exported_struct.name.clone()),
                    attrs: constructor.attrs,
                    args: constructor.args,
                    return_ty: constructor.return_ty,
                    is_async: constructor.is_async,
                    typescript_overwrite: constructor.typescript_overwrite,
                },
                None,
            );

            let constructor_get_type_ident = syn::Ident::new(
                &format!("{}_getType", exported_struct.name.to_string()),
                Span::call_site(),
            );

            let constructor_get_type_function = quote! {
                pub fn #constructor_get_type_ident() -> String {
                    struct TransmutedTypeId {
                        t: u64,
                    }

                    let type_id_value = std::any::TypeId::of::<#struct_name>();
                    let type_id_value = unsafe {
                        std::mem::transmute::<
                            std::any::TypeId,
                            TransmutedTypeId,
                        >(type_id_value)
                    }
                        .t;

                    type_id_value.to_string()
                }
            };

            let constructor_get_type_binding = wasm_function_export(
                FruityExportFn {
                    name:
                        syn::Path {
                            leading_colon: None,
                            segments: syn::punctuated::Punctuated::<
                                syn::PathSegment,
                                syn::token::Colon2,
                            >::from_iter(vec![
                                syn::PathSegment::from(constructor_get_type_ident.clone()),
                            ]),
                        },
                    name_overwrite: None,
                    attrs: vec![],
                    args: vec![],
                    return_ty: syn::ReturnType::Type(
                        Default::default(),
                        Box::new(syn::Type::Path(syn::TypePath {
                            qself: None,
                            path: syn::Path {
                                leading_colon: None,
                                segments: syn::punctuated::Punctuated::<
                                    syn::PathSegment,
                                    syn::token::Colon2,
                                >::from_iter(vec![
                                    syn::PathSegment::from(syn::Ident::new(
                                        "String",
                                        Span::call_site(),
                                    )),
                                ]),
                            },
                        })),
                    ),
                    is_async: false,
                    typescript_overwrite: None,
                },
                None,
            );

            quote! {
                #constructor_binding
                #constructor_get_type_function
                #constructor_get_type_binding
            }
        });

        quote! {
            #wasm_function_exports
        }
    };

    quote! {
        impl #fruity_crate::introspect::IntrospectMethods for #struct_name
        {
            #impl_get_const_method_names
            #impl_call_const_method
            #impl_get_mut_method_names
            #impl_call_mut_method
        }

        #napi_constructor_bindings
        #wasm_constructor_bindings
    }
}

pub(crate) fn intern_export_enum(item: ItemEnum) -> TokenStream2 {
    let fruity_crate = fruity_crate();

    // Parse the block
    let exported_enum = parse_enum_item(item.clone());
    let name = exported_enum.name.clone();
    let name_as_string = name.to_string();

    // Generate TryFromScriptValue converters
    let from_script_value_converters = exported_enum.variants.iter().map(|variant| {
        let variant_str = variant.to_string().to_case(Case::Camel);

        quote! {
            #variant_str => Ok(#name::#variant),
        }
    });

    // Generate TryIntoScriptValue converters
    let into_script_value_converters = exported_enum.variants.iter().map(|variant| {
        let variant_str = variant.to_string().to_case(Case::Camel);

        quote! {
            #name::#variant => #variant_str,
        }
    });

    // Prepare the infos
    quote! {
        impl #fruity_crate::script_value::TryFromScriptValue for #name {
            fn from_script_value(
                value: #fruity_crate::script_value::ScriptValue,
            ) -> #fruity_crate::error::FruityResult<Self> {
                if let #fruity_crate::script_value::ScriptValue::String(value) = &value {
                    match value as &str {
                        #(#from_script_value_converters)*
                        _ => Err(#fruity_crate::error::FruityError::GenericFailure(
                            format!(
                                "Couldn't convert {:?} to {:?}",
                                value, #name_as_string
                            ),
                        )),
                    }
                } else {
                    Err(#fruity_crate::error::FruityError::GenericFailure(
                        format!(
                            "Couldn't convert {:?} to {:?}",
                            value, #name_as_string
                        ),
                    ))
                }
            }
        }

        impl #fruity_crate::script_value::TryIntoScriptValue for #name {
            fn into_script_value(
                self,
            ) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                Ok(#fruity_crate::script_value::ScriptValue::String(
                    match self {
                        #(#into_script_value_converters)*
                    }
                    .to_string(),
                ))
            }
        }
    }
}

pub(crate) fn inner_extern(item: Item) -> TokenStream2 {
    let ident = match item {
        Item::Const(_) => unimplemented!(),
        Item::Enum(item) => item.ident,
        Item::ExternCrate(_) => unimplemented!(),
        Item::Fn(_) => unimplemented!(),
        Item::ForeignMod(_) => unimplemented!(),
        Item::Impl(_) => unimplemented!(),
        Item::Macro(_) => unimplemented!(),
        Item::Macro2(_) => unimplemented!(),
        Item::Mod(_) => unimplemented!(),
        Item::Static(_) => unimplemented!(),
        Item::Struct(item) => item.ident,
        Item::Trait(_) => unimplemented!(),
        Item::TraitAlias(_) => unimplemented!(),
        Item::Type(item) => item.ident,
        Item::Union(_) => unimplemented!(),
        Item::Use(_) => unimplemented!(),
        Item::Verbatim(_) => unimplemented!(),
        _ => unimplemented!(),
    };

    let ident_as_string = ident.to_string();

    let fruity_crate = fruity_crate();

    // Prepare the infos
    quote! {
        impl #fruity_crate::introspect::IntrospectFields for #ident {
            fn is_static(&self) -> #fruity_crate::FruityResult<bool> {
                Ok(false)
            }

            fn get_class_name(&self) -> #fruity_crate::FruityResult<String> {
                Ok(#ident_as_string.to_string())
            }

            fn get_field_names(&self) -> #fruity_crate::FruityResult<Vec<String>> {
                Ok(vec![])
            }

            fn set_field_value(&mut self, name: &str, value: #fruity_crate::script_value::ScriptValue) -> #fruity_crate::FruityResult<()> {
                unreachable!()
            }

            fn get_field_value(&self, name: &str) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                unreachable!()
            }
        }

        impl #fruity_crate::introspect::IntrospectMethods for #ident {
            fn get_const_method_names(&self) -> #fruity_crate::FruityResult<Vec<String>> {
                Ok(vec![])
            }

            fn call_const_method(&self, _name: &str, _args: Vec<#fruity_crate::script_value::ScriptValue>) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                unreachable!()
            }

            fn get_mut_method_names(&self) -> #fruity_crate::FruityResult<Vec<String>> {
                Ok(vec![])
            }

            fn call_mut_method(
                &mut self,
                _name: &str,
                _args: Vec<ScriptValue>,
            ) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                unreachable!()
            }
        }
    }
}
