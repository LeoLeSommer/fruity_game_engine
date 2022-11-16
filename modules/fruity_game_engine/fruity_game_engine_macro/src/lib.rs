use syn::Item;
use syn::__private::TokenStream2;
use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

fn current_crate() -> TokenStream2 {
    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap();
    
    if crate_name == "fruity_game_engine" {
        quote! { crate }
    } else {
        quote! { ::fruity_game_engine }
    }
}

#[proc_macro_attribute]
pub fn fruity_module_export(
    _attr: TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(item as syn_mid::ItemFn);

    let attrs = &input.attrs;
    let vis = &input.vis;
    let sig = &input.sig;
    let block = &input.block;
    let name = &sig.ident;
    let current_crate = current_crate();

    quote::quote!(
        #(#attrs) *
        #vis #sig {
            #[no_mangle]
            unsafe extern "C" fn napi_register_module_v1(
                env: ::neon::macro_internal::runtime::raw::Env,
                m: ::neon::macro_internal::runtime::raw::Local,
            ) -> ::neon::macro_internal::runtime::raw::Local {
                ::neon::macro_internal::initialize_module(
                    env,
                    ::std::mem::transmute(m),
                    |cx: ::neon::prelude::ModuleContext| -> ::neon::result::NeonResult<()> {
                        let mut ctx = #current_crate::javascript::JavascriptContext::new(cx);

                        #name(ctx)?;

                        Ok(())
                    },
                );
    
                m
            }

            #block
        }
    )
    .into()
}

#[proc_macro_derive(FruityAny)]
pub fn derive_fruity_any(input: TokenStream) -> TokenStream {
    let DeriveInput {
        ident, generics, ..
    } = parse_macro_input!(input);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();
    let current_crate = current_crate();

    let output = quote! {
        impl #impl_generics #current_crate::any::FruityAny for #ident #ty_generics #where_clause {
            fn as_any_ref(&self) -> &dyn std::any::Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self
            }

            fn as_any_box(self: Box<Self>) -> Box<dyn std::any::Any> {
                self
            }

            fn as_any_arc(self: std::sync::Arc<Self>) -> std::sync::Arc<dyn std::any::Any + Send + Sync> {
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
        }
    };

    output.into()
}

#[proc_macro]
pub fn fruity_export_class(input: TokenStream) -> TokenStream {
    let input_2 = input.clone();
    let item: Item = parse_macro_input!(input_2);
    
    eprintln!("####################################");
    eprintln!("####################################");
    eprintln!("####################################");
    eprintln!("####################################");
    eprintln!("####################################");
    eprintln!("####################################");
    eprintln!("####################################");
    
    match item {
        Item::Fn(_) => {
            eprintln!("fn");
            Some(())
        },
        Item::Struct(_) => {
            eprintln!("struct");
            None
        },
        Item::Enum(_) => {
            eprintln!("enum");
            None
        },
        Item::Const(_) => {
            eprintln!("const");
            None
        },
        Item::Impl(_) => {
            eprintln!("impl");
            None
        },
        Item::Mod(mod_) => {
            eprintln!("mod");
            None
        },
        _ => None,
    };

    input
}

/*#[proc_macro_attribute]
pub fn javascript_impl(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let input_2 = input.clone();
    let ItemImpl { generics, self_ty, items, .. } = parse_macro_input!(input_2);
    let current_crate = current_crate();

    eprintln!("ICICICICICI");

    let method_ouputs = items.into_iter().filter_map(|item| match item {
        ImplItem::Method(method) => {
            let is_visible = matches!(method.vis, Visibility::Public(_));
            let self_arg = method.sig.inputs.clone().into_iter().find_map(|arg| match arg {
                FnArg::Receiver(arg) => {
                    Some(arg)
                },
                FnArg::Typed(_) => {
                    None
                }
            });

            let is_object_method = matches!(self_arg, Some(_));
            let is_mutable = if let Some(self_arg) = self_arg.clone() {
                matches!(self_arg.mutability, Some(_))
            } else {
                false
            };
            let is_self_reference = if let Some(self_arg) = self_arg {
                matches!(self_arg.reference, Some(_))
            } else {
                false
            };

            eprintln!("method {:#?} mut:{:?} vis:{:?} ome:{:?}", &method.sig.ident.to_string(), is_mutable, is_visible, is_object_method);
            if is_visible && is_object_method && is_self_reference {
                eprintln!("method {:#?} mut:{:?}", &method.sig.ident.to_string(), is_mutable);

                for arg in method.sig.inputs.into_iter() {
                    match arg {
                        FnArg::Receiver(arg) => {
                            let is_reference = matches!(arg.reference, Some(_));
                            let is_mutable = matches!(arg.mutability, Some(_));

                            eprintln!("arg self mut:{:#?} ref:{:#?}", &is_mutable, &is_reference);
                        },
                        FnArg::Typed(arg) => {
                            eprintln!("arg {:#?}", quote! { #arg }.to_string());
                        }
                    }
                }

                None as Option<()>
            } else {
                None
            }
        },
        _ => None
    }).collect::<Vec<_>>();

    let output = quote! {
        impl #generics #current_crate::neon::types::Finalize for #self_ty #generics {}

        impl #generics #current_crate::javascript::ToJavascript for #self_ty #generics {
          fn to_js<'a>(
            self,
            cx: &'a mut #current_crate::neon::prelude::FunctionContext,
          ) -> #current_crate::neon::result::NeonResult<#current_crate::neon::prelude::Handle<'a, #current_crate::neon::types::JsValue>>
          {
            use #current_crate::neon::context::Context;
            use #current_crate::neon::object::Object;
            use #current_crate::neon::prelude::Value;
        
            let result = cx.empty_object();
        
            // Store the reference to the rust object
            let boxed: #current_crate::neon::prelude::Handle<#current_crate::neon::types::JsBox<Self>> = cx.boxed(self);
            result.set(cx, "___boxed_reference", boxed)?;
        
            // Generate method wrappers
            let get_delta = #current_crate::neon::types::JsFunction::new(cx, |mut cx: #current_crate::neon::prelude::FunctionContext| -> #current_crate::neon::result::JsResult<#current_crate::neon::types::JsValue> {
              let this: #current_crate::neon::prelude::Handle<#current_crate::neon::types::JsObject> = cx.this();
              let boxed: #current_crate::neon::prelude::Handle<#current_crate::neon::types::JsBox<Self>> = this.get(&mut cx, "___boxed_reference")?;
        
              let result = boxed.get_delta();
        
              // TODO: Find a way to remove this
              let cx = unsafe {
                std::mem::transmute::<
                  &mut neon::prelude::FunctionContext,
                  &mut neon::prelude::FunctionContext,
                >(&mut cx)
              };
        
              Ok(#current_crate::javascript::ToJavascript::to_js(result, cx)?)
            })?;
        
            result.set(cx, "getDelta", get_delta)?;
        
            Ok(result.as_value(cx))
          }
        }
    };

    // eprintln!("{}", output.to_string());

    input
}*/