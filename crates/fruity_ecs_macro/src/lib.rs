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
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_game_engine::object_factory_service::ObjectFactory for #ident {
            fn get_factory() -> fruity_game_engine::object_factory_service::Constructor {
                use fruity_game_engine::introspect::IntrospectFields;

                std::sync::Arc::new(|_resource_container: fruity_game_engine::resource::resource_container::ResourceContainer, fields: std::collections::HashMap<String, fruity_game_engine::script_value::ScriptValue>| {
                    let mut new_object = Self::default();
                    fields.into_iter().try_for_each(|(key, value)| new_object.set_field_value(&key, value))?;

                    Ok(fruity_game_engine::script_value::ScriptValue::Object(Box::new(Box::new(new_object) as Box<dyn fruity_ecs::component::component::Component>)))
                })
            }
        }
    };

    output.into()
}
