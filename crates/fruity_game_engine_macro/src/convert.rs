use crate::parse::{parse_struct_fields, ParsedField};
use crate::utils::current_crate;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

pub fn intern_derive_try_from_script_value(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let current_crate = current_crate();

    let output = match data {
        Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let convert_args = fields.iter().map(|ParsedField { name, ty, public }| {
                if *public {
                    let name_as_string = name.to_string();

                    Some(quote! {
                        #name: <#ty>::from_script_value(value.get_field_value(#name_as_string)?)?,
                    })
                } else {
                    None
                }
            });

            quote! {
                impl #current_crate::script_value::convert::TryFromScriptValue for #ident {
                    fn from_script_value(value: #current_crate::script_value::ScriptValue) -> #current_crate::FruityResult<Self> {
                        match value {
                            #current_crate::script_value::ScriptValue::Object(value) => {
                                match value.downcast::<Self>() {
                                    Ok(value) => Ok(*value),
                                    Err(value) => {
                                        Ok(Self {
                                            #(#convert_args)*
                                        })
                                    }
                                }
                            }
                            _ => Err(#current_crate::FruityError::InvalidArg(
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
