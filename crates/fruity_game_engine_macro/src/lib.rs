extern crate syn;
extern crate quote;

use parse::{parse_impl_method, ParsedField, ParsedMethod, parse_struct_fields, ParsedReceiver};
use proc_macro::{self, TokenStream};
use proc_macro2::Span;
use quote::quote;
use syn::__private::TokenStream2;
use syn::{
    parse_macro_input,  Data, DeriveInput, 
    Stmt, ItemFn
};

mod parse;

fn current_crate() -> TokenStream2 {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();

    if crate_name == "fruity_game_engine" {
        quote! { crate }
    } else {
        quote! { ::fruity_game_engine }
    }
}

#[proc_macro_derive(FruityAny)]
pub fn derive_fruity_any(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let current_crate = current_crate();
    let ident_string = ident.to_string();

    let output = quote! {
        impl #impl_generics #current_crate::any::FruityAny for #ident #ty_generics #where_clause {
            fn get_type_name(&self) -> &'static str {
                #ident_string
            }

            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }

            fn as_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn std::any::Any> {
                self
            }

            fn as_fruity_any_ref(&self) -> &dyn #current_crate::any::FruityAny {
                self
            }

            fn as_fruity_any_mut(&mut self) -> &mut dyn #current_crate::any::FruityAny {
                self
            }

            fn as_fruity_any_box(self: Box<Self>) -> Box<dyn #current_crate::any::FruityAny> {
                self
            }

            fn as_fruity_any_rc(self: std::rc::Rc<Self>) -> std::rc::Rc<dyn #current_crate::any::FruityAny> {
                self
            }
        }
    };

    output.into()
}

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let current_crate = current_crate();

    let output = quote! {
        impl #impl_generics #current_crate::resource::Resource for #ident #ty_generics #where_clause {
            fn as_resource_box(self: Box<Self>) -> Box<dyn #current_crate::resource::Resource> {
                self
            }

            fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
              self
            }
        }
    };

    output.into()
}

#[proc_macro_derive(TryFromScriptValue)]
pub fn derive_fruity_try_from_fruity_any(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, data, .. } = parse_macro_input!(input);
    let current_crate = current_crate();

    let output = match data {
        Data::Struct(ref data) => {
            // Create a list with all field names,
            let fields: Vec<_> = parse_struct_fields(&data.fields);

            let convert_args = fields.iter().map(|ParsedField { name, ty, public  }| {
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
                            _ => Err(#current_crate::FruityError::new(
                                #current_crate::FruityStatus::InvalidArg,
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

#[proc_macro_attribute]
pub fn fruity_module_exports(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_2 = input.clone();
    let fn_input: ItemFn = parse_macro_input!(input_2);
    let fn_ident = fn_input.sig.ident;
    let current_crate = current_crate();
    let input: TokenStream2 = input.clone().into();

    let output = quote! {
        #input

        #[#current_crate::napi::bindgen_prelude::ctor]
        fn __napi__explicit_module_register() {
            unsafe fn register(raw_env: #current_crate::napi::sys::napi_env, raw_exports: #current_crate::napi::sys::napi_value) -> #current_crate::napi::Result<()> {
                use #current_crate::napi::{Env, JsObject, NapiValue};

                let env = Env::from_raw(raw_env);
                let exports = JsObject::from_raw_unchecked(raw_env, raw_exports);
                let export_js = #current_crate::javascript::ExportJavascript::new(exports, env);

                #fn_ident(export_js)
            }

            #current_crate::napi::bindgen_prelude::register_module_exports(register)
        }
    };

    output.into()
}

#[proc_macro]
pub fn fruity_export(input: TokenStream) -> TokenStream {
    let current_crate = current_crate();
    let input_2: TokenStream2 = input.clone().into();
    let braced_input = quote! {
        {
            #input_2
        }
    };

    let item = syn::parse2::<syn::Block>(braced_input.into()).unwrap();

    // Parse the block
    let mut struct_name = syn::Ident::new("unknown", Span::call_site());
            let struct_name_as_string = struct_name.to_string();
            let mut fields: Vec<ParsedField> = Vec::new();

    let mut methods: Vec<ParsedMethod> = Vec::new();
    item.stmts.into_iter().for_each(|stmt| match stmt {
        Stmt::Item(item) => match item {
            syn::Item::Struct(item_struct) => {
                struct_name = item_struct.ident.clone();
                let mut structure_fields = parse_struct_fields(&item_struct.fields);
                fields.append(&mut structure_fields);
            }
            syn::Item::Impl(item_impl) => {
                let mut impl_methods = item_impl
                    .items
                    .into_iter()
                    .filter_map(|item| {
                        if let syn::ImplItem::Method(method) = item {
                            Some(method)
                        } else {
                            None
                        }
                    })
                    .map(|item| parse_impl_method(&item))
                    .collect::<Vec<_>>();

                methods.append(&mut impl_methods);
            }
            _ => {}
        },
        _ => {}
    });

    // Prepare the infos
    let exported_fields = fields
        .clone()
        .into_iter()
        .filter(|field| field.public)
        .collect::<Vec<_>>();

    let exported_const_methods = methods
        .clone()
        .into_iter()
        .filter_map(|method| {
            method.attrs.iter()
                .find(|attr| attr.ident.to_string() == "export")
                .map(|export_attr| (method.clone(), export_attr.clone()))
        })
        .filter(|(method, _)| matches!(method.receiver, ParsedReceiver::Const))
        .collect::<Vec<_>>();

    let exported_mut_methods = methods
        .clone()
        .into_iter()
        .filter_map(|method| {
            method.attrs.iter()
                .find(|attr| attr.ident.to_string() == "export")
                .map(|export_attr| (method.clone(), export_attr.clone()))
        })
        .filter(|(method, _)| matches!(method.receiver, ParsedReceiver::Mut))
        .collect::<Vec<_>>();

    // Implement the IntrospectObject functions
    let impl_get_class_name = quote! {
        fn get_class_name(&self) -> #current_crate::FruityResult<String> {
            Ok(#struct_name_as_string.to_string())
        }
    };

    let impl_get_field_names = {
        let fields_names = exported_fields
            .iter()
            .map(|field| field.name.to_string());

        quote! {
            fn get_field_names(&self) -> #current_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#fields_names.to_string(),)*])
            }
        }
    };

    let impl_set_field_value = {
        let fields_setters = exported_fields
            .iter()
            .map(|field| {
                let name = field.name.clone();
                let name_as_string = field.name.to_string();
                let ty = field.ty.clone();
                
                quote! {
                    #name_as_string => self.#name = <#ty>::from_script_value(value)?,
                }
            });


        if fields_setters.len() > 0 {
            quote! {
                fn set_field_value(&mut self, name: &str, value: #current_crate::script_value::ScriptValue) -> #current_crate::FruityResult<()> {
                    use #current_crate::script_value::convert::TryFromScriptValue;

                    match name {
                        #(#fields_setters)*
                        _ => unreachable!(),
                    };
            
                    #current_crate::FruityResult::Ok(())
                }
            }
        } else {
            quote! {
                fn set_field_value(&mut self, name: &str, value: #current_crate::script_value::ScriptValue) -> #current_crate::FruityResult<()> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_field_value = {
        let fields_getters = exported_fields
            .iter()
            .map(|field| {
                let name = field.name.clone();
                let name_as_string = field.name.to_string();
                let ty = field.ty.clone();
                
                quote! {
                    #name_as_string => <#ty>::into_script_value(self.#name.clone()),
                }
            });


        if fields_getters.len() > 0 {
            quote! {
                fn get_field_value(&self, name: &str) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    use #current_crate::script_value::convert::TryIntoScriptValue;
                
                    match name {
                        #(#fields_getters)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn get_field_value(&self, name: &str) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_const_method_names = {
        let method_names = exported_const_methods
            .iter()
            .map(|(method, export_attr)| {
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
                
                export_function_name
            });

        quote! {
            fn get_const_method_names(&self) -> #current_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#method_names.to_string(),)*])
            }
        }
    };

    let impl_call_const_method = {
        let method_callers = exported_const_methods
            .iter()
            .map(|(method, export_attr)| {
                let name = method.name.clone();
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
            
                let type_cast = match method.args.len() {
                    0 => None,
                    _ => {
                        let args_cast = method.args.iter().map(|arg| {
                            let name = arg.name.clone();
                            let ty = arg.ty.clone();
    
                            quote! {
                                let #name = __caster.cast_next::<#ty>()?;
                            }
                        }).collect::<Vec<_>>();
    
                        Some(
                            quote! {
                                let mut __caster = #current_crate::utils::introspect::ArgumentCaster::new(__args);
                                #(#args_cast)*
                            }
                        )
                    }
                };
                let arg_names = method.args.iter().map(|arg| arg.name.clone());
                
                quote! {
                    #export_function_name => {
                        #type_cast
                        self.#name(#(#arg_names),*).into_script_value()
                    },
                }
            });

        if method_callers.len() > 0 {
            quote! {
                fn call_const_method(&self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    use #current_crate::script_value::convert::TryIntoScriptValue;
    
                    match name {
                        #(#method_callers)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn call_const_method(&self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    let impl_get_mut_method_names = {
        let method_names = exported_mut_methods
            .iter()
            .map(|(method, export_attr)| {
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
                
                export_function_name
            });

        quote! {
            fn get_mut_method_names(&self) -> #current_crate::FruityResult<Vec<String>> {
                Ok(vec![#(#method_names.to_string(),)*])
            }
        }
    };

    let impl_call_mut_method = {
        let method_callers = exported_mut_methods
            .iter()
            .map(|(method, export_attr)| {
                let name = method.name.clone();
                let name_as_string = method.name.to_string();
                let export_function_name = export_attr.params
                    .get("name")
                    .map(|name| name.to_string().replace("\"", ""))
                    .unwrap_or(name_as_string);
            
                let type_cast = match method.args.len() {
                    0 => None,
                    _ => {
                        let args_cast = method.args.iter().map(|arg| {
                            let name = arg.name.clone();
                            let ty = arg.ty.clone();
    
                            quote! {
                                let #name = __caster.cast_next::<#ty>()?;
                            }
                        }).collect::<Vec<_>>();
    
                        Some(
                            quote! {
                                let mut __caster = #current_crate::utils::introspect::ArgumentCaster::new(__args);
                                #(#args_cast)*
                            }
                        )
                    }
                };
                let arg_names = method.args.iter().map(|arg| arg.name.clone());
                
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
                fn call_mut_method(&mut self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    use #current_crate::script_value::convert::TryIntoScriptValue;
    
                    match name {
                        #(#method_callers)*
                        _ => unreachable!(),
                    }
                }
            }
        } else {
            quote! {
                fn call_mut_method(&mut self, name: &str, __args: Vec<#current_crate::script_value::ScriptValue>) -> #current_crate::FruityResult<#current_crate::script_value::ScriptValue> {
                    unreachable!()
                }
            }
        }
    };

    let impl_introspect_object = quote!{
        impl #current_crate::introspect::IntrospectObject for #struct_name
        {
            #impl_get_class_name
            #impl_get_field_names
            #impl_set_field_value
            #impl_get_field_value
            #impl_get_const_method_names
            #impl_call_const_method
            #impl_get_mut_method_names
            #impl_call_mut_method
        }
    };
    
    // Assemble everything

    let input_2: TokenStream2 = input.clone().into();
    let output = quote! {
        #input_2

        #impl_introspect_object
    };

    output.into()
}

#[proc_macro_attribute]
pub fn export(_attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}