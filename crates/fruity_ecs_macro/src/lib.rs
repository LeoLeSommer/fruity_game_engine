use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;
use syn::Data;
use syn::DeriveInput;
use syn::Fields;
use syn::Index;
use syn::AttrStyle;

#[proc_macro_derive(Component, attributes(native_only))]
pub fn derive_component(input: TokenStream) -> TokenStream {
    let derive_component_trait = derive_component_trait(input.clone());
    let derive_introspect_object_trait = derive_introspect_object_trait(input.clone());
    let derive_instantiable_object_trait = derive_component_instantiable_object_trait(input);

    let derive_component_trait = proc_macro2::TokenStream::from(derive_component_trait);
    let derive_introspect_object_trait = proc_macro2::TokenStream::from(derive_introspect_object_trait);
    let derive_instantiable_object_trait = proc_macro2::TokenStream::from(derive_instantiable_object_trait);

    let output = quote! {
        #derive_component_trait
        #derive_introspect_object_trait
        #derive_instantiable_object_trait
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

#[proc_macro_derive(IntrospectObject)]
pub fn derive_introspect_object_trait(input: TokenStream)  -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let struct_name = ident.to_string();

    let body = match data {
        Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = match data.fields {
                Fields::Named(ref fields) => fields
                    .named
                    .iter()
                    .filter(|field| {
                        field.attrs.iter().find(|attr| {
                            attr.style == AttrStyle::Outer &&
                            attr.path.segments.len() == 1 &&
                            attr.path.segments[0].ident.to_string() == "native_only"
                        }) == None
                    })
                    .map(|field| {
                        let ty = &field.ty;
                        match &field.ident {
                            Some(ident) => (quote! { #ident }, quote! { #ty }),
                            None => unimplemented!(),
                        }
                    })
                    .collect(),
                Fields::Unnamed(ref fields) => {
                    // For tuple struct, field name are numbers
                    fields
                        .unnamed
                        .iter()
                        .enumerate()
                        .map(|(index, field)| {
                            let ty = &field.ty;
                            let index = Index::from(index);
                            ( quote! { #index }, quote! { #ty })
                        })
                        .collect()
                }
                Fields::Unit => {
                    unimplemented!()
                }
            };

            let recurse_infos = fields.iter().map(|(name, ty)| {
                let name_as_string = name.to_string();
                let type_as_string = ty.to_string();

                quote! {
                    fruity_game_engine::introspect::FieldInfo {
                        name: #name_as_string.to_string(),
                        serializable: true,
                        getter: std::sync::Arc::new(|this| this.downcast_ref::<#ident>().unwrap().#name.clone().into_script_value()),
                        setter: fruity_game_engine::introspect::SetterCaller::Mut(std::sync::Arc::new(|this, value| {
                            fn convert<
                                T: fruity_game_engine::script_value::convert::TryFromScriptValue<fruity_game_engine::serialize::serialized::ScriptValue>,
                            >(
                                value: fruity_game_engine::serialize::serialized::ScriptValue,
                            ) -> Result<
                                T,
                                <T as fruity_game_engine::script_value::convert::TryFromScriptValue<
                                    fruity_game_engine::serialize::serialized::ScriptValue,
                                >>::Error,
                            > {
                                T::from_script_value(value)
                            }
        
                            let this = this.downcast_mut::<#ident>().unwrap();

                            match script_value::convert::<#ty>(value) {
                                Ok(value) => {
                                    this.#name = value
                                }
                                Err(_) => {
                                    log::error!(
                                        "Expected a {} for property {:?}",
                                        #type_as_string,
                                        #name_as_string,
                                    );
                                }
                            }
                        })),
                    },
                }
            });

            quote! {
                fn get_field_infos(&self) -> Vec<fruity_game_engine::introspect::FieldInfo> {
                    use fruity_game_engine::script_value::convert::TryIntoScriptValue;

                    vec![
                        #(#recurse_infos)*
                    ]
                }
            }
        }
        Data::Union(_) => unimplemented!("Union not supported"),
        Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    let output = quote! {
        impl fruity_game_engine::introspect::IntrospectObject for #ident {
            fn get_class_name(&self) -> String {
                #struct_name.to_string()
            }

            #body

            fn get_method_infos(&self) -> Vec<fruity_game_engine::introspect::MethodInfo> {
                vec![]
            }
        }
    };

    output.into()
}

fn derive_component_instantiable_object_trait(input: TokenStream)  -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_game_engine::introspect::InstantiableObject for #ident {
            fn get_constructor() -> fruity_game_engine::introspect::Constructor {
                use fruity_game_engine::introspect::IntrospectObject;

                std::sync::Arc::new(|_resource_container: fruity_game_engine::resource::resource_container::ResourceContainer, mut args: Vec<fruity_game_engine::serialize::serialized::ScriptValue>| {
                    let mut new_object = #ident::default();

                    if args.len() > 0 {
                        let serialized = args.remove(0);
                        let new_object_fields = new_object.get_field_infos();

                        if let fruity_game_engine::serialize::serialized::ScriptValue::Object { fields, .. } =
                            serialized
                        {
                            fields.into_iter().for_each(|(key, value)| {
                                let field_info = new_object_fields
                                    .iter()
                                    .find(|field_info| field_info.name == *key);

                                if let Some(field_info) = field_info {
                                    match &field_info.setter {
                                        fruity_game_engine::introspect::SetterCaller::Const(call) => {
                                            call(new_object.as_any_ref(), value);
                                        }
                                        fruity_game_engine::introspect::SetterCaller::Mut(call) => {
                                            call(new_object.as_any_mut(), value);
                                        }
                                        fruity_game_engine::introspect::SetterCaller::None => (),
                                    }
                                }
                            })
                        };
                    };
        
                    Ok(fruity_game_engine::serialize::serialized::ScriptValue::Object(Box::new(fruity_ecs::component::component::AnyComponent::new(new_object))))
                })
            }
        }
    };

    output.into()
}

#[proc_macro_derive(ObjectFactory)]
pub fn derive_instantiable_object_trait(input: TokenStream)  -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);

    let output = quote! {
        impl fruity_game_engine::introspect::InstantiableObject for #ident {
            fn get_constructor() -> fruity_game_engine::introspect::Constructor {
                use fruity_game_engine::script_value::convert::TryIntoScriptValue;
                use fruity_game_engine::introspect::IntrospectObject;

                std::sync::Arc::new(|_resource_container: fruity_game_engine::resource::resource_container::ResourceContainer, mut args: Vec<fruity_game_engine::serialize::serialized::ScriptValue>| {
                    let mut new_object = #ident::default();

                    if args.len() > 0 {
                        let serialized = args.remove(0);
                        let new_object_fields = new_object.get_field_infos();

                        if let fruity_game_engine::serialize::serialized::ScriptValue::Object { fields, .. } =
                            serialized
                        {
                            fields.into_iter().for_each(|(key, value)| {
                                let field_info = new_object_fields
                                    .iter()
                                    .find(|field_info| field_info.name == *key);

                                if let Some(field_info) = field_info {
                                    match &field_info.setter {
                                        fruity_game_engine::introspect::SetterCaller::Const(call) => {
                                            call(new_object.as_any_ref(), value);
                                        }
                                        fruity_game_engine::introspect::SetterCaller::Mut(call) => {
                                            call(new_object.as_any_mut(), value);
                                        }
                                        fruity_game_engine::introspect::SetterCaller::None => (),
                                    }
                                }
                            })
                        };
                    };
        
                    Ok(new_object.into_script_value())
                })
            }
        }
    };

    output.into()
}

#[proc_macro_derive(SerializableObject)]
pub fn derive_serializable_object(input: TokenStream)  -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let ident_as_string = ident.to_string();

    let output = quote! {
        impl fruity_game_engine::serialize::serialized::SerializableObject for #ident {
            fn duplicate(&self) -> Box<dyn fruity_game_engine::serialize::serialized::SerializableObject> {
                Box::new(self.clone())
            }
        }
        
        impl fruity_game_engine::script_value::convert::TryFromScriptValue<fruity_game_engine::serialize::serialized::ScriptValue> for #ident {
            type Error = String;
        
            fn from_script_value(value: fruity_game_engine::serialize::serialized::ScriptValue) -> Result<Self, Self::Error> {
                match value {
                    /*fruity_game_engine::serialize::serialized::ScriptValue::Object { fields, .. } => {
                        use fruity_game_engine::introspect::IntrospectObject;

                        let mut new_object = #ident::default();
                        let new_object_fields = new_object.get_field_infos();

                        fields.into_iter().for_each(|(key, value)| {
                            let field_info = new_object_fields
                                .iter()
                                .find(|field_info| field_info.name == *key);

                            if let Some(field_info) = field_info {
                                match &field_info.setter {
                                    fruity_game_engine::introspect::SetterCaller::Const(call) => {
                                        call(new_object.as_any_ref(), value);
                                    }
                                    fruity_game_engine::introspect::SetterCaller::Mut(call) => {
                                        call(new_object.as_any_mut(), value);
                                    }
                                    fruity_game_engine::introspect::SetterCaller::None => (),
                                }
                            }
                        });

                        Ok(new_object)
                    }*/
                    fruity_game_engine::serialize::serialized::ScriptValue::Object(value) => {
                        match value.as_any_box().downcast::<#ident>() {
                            Ok(value) => Ok(*value),
                            Err(_) => Err(format!(
                                "Couldn't convert a {} to native object", #ident_as_string
                            )),
                        }
                    }
                    _ => Err(format!("Couldn't convert {:?} to native object", value)),
                }
            }
        }
        
        impl fruity_game_engine::script_value::convert::TryIntoScriptValue<fruity_game_engine::serialize::serialized::ScriptValue> for #ident {
            fn into_script_value(&self) -> fruity_game_engine::serialize::serialized::ScriptValue {
                fruity_game_engine::serialize::serialized::ScriptValue::Object(Box::new(self))
            }
        }
    };

    output.into()
}