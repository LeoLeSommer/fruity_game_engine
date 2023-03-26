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

#[proc_macro_derive(Serializable)]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    intern_derive_deserialize(input)
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

fn intern_derive_deserialize(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let ident_as_string = ident.to_string();

    let fields_initializer = match data {
        syn::Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let fields_converters = fields
                .into_iter()
                .map(|FruityExportClassField { name, ty, public }| {
                    if public {
                        match name {
                            FruityExportClassFieldName::Named(name) => {
                                let name_as_string = name.to_string();
                                quote! {
                                    if script_object_field_names.contains(&#name_as_string.to_string()) {
                                        result.#name = <#ty as fruity_ecs::serializable::Serializable>::deserialize(
                                            script_object.get_field_value(#name_as_string)?,
                                            resource_container.clone(),
                                            local_id_to_entity_id,
                                        )?;
                                    }
                                }
                            }
                            FruityExportClassFieldName::Unnamed(name) => {
                                let name_as_string = name.to_string();
                                let name = syn::Index::from(name);
                                quote! {
                                    if script_object_field_names.contains(&#name_as_string.to_string()) {
                                        result.#name = <#ty as fruity_ecs::serializable::Serializable>::deserialize(
                                            script_object.get_field_value(#name_as_string)?,
                                            resource_container.clone(),
                                            local_id_to_entity_id,
                                        )?;
                                    }
                                }
                            }
                        }
                    } else {
                        quote! { }
                    }
                });

            quote! {
                #(#fields_converters)*
            }
        }
        syn::Data::Union(_) => unimplemented!("Union not supported"),
        syn::Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    let output = quote! {
        impl fruity_ecs::serializable::Serializable for #ident {
            fn get_identifier() -> String {
                #ident_as_string.to_string()
            }

            fn deserialize(
                script_value: fruity_game_engine::script_value::ScriptValue,
                resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<u64, fruity_ecs::entity::EntityId>,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::script_value::ScriptValue::Object(script_object) = script_value {
                    let mut result = Self::default();
                    let script_object_field_names = script_object.get_field_names()?;

                    #fields_initializer

                    Ok(result)
                } else {
                    Err(fruity_game_engine::FruityError::GenericFailure({
                        let res = format!(
                            "Failed to deserialize a {0} from {1:?}",
                            #ident_as_string, &script_value
                        );
                        res
                    }))
                }
            }
        }
    };

    output.into()
}

fn intern_derive_deserialize_component_factory(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let ident_as_string = ident.to_string();

    let fields_initializer = match data {
        syn::Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let fields_converters = fields
                .into_iter()
                .map(|FruityExportClassField { name, ty, public }| {
                    if public {
                        match name {
                            FruityExportClassFieldName::Named(name) => {
                                let name_as_string = name.to_string();
                                quote! {
                                    if script_object_field_names.contains(&#name_as_string.to_string()) {
                                        result.#name = <#ty as fruity_ecs::serializable::Serializable>::deserialize(
                                            script_object.get_field_value(#name_as_string)?,
                                            resource_container.clone(),
                                            local_id_to_entity_id,
                                        )?;
                                    }
                                }
                            }
                            FruityExportClassFieldName::Unnamed(name) => {
                                let name_as_string = name.to_string();
                                let name = syn::Index::from(name);
                                quote! {
                                    if script_object_field_names.contains(&#name_as_string.to_string()) {
                                        result.#name = <#ty as fruity_ecs::serializable::Serializable>::deserialize(
                                            script_object.get_field_value(#name_as_string)?,
                                            resource_container.clone(),
                                            local_id_to_entity_id,
                                        )?;
                                    }
                                }
                            }
                        }
                    } else {
                        quote! { }
                    }
                });

            quote! {
                #(#fields_converters)*
            }
        }
        syn::Data::Union(_) => unimplemented!("Union not supported"),
        syn::Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    let output = quote! {
        impl fruity_ecs::serializable::Serializable for #ident {
            fn get_identifier() -> String {
                #ident_as_string.to_string()
            }

            fn deserialize(
                script_value: fruity_game_engine::script_value::ScriptValue,
                resource_container: fruity_game_engine::resource::resource_container::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<u64, fruity_ecs::entity::EntityId>,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::script_value::ScriptValue::Object(script_object) = script_value {
                    let mut result = Self::default();
                    let script_object_field_names = script_object.get_field_names()?;

                    #fields_initializer

                    Ok(result)
                } else {
                    Err(fruity_game_engine::FruityError::GenericFailure({
                        let res = format!(
                            "Failed to deserialize a {0} from {1:?}",
                            #ident_as_string, &script_value
                        );
                        res
                    }))
                }
            }
        }
    };

    output.into()
}
