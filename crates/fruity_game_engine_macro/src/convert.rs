use crate::utils::fruity_crate;
use fruity_game_engine_code_parser::{parse_struct_fields, FruityExportClassField, FruityExportClassFieldName};
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn intern_derive_try_from_script_value(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let fruity_crate = fruity_crate();

    let output = match data {
        Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let convert_args = fields
                .iter()
                .map(|FruityExportClassField { name, ty, public, .. }| {
                    if *public {
                        Some(match name {
                            FruityExportClassFieldName::Named(name) => {
                                let name_as_string = name.to_string();
                    
                                quote! {
                                    #name: <#ty>::from_script_value(value.get_field_value(#name_as_string)?)?,
                                }
                            },
                            FruityExportClassFieldName::Unnamed(name) => {
                                let name_as_string = name.to_string();
                    
                                quote! {
                                    #name: <#ty>::from_script_value(value.get_field_value(#name_as_string)?)?,
                                }
                            },
                        })
                    } else {
                        None
                    }
                });

            quote! {
                impl #fruity_crate::script_value::convert::TryFromScriptValue for #ident {
                    fn from_script_value(value: #fruity_crate::script_value::ScriptValue) -> #fruity_crate::FruityResult<Self> {
                        match value {
                            #fruity_crate::script_value::ScriptValue::Object(value) => {
                                match value.downcast::<Self>() {
                                    Ok(value) => Ok(*value),
                                    Err(value) => {
                                        Ok(Self {
                                            #(#convert_args)*
                                        })
                                    }
                                }
                            }
                            _ => Err(#fruity_crate::FruityError::InvalidArg(
                                format!("Couldn't convert {:?} to native object", value),
                             )),
                        }
                    }
                }
            }
        }
        Data::Union(_) => unimplemented!("Union not supported"),
        Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    output.into()
}

pub fn intern_derive_try_into_script_value(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let fruity_crate = fruity_crate();
    let ident_as_string = ident.to_string();

    let output = match data {
        Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let convert_args = fields.iter().map(|FruityExportClassField { name, ty, public, .. }| {
                if *public {
                    Some(match name {
                        FruityExportClassFieldName::Named(name) => {
                            let name_as_string = name.to_string();
                
                            quote! {
                                fields.insert(#name_as_string.to_string(), <#ty>::into_script_value(self.#name)?);
                            }
                        },
                        FruityExportClassFieldName::Unnamed(name) => {
                            let name_as_string = name.to_string();
                
                            quote! {
                                fields.insert(#name_as_string.to_string(), <#ty>::into_script_value(self.#name)?);
                            }
                        },
                    })
                } else {
                    None
                }
            });

            quote! {
                impl #fruity_crate::script_value::convert::TryIntoScriptValue for #ident {
                    fn into_script_value(self) -> #fruity_crate::FruityResult<#fruity_crate::script_value::ScriptValue> {
                        let mut fields = std::collections::HashMap::new();

                        #(#convert_args)*

                        let script_object = #fruity_crate::script_value::HashMapScriptObject {
                            class_name: #ident_as_string.to_string(),
                            fields,
                        };

                        Ok(#fruity_crate::script_value::ScriptValue::Object(Box::new(
                            script_object,
                        )))
                    }
                }
            }
        }
        Data::Union(_) => unimplemented!("Union not supported"),
        Data::Enum(_) => unimplemented!("Enum not supported"),
    };

    output.into()
}
