use fruity_game_engine_code_parser::parse_struct_fields;
use fruity_game_engine_code_parser::FruityExportClassField;
use fruity_game_engine_code_parser::FruityExportClassFieldName;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;

#[proc_macro_derive(Component, attributes(native_only))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let derive_component_trait = derive_component_trait(input.clone());
    let derive_component_trait = proc_macro2::TokenStream::from(derive_component_trait);

    let derive_component_instantiable_object_trait =
        derive_component_instantiable_object_trait(input.clone());
    let derive_component_instantiable_object_trait =
        proc_macro2::TokenStream::from(derive_component_instantiable_object_trait);

    let output = quote! {
        #derive_component_trait
        #derive_component_instantiable_object_trait
    };

    output.into()
}

fn derive_component_trait(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let struct_name = ident.to_string();

    let output = quote! {
        impl fruity_ecs::component::component::Component for #ident {
            fn get_collection(&self) -> Box<dyn fruity_ecs::entity::archetype::component_collection::ComponentCollection> {
                Box::new(fruity_ecs::entity::archetype::component_array::ComponentArray::<#ident>::new())
            }

            fn duplicate(&self) -> Box<dyn fruity_ecs::component::component::Component> {
                Box::new(self.clone())
            }
        }

        impl fruity_ecs::component::component::StaticComponent for #ident {
            fn get_component_name() -> &'static str {
                #struct_name
            }
        }
    };

    output.into()
}

fn derive_component_instantiable_object_trait(input: TokenStream) -> TokenStream {
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

                    Ok(fruity_game_engine::script_value::ScriptValue::Object(Box::new(Box::new(new_object) as Box<dyn fruity_ecs::component::component::Component>)))
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

    let signal_property_ty =
        if let syn::GenericArgument::Type(signal_property_ty) = signal_property_bracketed {
            Some(signal_property_ty)
        } else {
            None
        }?;

    Some(signal_property_ty)
}
