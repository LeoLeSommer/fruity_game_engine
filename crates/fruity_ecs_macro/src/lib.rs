use fruity_game_engine_code_parser::parse_struct_fields;
use fruity_game_engine_code_parser::FruityExportClassField;
use fruity_game_engine_code_parser::FruityExportClassFieldName;
use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::DeriveInput;
use syn::__private::TokenStream2;

#[proc_macro_derive(Component, attributes(serialize_skip))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let derive_component_trait = derive_component_trait(input.clone());
    let derive_component_trait = proc_macro2::TokenStream::from(derive_component_trait);

    let derive_serialize = intern_derive_serialize(input.clone());
    let derive_serialize = proc_macro2::TokenStream::from(derive_serialize);

    let derive_deserialize = intern_derive_deserialize(input.clone());
    let derive_deserialize = proc_macro2::TokenStream::from(derive_deserialize);

    let output = quote! {
        #derive_component_trait
        #derive_serialize
        #derive_deserialize
    };

    output.into()
}

#[proc_macro_derive(Serialize, attributes(serialize_skip))]
pub fn derive_serialize(input: TokenStream) -> TokenStream {
    intern_derive_serialize(input)
}

#[proc_macro_derive(Deserialize, attributes(serialize_skip))]
pub fn derive_deserialize(input: TokenStream) -> TokenStream {
    intern_derive_deserialize(input)
}

fn derive_component_trait(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let fruity_ecs_crate = fruity_ecs_crate();

    let output = quote! {
        impl #fruity_ecs_crate::component::Component for #ident {
            fn duplicate(&self) -> Box<dyn #fruity_ecs_crate::component::Component> {
                Box::new(self.clone())
            }

            fn get_component_type_id(&self) -> fruity_game_engine::FruityResult<#fruity_ecs_crate::component::ComponentTypeId> {
                Ok(#fruity_ecs_crate::component::ComponentTypeId::Rust(std::any::TypeId::of::<Self>()))
            }

            fn get_storage(&self) -> Box<dyn #fruity_ecs_crate::component::ComponentStorage> {
                Box::new(#fruity_ecs_crate::component::VecComponentStorage::<Self>::new())
            }
        }
    };

    output.into()
}

fn intern_derive_serialize(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let fruity_ecs_crate = fruity_ecs_crate();

    let fields_initializer = match data {
        syn::Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let fields_converters = fields
                .into_iter()
                .map(|FruityExportClassField { name, attrs, .. }| {
                    if !attrs.contains(&"serialize_skip".to_string()) {
                        match name {
                            FruityExportClassFieldName::Named(name) => {
                                let name_as_string = name.to_string();
                                quote! {
                                    result.insert(#name_as_string.to_string(), self.#name.serialize(resource_container)?);
                                }
                            }
                            FruityExportClassFieldName::Unnamed(name) => {
                                let name_as_string = name.to_string();
                                let name = syn::Index::from(name);
                                quote! {
                                    result.insert(#name_as_string.to_string(), self.#name.serialize(resource_container)?);
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
        impl #fruity_ecs_crate::serialization::Serialize for #ident {
            fn serialize(
                &self,
                resource_container: &fruity_game_engine::resource::resource_container::ResourceContainer
            ) -> fruity_game_engine::FruityResult<fruity_game_engine::settings::Settings> {
                let mut result = std::collections::HashMap::<String, fruity_game_engine::settings::Settings>::new();
                #fields_initializer

                Ok(fruity_game_engine::settings::Settings::Object(result))
            }
        }
    };

    output.into()
}

fn intern_derive_deserialize(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let ident_as_string = ident.to_string();
    let fruity_ecs_crate = fruity_ecs_crate();

    let fields_initializer = match data {
        syn::Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let fields_converters = fields
                .into_iter()
                .map(|FruityExportClassField { name, ty, attrs, .. }| {
                    if !attrs.contains(&"serialize_skip".to_string()) {
                        match name {
                            FruityExportClassFieldName::Named(name) => {
                                let name_as_string = name.to_string();
                                quote! {
                                    if serialized.contains_key(&#name_as_string.to_string()) {
                                        result.#name = <#ty as #fruity_ecs_crate::serialization::Deserialize>::deserialize(
                                            serialized.get(#name_as_string).unwrap(),
                                            resource_container,
                                            local_id_to_entity_id,
                                        )?;
                                    }
                                }
                            }
                            FruityExportClassFieldName::Unnamed(name) => {
                                let name_as_string = name.to_string();
                                let name = syn::Index::from(name);
                                quote! {
                                    if serialized.contains_key(&#name_as_string.to_string()) {
                                        result.#name = <#ty as #fruity_ecs_crate::serialization::Deserialize>::deserialize(
                                            serialized.get(#name_as_string).unwrap(),
                                            resource_container,
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
        impl #fruity_ecs_crate::serialization::Deserialize for #ident {
            fn get_identifier() -> String {
                #ident_as_string.to_string()
            }

            fn deserialize(
                serialized: &fruity_game_engine::settings::Settings,
                resource_container: &fruity_game_engine::resource::resource_container::ResourceContainer,
                local_id_to_entity_id: &std::collections::HashMap<u64, #fruity_ecs_crate::entity::EntityId>,
            ) -> fruity_game_engine::FruityResult<Self> {
                if let fruity_game_engine::settings::Settings::Object(serialized) = serialized {
                    let mut result = Self::default();

                    #fields_initializer

                    Ok(result)
                } else {
                    Err(fruity_game_engine::FruityError::GenericFailure({
                        let res = format!(
                            "Failed to deserialize a {0} from {1:?}",
                            #ident_as_string, &serialized
                        );
                        res
                    }))
                }
            }
        }
    };

    output.into()
}

fn fruity_ecs_crate() -> TokenStream2 {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();

    if crate_name == "fruity_ecs" {
        quote! { crate }
    } else {
        quote! { ::fruity_ecs }
    }
}
