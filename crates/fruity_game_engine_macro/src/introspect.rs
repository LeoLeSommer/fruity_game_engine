use crate::{fruity_crate};
use convert_case::{Casing, Case};
use fruity_game_engine_code_parser::{parse_struct_item, FruityExportReceiver, parse_impl_item, FruityExportClassFieldName, parse_enum_item, parse_struct_fields, FruityExportClassField};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{ItemStruct, __private::TokenStream2, ItemImpl, DeriveInput, parse_macro_input, ItemEnum};
use fruity_game_engine_code_parser::FruityExportFn;

#[cfg(feature = "wasm-platform")]
use crate::wasm_function_export::wasm_function_export;

#[cfg(not(feature = "wasm-platform"))]
use crate::napi_function_export;

pub fn intern_export_struct(item: ItemStruct) -> TokenStream2 {
    let fruity_crate = fruity_crate();

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
        let fields_setters = exported_fields.iter().map(|field| {
            match &field.name {
                FruityExportClassFieldName::Named(name) => {
                    let name_as_string = name.to_string();
                    let ty = field.ty.clone();
        
                    quote! {
                        #name_as_string => self.#name = <#ty>::from_script_value(value)?,
                    }
                },
                FruityExportClassFieldName::Unnamed(name) => {
                    let name_as_string = name.to_string();
                    let ty = field.ty.clone();
        
                    quote! {
                        #name_as_string => self.#name = <#ty>::from_script_value(value)?,
                    }
                },
            }
        });

        if fields_setters.len() > 0 {
            quote! {
                fn set_field_value(&mut self, name: &str, value: #fruity_crate::script_value::ScriptValue) -> #fruity_crate::FruityResult<()> {
                    use #fruity_crate::script_value::convert::TryFromScriptValue;

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
        let fields_getters = exported_fields.iter().map(|field| {
            match &field.name {
                FruityExportClassFieldName::Named(name) => {
                    let name_as_string = name.to_string();
                    let ty = field.ty.clone();
        
                    quote! {
                        #name_as_string => <#ty>::into_script_value(self.#name.clone()),
                    }
                },
                FruityExportClassFieldName::Unnamed(name) => {
                    let name_as_string = name.to_string();
                    let ty = field.ty.clone();
        
                    quote! {
                        #name_as_string => <#ty>::into_script_value(self.#name.clone()),
                    }
                },
            }
        });

        if fields_getters.len() > 0 {
            quote! {
                fn get_field_value(&self, name: &str) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                    use #fruity_crate::script_value::convert::TryIntoScriptValue;

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

    quote! {
        impl #fruity_crate::introspect::IntrospectFields for #struct_name
        {
            #impl_get_class_name
            #impl_get_field_names
            #impl_set_field_value
            #impl_get_field_value
        }
    }
}

pub(crate) fn intern_export_impl(item: ItemImpl) -> TokenStream2 {
    let fruity_crate = fruity_crate();

    // Parse the block
    let exported_struct = parse_impl_item(item.clone());
    let struct_name = item.self_ty.clone();

    // Prepare the infos
    let exported_const_methods = exported_struct.methods
        .clone()
        .into_iter()
        .filter(|method| matches!(method.receiver, FruityExportReceiver::Const))
        .collect::<Vec<_>>();

    let exported_mut_methods = exported_struct.methods
        .clone()
        .into_iter()
        .filter(|method| matches!(method.receiver, FruityExportReceiver::Mut))
        .collect::<Vec<_>>();

    // Implement the IntrospectMethods functions
    let impl_get_const_method_names = {
        let method_names = exported_const_methods
            .iter()
            .map(|method| method.name_overwrite.clone().unwrap_or(method.name.clone()).to_string());

        quote! {
            fn get_const_method_names(&self) -> #fruity_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#method_names.to_string(),)*])
            }
        }
    };

    let impl_call_const_method = {
        let method_callers = exported_const_methods
            .iter()
            .map(|method| {
                let name = method.name.clone();
                let export_function_name = method.name_overwrite.clone().unwrap_or(method.name.clone()).to_string();
            
                let type_cast = match method.args.len() {
                    0 => None,
                    _ => {
                        let args_cast = method.args.iter().enumerate().map(|(index, arg)| {
                            let ident = syn::Ident::new(&format!("__arg_{}", index), Span::call_site());
                            let ty = arg.ty.clone();
    
                            quote! {
                                let #ident = __caster.cast_next::<#ty>()?;
                            }
                        }).collect::<Vec<_>>();
    
                        Some(
                            quote! {
                                let mut __caster = #fruity_crate::utils::introspect::ArgumentCaster::new(__args);
                                #(#args_cast)*
                            }
                        )
                    }
                };
                let arg_names = method.args.iter().enumerate().map(|(index, _arg)| syn::Ident::new(&format!("__arg_{}", index), Span::call_site()));
                
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
                    use #fruity_crate::script_value::convert::TryIntoScriptValue;
    
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
        let method_names = exported_mut_methods
            .iter()
            .map(|method| method.name_overwrite.clone().unwrap_or(method.name.clone()).to_string());

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
                let export_function_name = method.name_overwrite.clone().unwrap_or(method.name.clone()).to_string();
            
                let type_cast = match method.args.len() {
                    0 => None,
                    _ => {
                        let args_cast = method.args.iter().enumerate().map(|(index, arg)| {
                            let ident = syn::Ident::new(&format!("__arg_{}", index), Span::call_site());
                            let ty = arg.ty.clone();
        
                            quote! {
                                let #ident = __caster.cast_next::<#ty>()?;
                            }
                        }).collect::<Vec<_>>();
    
                        Some(
                            quote! {
                                let mut __caster = #fruity_crate::utils::introspect::ArgumentCaster::new(__args);
                                #(#args_cast)*
                            }
                        )
                    }
                };
                let arg_names = method.args.iter().enumerate().map(|(index, _arg)| syn::Ident::new(&format!("__arg_{}", index), Span::call_site()));
                
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
                    use #fruity_crate::script_value::convert::TryIntoScriptValue;
    
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
    let napi_constructor_bindings = quote!{};

    #[cfg(not(feature = "wasm-platform"))]
    let napi_constructor_bindings = {
        let napi_function_exports = exported_struct.clone().constructor
            .map(|constructor| {
                let ident = constructor.name.get_ident().unwrap();

                napi_function_export(
                    FruityExportFn {
                        name: syn::Path {
                            leading_colon: None,
                            segments:
                                syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::from_iter(
                                    vec![
                                        syn::PathSegment::from(exported_struct.name.clone()),
                                        syn::PathSegment::from(ident.clone())
                                    ],
                                ),
                        },
                        name_overwrite: Some(exported_struct.name.clone()),
                        attrs: constructor.attrs,
                        args: constructor.args,
                        return_ty: constructor.return_ty,
                        typescript_overwrite: constructor.typescript_overwrite,
                    }, None
                )
            });
        
            quote! {
                #napi_function_exports
            }
        };

    #[cfg(not(feature = "wasm-platform"))]
    let wasm_constructor_bindings = quote!{};

    #[cfg(feature = "wasm-platform")]
    let wasm_constructor_bindings = {
        let wasm_function_exports = exported_struct.clone().constructor
            .map(|constructor| {
                let ident = constructor.name.get_ident().unwrap();

                wasm_function_export(
                    FruityExportFn {
                        name: syn::Path {
                            leading_colon: None,
                            segments:
                                syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::from_iter(
                                    vec![
                                        syn::PathSegment::from(exported_struct.name.clone()),
                                        syn::PathSegment::from(ident.clone())
                                    ],
                                ),
                        },
                        name_overwrite: Some(exported_struct.name.clone()),
                        attrs: constructor.attrs,
                        args: constructor.args,
                        return_ty: constructor.return_ty,
                        typescript_overwrite: constructor.typescript_overwrite,
                    }, None
                )
            });
        
            quote! {
                #wasm_function_exports
            }
        };

    quote!{
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
    let from_script_value_converters = exported_enum.variants
        .iter()
        .map(|variant| {
            let variant_str = variant.to_string().to_case(Case::Camel);

            quote! {
                #variant_str => Ok(#name::#variant),
            }
        });

    // Generate TryIntoScriptValue converters
    let into_script_value_converters = exported_enum.variants
    .iter()
    .map(|variant| {
        let variant_str = variant.to_string().to_case(Case::Camel);

        quote! {
            #name::#variant => #variant_str,
        }
    });

    // Prepare the infos
    quote!{
        impl #fruity_crate::script_value::convert::TryFromScriptValue for #name {
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
        
        impl #fruity_crate::script_value::convert::TryIntoScriptValue for #name {
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

pub fn intern_derive_object_factory(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let ident_as_string = ident.to_string();

    let fields_converters = match data {
        syn::Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            fields
                .into_iter()
                .map(|FruityExportClassField { name, ty, public }| {
                    if public {
                        let name  = match name {
                            FruityExportClassFieldName::Named(name) => {
                                quote! { #name }
                            },
                            FruityExportClassFieldName::Unnamed(name) => {
                                quote! { #name }
                            },
                        };
                        let name_as_string = name.to_string();
                        let ty_as_string = quote! { #ty }.to_string();

                        // Special case for Option<ResourceReference<...>> we just grab the name and take the resource
                        // from the resource container
                        let option_resource_reference_resource_type = get_option_resource_reference_resource_type(&ty);
                        if let Some(option_resource_reference_resource_type) = option_resource_reference_resource_type {
                            return quote! {
                                if let Some(fruity_game_engine::script_value::ScriptValue::String(value)) = fields.get(#name_as_string) {
                                    new_object.#name = _resource_container.get::<#option_resource_reference_resource_type>(value);
                                }
                            }
                        }

                        // Special case for SignalProperty<...> we just grab the name and take the resource
                        // from the resource container
                        let signal_property_type = get_signal_property_type(&ty);
                        if let Some(signal_property_type) = signal_property_type {
                            let ty = signal_property_type;
                            let ty_as_string = quote! { #ty }.to_string();

                            return quote! {
                                if let Some(value) = fields.get(#name_as_string) {
                                    *new_object.#name.write() = {
                                        // First try to convert with TryFromScriptValue trait
                                        if let Ok(value) = <#ty as fruity_game_engine::script_value::convert::TryFromScriptValue>::from_script_value(value.clone()) {
                                            Ok(value)
                                        } else if let fruity_game_engine::script_value::ScriptValue::Object(script_object) = value {
                                            // Otherwise try to instantiate with the object factory service
                                            let script_object_values = <dyn fruity_game_engine::introspect::IntrospectFields>::get_field_values(script_object)?;
                                            if let Some(value) = object_factory_service.instantiate(#ty_as_string.to_string(), script_object_values)? {
                                                <#ty as fruity_game_engine::script_value::convert::TryFromScriptValue>::from_script_value(value.clone())
                                            } else {
                                                Err(fruity_game_engine::FruityError::GenericFailure(format!(
                                                    "Failed to instantiate a new {} with the object factory service, wrong value for the field {}",
                                                    #ident_as_string,
                                                    #name_as_string,
                                                )))
                                            }
                                        } else {
                                            Err(fruity_game_engine::FruityError::GenericFailure(format!(
                                                "Failed to instantiate a new {} with the object factory service, wrong value for the field {}",
                                                #ident_as_string,
                                                #name_as_string,
                                            )))
                                        }
                                    }?;
                                }
                            }
                        }

                        // Fallback to the regular case
                        quote! {
                            if let Some(value) = fields.get(#name_as_string) {
                                new_object.#name = {
                                    // First try to convert with TryFromScriptValue trait
                                    if let Ok(value) = <#ty as fruity_game_engine::script_value::convert::TryFromScriptValue>::from_script_value(value.clone()) {
                                        Ok(value)
                                    } else if let fruity_game_engine::script_value::ScriptValue::Object(script_object) = value {
                                        // Otherwise try to instantiate with the object factory service
                                        let script_object_values = <dyn fruity_game_engine::introspect::IntrospectFields>::get_field_values(script_object)?;
                                        if let Some(value) = object_factory_service.instantiate(#ty_as_string.to_string(), script_object_values)? {
                                            <#ty as fruity_game_engine::script_value::convert::TryFromScriptValue>::from_script_value(value.clone())
                                        } else {
                                            Err(fruity_game_engine::FruityError::GenericFailure(format!(
                                                "Failed to instantiate a new {} with the object factory service, wrong value for the field {}",
                                                #ident_as_string,
                                                #name_as_string,
                                            )))
                                        }
                                    } else {
                                        Err(fruity_game_engine::FruityError::GenericFailure(format!(
                                            "Failed to instantiate a new {} with the object factory service, wrong value for the field {}",
                                            #ident_as_string,
                                            #name_as_string,
                                        )))
                                    }
                                }?;
                            }
                        }
                    } else {
                        quote! { }
                    }
                })
        }
        syn::Data::Union(_) => unimplemented!("Union not supported"),
        syn::Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    let output = quote! {
        impl fruity_game_engine::object_factory_service::ObjectFactory for #ident {
            fn get_factory() -> fruity_game_engine::object_factory_service::Factory {
                std::sync::Arc::new(|object_factory_service: &fruity_game_engine::object_factory_service::ObjectFactoryService, _resource_container: fruity_game_engine::resource::resource_container::ResourceContainer, fields: std::collections::HashMap<String, fruity_game_engine::script_value::ScriptValue>| {
                    let mut new_object = Self::default();
        
                    #(#fields_converters)*
        
                    Ok(fruity_game_engine::script_value::ScriptValue::Object(Box::new(new_object) as Box<dyn fruity_game_engine::script_value::ScriptObject>))
                })
            }
        }
    };

    output.into()
}

fn get_option_resource_reference_resource_type(ty: &syn::Type) -> Option<&syn::Type> {
    let ty_path = if let syn::Type::Path(path) = ty {
        Some(path)
    } else {
        None
    }?;

    let last_segment = ty_path.path.segments.last()?;
    if last_segment.ident.to_string().as_str() != "Option" {
        return None;
    }

    let option_bracketed =
        if let syn::PathArguments::AngleBracketed(bracketed) = &last_segment.arguments {
            bracketed.args.first()
        } else {
            None
        }?;

    let option_ty = if let syn::GenericArgument::Type(option_ty) = option_bracketed {
        Some(option_ty)
    } else {
        None
    }?;

    let option_ty_path = if let syn::Type::Path(path) = option_ty {
        Some(path)
    } else {
        None
    }?;

    let last_segment = option_ty_path.path.segments.last()?;
    if last_segment.ident.to_string().as_str() != "ResourceReference" {
        return None;
    }

    let resource_reference_bracketed =
        if let syn::PathArguments::AngleBracketed(bracketed) = &last_segment.arguments {
            bracketed.args.first()
        } else {
            None
        }?;

    let resource_reference_ty =
        if let syn::GenericArgument::Type(resource_reference_ty) = resource_reference_bracketed {
            Some(resource_reference_ty)
        } else {
            None
        }?;

    Some(resource_reference_ty)
}

fn get_signal_property_type(ty: &syn::Type) -> Option<&syn::Type> {
    let ty_path = if let syn::Type::Path(path) = ty {
        Some(path)
    } else {
        None
    }?;

    let last_segment = ty_path.path.segments.last()?;
    if last_segment.ident.to_string().as_str() != "SignalProperty" {
        return None;
    }

    let signal_property_bracketed =
        if let syn::PathArguments::AngleBracketed(bracketed) = &last_segment.arguments {
            bracketed.args.first()
        } else {
            None
        }?;

    let signal_property_ty = if let syn::GenericArgument::Type(signal_property_ty) = signal_property_bracketed {
        Some(signal_property_ty)
    } else {
        None
    }?;

    Some(signal_property_ty)
}
