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
        intern_derive_deserialize_component_factory(input.clone());
    let derive_component_instantiable_object_trait =
        proc_macro2::TokenStream::from(derive_component_instantiable_object_trait);

    let output = quote! {
        #derive_component_trait
        #derive_component_instantiable_object_trait
    };

    output.into()
}

#[proc_macro_derive(DeserializeFactory)]
pub fn derive_deserialize_factory(input: TokenStream) -> TokenStream {
    intern_derive_deserialize_factory(input)
}

fn derive_component_trait(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let struct_name = ident.to_string();

    let output = quote! {
        impl fruity_ecs::component::Component for #ident {
            fn get_storage(&self) -> Box<dyn fruity_ecs::entity::archetype::component_storage::ComponentStorage> {
                Box::new(fruity_ecs::entity::archetype::component_storage::VecComponentStorage::<Self>::new())
            }

            fn duplicate(&self) -> Box<dyn fruity_ecs::component::Component> {
                Box::new(self.clone())
            }
        }

        impl fruity_ecs::component::StaticComponent for #ident {
            fn get_component_name() -> &'static str {
                #struct_name
            }
        }
    };

    output.into()
}

fn intern_derive_deserialize_factory(input: TokenStream) -> TokenStream {
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
                                let name = syn::Index::from(name);
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

                        // Fallback to the regular case
                        quote! {
                            if let Some(value) = fields.get(#name_as_string) {
                                new_object.#name = {
                                    // First try to deserialize with the deserialize factory service
                                    let value = if let Some(value) = deserialize_service.instantiate(value.clone(), #ty_as_string.to_string(), local_id_to_entity_id)? {
                                        value
                                    } else {
                                        value.clone()
                                    };

                                    <#ty as fruity_game_engine::script_value::convert::TryFromScriptValue>::from_script_value(value.clone())
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
        impl fruity_ecs::deserialize_service::DeserializeFactory for #ident {
            fn get_factory() -> fruity_ecs::deserialize_service::Factory {
                std::sync::Arc::new(|
                    deserialize_service: &fruity_ecs::deserialize_service::DeserializeService,
                    value: fruity_game_engine::script_value::ScriptValue,
                    _resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
                    local_id_to_entity_id: &std::collections::HashMap<u64, fruity_ecs::entity::EntityId>
                | {
                    if let fruity_game_engine::script_value::ScriptValue::Object(script_object) = value {
                        let fields = <dyn fruity_game_engine::introspect::IntrospectFields>::get_field_values(&script_object)?;

                        let mut new_object = Self::default();

                        #(#fields_converters)*

                        Ok(fruity_game_engine::script_value::ScriptValue::Object(Box::new(new_object)))
                    } else {
                        Err(fruity_game_engine :: FruityError ::
                        GenericFailure(format!
                        ("Failed to deserialize a {} from {:?}",
                        #ident_as_string, &value)))
                    }
                })
            }
        }
    };

    output.into()
}

fn intern_derive_deserialize_component_factory(input: TokenStream) -> TokenStream {
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
                                let name = syn::Index::from(name);
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

                        // Fallback to the regular case
                        quote! {
                            if let Some(value) = fields.get(#name_as_string) {
                                new_object.#name = {
                                    // First try to deserialize with the deserialize factory service
                                    let value = if let Some(value) = deserialize_service.instantiate(value.clone(), #ty_as_string.to_string(), local_id_to_entity_id)? {
                                        value
                                    } else {
                                        value.clone()
                                    };

                                    <#ty as fruity_game_engine::script_value::convert::TryFromScriptValue>::from_script_value(value.clone())
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
        impl fruity_ecs::deserialize_service::DeserializeFactory for #ident {
            fn get_factory() -> fruity_ecs::deserialize_service::Factory {
                std::sync::Arc::new(|
                    deserialize_service: &fruity_ecs::deserialize_service::DeserializeService,
                    value: fruity_game_engine::script_value::ScriptValue,
                    _resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
                    local_id_to_entity_id: &std::collections::HashMap<u64, fruity_ecs::entity::EntityId>
                | {
                    if let fruity_game_engine::script_value::ScriptValue::Object(script_object) = value {
                        let fields = <dyn fruity_game_engine::introspect::IntrospectFields>::get_field_values(&script_object)?;

                        let mut new_object = Self::default();

                        #(#fields_converters)*

                        Ok(fruity_game_engine::script_value::ScriptValue::Object(Box::new(Box::new(new_object) as Box<dyn fruity_ecs::component::Component>)))
                    } else {
                        Err(fruity_game_engine :: FruityError ::
                        GenericFailure(format!
                        ("Failed to deserialize a {} from {:?}",
                        #ident_as_string, &value)))
                    }
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
