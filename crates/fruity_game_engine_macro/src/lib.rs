extern crate syn;
extern crate quote;

use parse::{parse_impl_method, ParsedField, ParsedMethod, parse_struct_fields};
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

#[proc_macro_derive(FruityFrom)]
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
                        #name: <#ty>::fruity_try_from(
                            fields
                                .remove(#name_as_string)
                                .unwrap_or(#current_crate::script_value::ScriptValue::Undefined),
                        )?,
                    })
                } else {
                    None
                }
            });

            quote! {
                impl #current_crate::convert::FruityFrom<#current_crate::script_value::ScriptValue> for #ident {
                    fn fruity_try_from(value: #current_crate::script_value::ScriptValue) -> #current_crate::FruityResult<Self> {
                        match value {
                            #current_crate::script_value::ScriptValue::NativeObject(value) => {
                                match value.as_any_box().downcast::<Self>() {
                                    Ok(value) => Ok(*value),
                                    Err(_) => Err(#current_crate::FruityError::new(
                                        #current_crate::FruityStatus::InvalidArg,
                                        "Couldn't convert An AnyComponent to native object".to_string(),
                                      )),
                                }
                            }
                            #current_crate::script_value::ScriptValue::Object { mut fields, .. } => Ok(Self {
                                #(#convert_args)*
                            }),
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

        #[napi::bindgen_prelude::ctor]
        fn __napi__explicit_module_register() {
            unsafe fn register(raw_env: napi::sys::napi_env, raw_exports: napi::sys::napi_value) -> napi::Result<()> {
                use napi::{Env, JsObject, NapiValue};

                let env = Env::from_raw(raw_env);
                let exports = JsObject::from_raw_unchecked(raw_env, raw_exports);
                let export_js = #current_crate::javascript::ExportJavascript::new(exports, env);

                #fn_ident(export_js)
            }

            napi::bindgen_prelude::register_module_exports(register)
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

    // Generate the field wrappers
    let export_fields = fields
        .into_iter()
        .filter(|field| field.public)
        .map(|field| {
            let name = field.name.clone();
            let name_as_string = field.name.to_string();

            quote! {
                #current_crate::introspect::FieldInfo {
                    name: #name_as_string.to_string(),
                    getter: std::rc::Rc::new(|__this| {
                        use #current_crate::convert::FruityInto;
                        let __this = #current_crate::utils::introspect::cast_introspect_ref::<#struct_name>(__this)?;
                        __this.#name.fruity_into()
                    }),
                    setter: #current_crate::introspect::SetterCaller::None,
                },
            }
        })
        .collect::<Vec<_>>();

    // Generate the method wrappers
    let export_methods = methods
        .into_iter()
        .filter(|method| matches!(method.attrs.iter().filter_map(|attr| attr.path.get_ident()).find(|attr_ident| attr_ident.to_string() == "export"), Some(..)))
        .map(|method| {
            let name = method.name.clone();
            let name_as_string = method.name.to_string();

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

            let arg_names = method.args.iter().map(|arg| arg.name.clone()).collect::<Vec<_>>();
            quote! {
                #current_crate::introspect::MethodInfo {
                    name: #name_as_string.to_string(),
                    call: #current_crate::introspect::MethodCaller::Mut(std::rc::Rc::new(|__this, __args| {
                        use #current_crate::convert::FruityInto;
                        let __this = #current_crate::utils::introspect::cast_introspect_mut::<#struct_name>(__this)?;
        
                        #type_cast

                        let __result = __this.#name(
                            #(#arg_names),*
                        );
        
                        Ok(__result.fruity_into()?)
                    })),
                },
            }
        })
        .collect::<Vec<_>>();

    // Generate the impl for introspection
    let impl_introspect_object = quote! {

        impl #current_crate::introspect::IntrospectObject for #struct_name {
            fn get_class_name(&self) -> String {
                #struct_name_as_string.to_string()
            }

            fn get_method_infos(&self) -> Vec<#current_crate::introspect::MethodInfo> {
                vec![
                    #(#export_methods)*
                ]
            }

            fn get_field_infos(&self) -> Vec<#current_crate::introspect::FieldInfo> {
                vec![
                    #(#export_fields)*
                ]
            }
        }
    };


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