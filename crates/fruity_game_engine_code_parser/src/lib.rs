#![warn(missing_docs)]

//! Code parser
//!
//! A utility used to parse code structure
//!
//! This is widely used by two other crates, fruity_game_engine_macro and fruity_game_engine_build

use itertools::Itertools;
use proc_macro2::Span;
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use quote::ToTokens;
use std::collections::HashMap;

/// Args for a functional item
#[derive(Clone, Debug)]
pub struct FruityExportArg {
    /// Name
    pub name: syn::Ident,
    /// Type
    pub ty: syn::Type,
}

/// Args receiver for a method
#[derive(Clone, Debug)]
pub enum FruityExportReceiver {
    /// No receiver, static method
    None,
    /// Receiver with read-only access to this
    Const,
    /// Receiver with read-write access to this
    Mut,
}

/// An attribute like #[yeah(test = "lalala", test2 = "lololo")]
#[derive(Clone, Debug)]
pub struct FruityExportAttribute {
    /// The name, in #[yeah(test = "lalala")], it is yeah
    pub name: syn::Ident,
    /// The params given into the parenthesis
    pub params: FruityExportAttributeParameters,
}

/// A hashmap of attribute parameters, in #[yeah(test = "lalala", test2 = "lololo")], the hashmap is:
/// {
///   "test" => "lalala",
///   "test2" => "lololo"
/// }
pub type FruityExportAttributeParameters = HashMap<String, TokenStream>;

/// A function
#[derive(Clone)]
pub struct FruityExportFn {
    /// Name of the rust function
    pub name: syn::Path,
    /// You can override the name with #[export_func(name = "custom")]
    pub name_overwrite: Option<syn::Ident>,
    /// Attributes
    pub attrs: Vec<FruityExportAttribute>,
    /// Args
    pub args: Vec<FruityExportArg>,
    /// Return type
    pub return_ty: syn::ReturnType,
    /// Is async
    pub is_async: bool,
    /// Typescript overwrite, is used by the typescript build script
    /// You can use it like this #[export_func(typescript = "type = any")]
    pub typescript_overwrite: Option<String>,
}

/// A class constructor
#[derive(Clone)]
pub struct FruityExportConstructor {
    /// Name of the rust function
    pub name: syn::Path,
    /// Attributes
    pub attrs: Vec<FruityExportAttribute>,
    /// Args
    pub args: Vec<FruityExportArg>,
    /// Return type
    pub return_ty: syn::ReturnType,
    /// Is async
    pub is_async: bool,
    /// Typescript overwrite, is used by the typescript build script
    /// You can use it like this #[export_func(typescript = "constructor<T>(arg: T)")]
    pub typescript_overwrite: Option<String>,
}

/// A struct field name
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FruityExportClassFieldName {
    /// Named field
    Named(syn::Ident),
    /// Unnamed field for tuple structs
    Unnamed(usize),
}

/// A struct field
#[derive(Clone, Debug)]
pub struct FruityExportClassField {
    /// Is the field public
    pub public: bool,
    /// Identifier of the field
    pub name: FruityExportClassFieldName,
    /// Type
    pub ty: syn::Type,
}

/// A class method
#[derive(Clone)]
pub struct FruityExportClassMethod {
    /// Name of the rust function
    pub name: syn::Ident,
    /// You can override the name with #[export(name = "custom")]
    pub name_overwrite: Option<syn::Ident>,
    /// Attributes
    pub attrs: Vec<FruityExportAttribute>,
    /// The receiver, correspond to &self, &mut self or nothing
    pub receiver: FruityExportReceiver,
    /// Args
    pub args: Vec<FruityExportArg>,
    /// Return type
    pub return_ty: syn::ReturnType,
    /// Is async
    pub is_async: bool,
    /// Typescript overwrite, is used by the typescript build script
    /// You can use it like this #[export_func(typescript = "method<T>(arg: T): number")]
    pub typescript_overwrite: Option<String>,
}

/// A class
#[derive(Clone)]
pub struct FruityExportClass {
    /// Name of the rust struct
    pub name: syn::Ident,
    /// You can override the name with #[export_struct(name = "custom")]
    pub name_overwrite: Option<syn::Ident>,
    /// Attributes
    pub attrs: Vec<FruityExportAttribute>,
    /// Factory
    pub constructor: Option<FruityExportConstructor>,
    /// Fields
    pub fields: Vec<FruityExportClassField>,
    /// Methods
    pub methods: Vec<FruityExportClassMethod>,
    /// Typescript overwrite, is used by the typescript build script
    /// You can use it like this #[export_func(typescript = "interface { index: number }")]
    pub typescript_overwrite: Option<String>,
}

/// An enum
#[derive(Clone)]
pub struct FruityExportEnum {
    /// Name of the rust struct
    pub name: syn::Ident,
    /// You can override the name with #[export_struct(name = "custom")]
    pub name_overwrite: Option<syn::Ident>,
    /// Attributes
    pub attrs: Vec<FruityExportAttribute>,
    /// Variants, the potential values that the enum can take
    pub variants: Vec<syn::Ident>,
    /// Typescript overwrite, is used by the typescript build script
    /// You can use it like this #[export_func(typescript = "interface { index: number }")]
    pub typescript_overwrite: Option<String>,
}

/// An extern import
#[derive(Clone)]
pub struct FruityExportExternImport {
    /// Name of the package
    pub package: String,
    /// List of the imported items
    pub imported_items: Vec<syn::Ident>,
}

/// An item exposed to the javascript API
#[derive(Clone)]
pub enum FruityExport {
    /// Typescript extern imports, defined using the function macro typescript_import!({Signal, ScriptCallback, ObserverHandler, Module} from "fruity_game_engine");
    ExternImports(FruityExportExternImport),
    /// A raw typescript definition stored into a string
    Raw(String),
    /// An enum definition
    Enum(FruityExportEnum),
    /// A function typescript definition
    Fn(FruityExportFn),
    /// A class typescript definition, if there is no constructor, it is an interface instead
    Class(FruityExportClass),
}

/// Parse all the items that are exposed to the javascript
pub fn parse_fruity_exports(items: Vec<syn::Item>) -> Vec<FruityExport> {
    let exports = items
        .into_iter()
        .map(|item| parse_item(item))
        .flatten()
        .collect::<Vec<_>>();

    merge_all_fruity_exports(exports)
}

fn parse_item(item: syn::Item) -> Vec<FruityExport> {
    let mut res1 = parse_item_typescript_attr(&item);
    let mut res2 = match item {
        syn::Item::Const(_) => vec![],
        syn::Item::Enum(item) => {
            if item.attrs.iter().any(|attr| match attr.path.get_ident() {
                Some(ident) => ident.to_string() == "export_enum",
                None => false,
            }) {
                vec![FruityExport::Enum(parse_enum_item(item))]
            } else {
                vec![]
            }
        }
        syn::Item::ExternCrate(_) => vec![],
        syn::Item::Fn(item) => {
            if item.attrs.iter().any(|attr| match attr.path.get_ident() {
                Some(ident) => ident.to_string() == "export_function",
                None => false,
            }) {
                vec![FruityExport::Fn(parse_fn_item(item))]
            } else {
                vec![]
            }
        }
        syn::Item::ForeignMod(_) => vec![],
        syn::Item::Impl(item) => {
            if item.attrs.iter().any(|attr| match attr.path.get_ident() {
                Some(ident) => ident.to_string() == "export_impl",
                None => false,
            }) {
                vec![FruityExport::Class(parse_impl_item(item))]
            } else {
                vec![]
            }
        }
        syn::Item::Macro(_) => vec![],
        syn::Item::Macro2(_) => vec![],
        syn::Item::Mod(item) => match item.content {
            Some(content) => content
                .1
                .into_iter()
                .map(|item| parse_item(item))
                .flatten()
                .collect::<Vec<_>>(),
            None => vec![],
        },
        syn::Item::Static(_) => vec![],
        syn::Item::Struct(item) => {
            // Check if the struct has the attr export_struct
            let has_export_struct = item.attrs.iter().any(|attr| match attr.path.get_ident() {
                Some(ident) => ident.to_string() == "export_struct",
                None => false,
            });

            // Check if the struct derive TryFromScriptValue or TryIntoScriptValue
            let derives = parse_derive_attr(item.attrs.clone())
                .into_iter()
                .map(|derive| derive.to_string())
                .collect::<Vec<_>>();
            let has_script_value_converter = derives.contains(&"TryFromScriptValue".to_string())
                || derives.contains(&"TryIntoScriptValue".to_string());

            if has_export_struct || has_script_value_converter {
                vec![FruityExport::Class(parse_struct_item(item))]
            } else {
                vec![]
            }
        }
        syn::Item::Trait(item) => {
            if item.attrs.iter().any(|attr| match attr.path.get_ident() {
                Some(ident) => ident.to_string() == "export_trait",
                None => false,
            }) {
                vec![FruityExport::Class(parse_trait_item(item))]
            } else {
                vec![]
            }
        }
        syn::Item::TraitAlias(_) => vec![],
        syn::Item::Type(_) => vec![],
        syn::Item::Union(_) => vec![],
        syn::Item::Use(_) => vec![],
        syn::Item::Verbatim(_) => vec![],
        _ => vec![],
    };

    res1.append(&mut res2);
    res1
}

/// Parse a function item
pub fn parse_fn_item(item: syn::ItemFn) -> FruityExportFn {
    let args = item
        .sig
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(pat_type) => Some(pat_type),
        })
        .enumerate()
        .map(|(index, input)| FruityExportArg {
            name: syn::Ident::new(&format!("__arg_{}", index), Span::call_site()),
            ty: *input.ty.clone(),
        })
        .collect::<Vec<_>>();

    let parsed_attrs = parse_attrs_item(&item.attrs);

    FruityExportFn {
        name: syn::Path {
            leading_colon: None,
            segments:
                syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::from_iter(
                    vec![syn::PathSegment::from(item.sig.ident)],
                ),
        },
        name_overwrite: parsed_attrs.name_overwrite,
        attrs: parsed_attrs.attrs,
        args,
        return_ty: item.sig.output,
        is_async: match item.sig.asyncness {
            Some(_) => true,
            None => false,
        },
        typescript_overwrite: parsed_attrs.typescript_overwrite,
    }
}

/// Parse an trait item for a struct
pub fn parse_trait_item(item: syn::ItemTrait) -> FruityExportClass {
    let name = item.ident;

    let class_functions = item
        .items
        .into_iter()
        .filter_map(|item| {
            if let syn::TraitItem::Method(method) = item {
                Some(method)
            } else {
                None
            }
        })
        .map(|item| parse_trait_method(&item))
        .collect::<Vec<_>>();

    let methods = class_functions
        .clone()
        .into_iter()
        .filter_map(|method| {
            method
                .attrs
                .iter()
                .find(|attr| attr.name == "export")
                .map(|_| method.clone())
        })
        .collect::<Vec<_>>();

    FruityExportClass {
        name,
        name_overwrite: None,
        attrs: vec![],
        constructor: None,
        fields: vec![],
        methods,
        typescript_overwrite: None,
    }
}

fn parse_trait_method(item: &syn::TraitItemMethod) -> FruityExportClassMethod {
    let name = item.sig.ident.clone();

    let receiver = match item.sig.receiver() {
        Some(syn::FnArg::Receiver(receiver)) => match receiver.reference {
            Some(_) => match receiver.mutability {
                Some(_) => FruityExportReceiver::Mut,
                None => FruityExportReceiver::Const,
            },
            None => FruityExportReceiver::None,
        },
        _ => FruityExportReceiver::None,
    };

    let args = item
        .sig
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(pat_type) => Some(pat_type),
        })
        .map(|input| FruityExportArg {
            name: match &*input.pat {
                syn::Pat::Box(_) => unimplemented!(),
                syn::Pat::Ident(ident) => ident.ident.clone(),
                syn::Pat::Lit(_) => unimplemented!(),
                syn::Pat::Macro(_) => unimplemented!(),
                syn::Pat::Or(_) => unimplemented!(),
                syn::Pat::Path(_) => unimplemented!(),
                syn::Pat::Range(_) => unimplemented!(),
                syn::Pat::Reference(_) => unimplemented!(),
                syn::Pat::Rest(_) => unimplemented!(),
                syn::Pat::Slice(_) => unimplemented!(),
                syn::Pat::Struct(_) => unimplemented!(),
                syn::Pat::Tuple(_) => unimplemented!(),
                syn::Pat::TupleStruct(_) => unimplemented!(),
                syn::Pat::Type(_) => unimplemented!(),
                syn::Pat::Verbatim(_) => unimplemented!(),
                syn::Pat::Wild(_) => unimplemented!(),
                _ => unimplemented!(),
            },
            ty: *input.ty.clone(),
        })
        .collect::<Vec<_>>();

    let parsed_attrs = parse_attrs_item(&item.attrs);

    FruityExportClassMethod {
        name,
        name_overwrite: parsed_attrs.name_overwrite,
        attrs: parsed_attrs.attrs,
        receiver,
        args,
        return_ty: item.sig.output.clone(),
        is_async: match item.sig.asyncness {
            Some(_) => true,
            None => false,
        },
        typescript_overwrite: parsed_attrs.typescript_overwrite,
    }
}

/// Parse an impl item for a struct
pub fn parse_impl_item(item: syn::ItemImpl) -> FruityExportClass {
    let self_ty = item.self_ty;
    let name = syn::Ident::new(&quote! { #self_ty }.to_string(), Span::call_site());

    let class_functions = item
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

    let constructor = class_functions
        .clone()
        .into_iter()
        .filter_map(|method| {
            method
                .attrs
                .iter()
                .find(|attr| attr.name == "export_constructor")
                .map(|_| method.clone())
        })
        .last()
        .map(|method| FruityExportConstructor {
            name: syn::Path {
                leading_colon: None,
                segments:
                    syn::punctuated::Punctuated::<syn::PathSegment, syn::token::Colon2>::from_iter(
                        vec![syn::PathSegment::from(method.name)],
                    ),
            },
            attrs: method.attrs,
            args: method.args,
            return_ty: method.return_ty,
            is_async: method.is_async,
            typescript_overwrite: method.typescript_overwrite,
        });

    let methods = class_functions
        .clone()
        .into_iter()
        .filter_map(|method| {
            method
                .attrs
                .iter()
                .find(|attr| attr.name == "export")
                .map(|_| method.clone())
        })
        .collect::<Vec<_>>();

    FruityExportClass {
        name,
        name_overwrite: None,
        attrs: vec![],
        constructor,
        fields: vec![],
        methods,
        typescript_overwrite: None,
    }
}

/// Parse an enum item
pub fn parse_enum_item(item: syn::ItemEnum) -> FruityExportEnum {
    let name = item.ident.clone();

    let variants = item
        .variants
        .iter()
        .map(|variant| variant.ident.clone())
        .collect::<Vec<_>>();

    let parsed_attrs = parse_attrs_item(&item.attrs);

    FruityExportEnum {
        name,
        name_overwrite: parsed_attrs.name_overwrite,
        attrs: parsed_attrs.attrs,
        variants,
        typescript_overwrite: parsed_attrs.typescript_overwrite,
    }
}

fn parse_impl_method(item: &syn::ImplItemMethod) -> FruityExportClassMethod {
    let name = item.sig.ident.clone();

    let receiver = match item.sig.receiver() {
        Some(syn::FnArg::Receiver(receiver)) => match receiver.reference {
            Some(_) => match receiver.mutability {
                Some(_) => FruityExportReceiver::Mut,
                None => FruityExportReceiver::Const,
            },
            None => FruityExportReceiver::None,
        },
        _ => FruityExportReceiver::None,
    };

    let args = item
        .sig
        .inputs
        .iter()
        .filter_map(|input| match input {
            syn::FnArg::Receiver(_) => None,
            syn::FnArg::Typed(pat_type) => Some(pat_type),
        })
        .map(|input| FruityExportArg {
            name: match &*input.pat {
                syn::Pat::Box(_) => unimplemented!(),
                syn::Pat::Ident(ident) => ident.ident.clone(),
                syn::Pat::Lit(_) => unimplemented!(),
                syn::Pat::Macro(_) => unimplemented!(),
                syn::Pat::Or(_) => unimplemented!(),
                syn::Pat::Path(_) => unimplemented!(),
                syn::Pat::Range(_) => unimplemented!(),
                syn::Pat::Reference(_) => unimplemented!(),
                syn::Pat::Rest(_) => unimplemented!(),
                syn::Pat::Slice(_) => unimplemented!(),
                syn::Pat::Struct(_) => unimplemented!(),
                syn::Pat::Tuple(_) => unimplemented!(),
                syn::Pat::TupleStruct(_) => unimplemented!(),
                syn::Pat::Type(_) => unimplemented!(),
                syn::Pat::Verbatim(_) => unimplemented!(),
                syn::Pat::Wild(_) => unimplemented!(),
                _ => unimplemented!(),
            },
            ty: *input.ty.clone(),
        })
        .collect::<Vec<_>>();

    let parsed_attrs = parse_attrs_item(&item.attrs);

    FruityExportClassMethod {
        name,
        name_overwrite: parsed_attrs.name_overwrite,
        attrs: parsed_attrs.attrs,
        receiver,
        args,
        return_ty: item.sig.output.clone(),
        is_async: match item.sig.asyncness {
            Some(_) => true,
            None => false,
        },
        typescript_overwrite: parsed_attrs.typescript_overwrite,
    }
}

/// Parse a struct item
pub fn parse_struct_item(item: syn::ItemStruct) -> FruityExportClass {
    let name = item.ident;
    let parsed_attrs = parse_attrs_item(&item.attrs);

    FruityExportClass {
        name,
        name_overwrite: parsed_attrs.name_overwrite,
        attrs: parsed_attrs.attrs,
        constructor: None,
        fields: parse_struct_fields(&item.fields),
        methods: vec![],
        typescript_overwrite: parsed_attrs.typescript_overwrite,
    }
}

/// Parse a struct single field
pub fn parse_struct_fields(fields: &syn::Fields) -> Vec<FruityExportClassField> {
    match fields {
        syn::Fields::Named(ref fields) => fields
            .named
            .iter()
            .filter(|field| {
                matches!(
                    field.attrs.iter().find(|attr| {
                        matches!(attr.style, syn::AttrStyle::Outer)
                            && attr.path.segments.len() == 1
                            && attr.path.segments[0].ident.to_string() == "native_only"
                    }),
                    None
                )
            })
            .map(|field| match &field.ident {
                Some(ident) => FruityExportClassField {
                    public: if let syn::Visibility::Public(_) = field.vis {
                        true
                    } else {
                        false
                    },
                    name: FruityExportClassFieldName::Named(ident.clone()),
                    ty: field.ty.clone(),
                },
                None => unimplemented!(),
            })
            .collect(),
        syn::Fields::Unnamed(ref fields) => {
            // For tuple struct, field name are numbers
            fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(index, field)| FruityExportClassField {
                    public: if let syn::Visibility::Public(_) = field.vis {
                        true
                    } else {
                        false
                    },
                    name: FruityExportClassFieldName::Unnamed(index),
                    ty: field.ty.clone(),
                })
                .collect()
        }
        syn::Fields::Unit => {
            unimplemented!()
        }
    }
}

struct ParseAttrsItemResult {
    pub attrs: Vec<FruityExportAttribute>,
    pub name_overwrite: Option<syn::Ident>,
    pub typescript_overwrite: Option<String>,
}

fn parse_attrs_item(items: &Vec<syn::Attribute>) -> ParseAttrsItemResult {
    // Parse attributes
    let attrs = items
        .into_iter()
        .filter_map(|item| {
            item.clone()
                .path
                .get_ident()
                .map(|attr_ident| FruityExportAttribute {
                    name: syn::Ident::new(&attr_ident.to_string(), Span::call_site()),
                    params: item
                        .clone()
                        .tokens
                        .into_iter()
                        .filter_map(|e| match e {
                            TokenTree::Group(group) => Some(group),
                            _ => None,
                        })
                        .find(|_| true)
                        .map(|e| e.stream())
                        .map(|e| {
                            e.into_iter()
                                .enumerate()
                                .group_by(|(index, _)| index / 3)
                                .into_iter()
                                .map(|(_, metas)| {
                                    metas
                                        .into_iter()
                                        .map(|(_, metas)| metas.into())
                                        .collect::<Vec<TokenStream>>()
                                })
                                .filter(|metas| metas.len() == 3)
                                .filter(|metas| metas[1].to_string() == "=")
                                .map(|metas| (metas[0].to_string(), metas[2].clone()))
                                .collect::<HashMap<String, TokenStream>>()
                        })
                        .unwrap_or(HashMap::new()),
                })
        })
        .collect_vec();

    // Extract common overwrites by attributes
    let export_attr = attrs
        .iter()
        .filter(|attr| {
            attr.name == "export_function"
                || attr.name == "export_impl"
                || attr.name == "export_struct"
                || attr.name == "export_constructor"
                || attr.name == "export"
        })
        .last();

    // Extract name overwrite
    let name_overwrite = if let Some(export_attr) = export_attr {
        export_attr
            .params
            .get("name")
            .map(|name| name.to_string().replace("\"", ""))
            .map(|name| syn::Ident::new(&name, Span::call_site()))
    } else {
        None
    };

    // Extract typescript overwrite
    let typescript_overwrite = if let Some(export_attr) = export_attr {
        export_attr
            .params
            .get("typescript")
            .map(|typescript| typescript.to_string().replace("\"", ""))
    } else {
        None
    };

    ParseAttrsItemResult {
        attrs,
        name_overwrite,
        typescript_overwrite,
    }
}

/// Merge two list of FruityExport
/// Combine the classes fields and methods, to merge the impl and struct informations
fn merge_all_fruity_exports(exports: Vec<FruityExport>) -> Vec<FruityExport> {
    // Extract all class names
    let mut all_class_names = exports
        .iter()
        .filter_map(|export| match export {
            FruityExport::ExternImports(_) => None,
            FruityExport::Raw(_) => None,
            FruityExport::Enum(_) => None,
            FruityExport::Fn(_) => None,
            FruityExport::Class(class) => Some(class.name.clone()),
        })
        .collect::<Vec<_>>();

    all_class_names.sort();
    all_class_names.dedup();

    // Extract all function names
    let mut all_fn_names = exports
        .iter()
        .filter_map(|export| match export {
            FruityExport::ExternImports(_) => None,
            FruityExport::Raw(_) => None,
            FruityExport::Enum(_) => None,
            FruityExport::Fn(function) => Some(function.name.to_token_stream().to_string()),
            FruityExport::Class(_) => None,
        })
        .collect::<Vec<_>>();

    all_fn_names.sort();
    all_fn_names.dedup();

    // Merge class entries
    let mut classes = all_class_names
        .into_iter()
        .map(|name| {
            let classes = exports
                .iter()
                .filter_map(|export| match export {
                    FruityExport::ExternImports(_) => None,
                    FruityExport::Raw(_) => None,
                    FruityExport::Enum(_) => None,
                    FruityExport::Fn(_) => None,
                    FruityExport::Class(export) => {
                        if export.name == name {
                            Some(export)
                        } else {
                            None
                        }
                    }
                })
                .collect::<Vec<_>>();

            // Get the class constructor
            let constructor = classes
                .iter()
                .filter_map(|class| class.constructor.clone())
                .last();

            // Get the name overwrite
            let name_overwrite = classes
                .iter()
                .filter_map(|class| class.name_overwrite.clone())
                .last();

            // Get the typescript overwrite
            let typescript_overwrite = classes
                .iter()
                .filter_map(|class| class.typescript_overwrite.clone())
                .last();

            // Get attrs of the class
            let mut attrs = classes
                .iter()
                .map(|class| class.attrs.iter())
                .flatten()
                .map(|attr| attr.clone())
                .collect::<Vec<_>>();
            attrs.dedup_by(|attr1, attr2| attr1.name == attr2.name);

            // Get fields of the class
            let mut fields = classes
                .iter()
                .map(|class| class.fields.iter())
                .flatten()
                .map(|field| field.clone())
                .collect::<Vec<_>>();
            fields.dedup_by(|field1, field2| field1.name == field2.name);

            // Get methods of the class
            let mut methods = classes
                .iter()
                .map(|class| class.methods.iter())
                .flatten()
                .map(|method| method.clone())
                .collect::<Vec<_>>();
            methods.dedup_by(|method1, method2| method1.name == method2.name);

            // Create the class object
            FruityExportClass {
                name,
                name_overwrite,
                attrs,
                constructor,
                fields,
                methods,
                typescript_overwrite,
            }
        })
        .map(|class| FruityExport::Class(class))
        .collect::<Vec<_>>();

    // Merge function entries (here this consist of just taking the first entry, cause a function should have a unique name)
    let mut functions = all_fn_names
        .into_iter()
        .filter_map(|name| {
            exports
                .iter()
                .filter_map(|export| match export {
                    FruityExport::ExternImports(_) => None,
                    FruityExport::Raw(_) => None,
                    FruityExport::Enum(_) => None,
                    FruityExport::Fn(export) => {
                        if export.name.to_token_stream().to_string() == name {
                            Some(export)
                        } else {
                            None
                        }
                    }
                    FruityExport::Class(_) => None,
                })
                .last()
        })
        .map(|export| FruityExport::Fn(export.clone()))
        .collect::<Vec<_>>();

    // Extract all extern imports
    let mut extern_imports = exports
        .iter()
        .filter_map(|export| match export {
            FruityExport::ExternImports(ex_import) => {
                Some(FruityExport::ExternImports(ex_import.clone()))
            }
            FruityExport::Raw(_) => None,
            FruityExport::Enum(_) => None,
            FruityExport::Fn(_) => None,
            FruityExport::Class(_) => None,
        })
        .collect::<Vec<_>>();

    // Extract all enum
    let mut enums = exports
        .iter()
        .filter_map(|export| match export {
            FruityExport::ExternImports(_) => None,
            FruityExport::Enum(enumeration) => Some(FruityExport::Enum(enumeration.clone())),
            FruityExport::Raw(_) => None,
            FruityExport::Fn(_) => None,
            FruityExport::Class(_) => None,
        })
        .collect::<Vec<_>>();

    // Extract all raws
    let mut raws = exports
        .iter()
        .filter_map(|export| match export {
            FruityExport::ExternImports(_) => None,
            FruityExport::Enum(_) => None,
            FruityExport::Raw(raw) => Some(FruityExport::Raw(raw.clone())),
            FruityExport::Fn(_) => None,
            FruityExport::Class(_) => None,
        })
        .collect::<Vec<_>>();

    // Return the result
    let mut result = Vec::<FruityExport>::new();
    result.append(&mut extern_imports);
    result.append(&mut raws);
    result.append(&mut enums);
    result.append(&mut classes);
    result.append(&mut functions);

    result
}

fn parse_item_typescript_attr(item: &syn::Item) -> Vec<FruityExport> {
    let attrs = match item {
        syn::Item::Const(item) => item.attrs.clone(),
        syn::Item::Enum(item) => item.attrs.clone(),
        syn::Item::ExternCrate(item) => item.attrs.clone(),
        syn::Item::Fn(item) => item.attrs.clone(),
        syn::Item::ForeignMod(item) => item.attrs.clone(),
        syn::Item::Impl(item) => item.attrs.clone(),
        syn::Item::Macro(item) => item.attrs.clone(),
        syn::Item::Macro2(item) => item.attrs.clone(),
        syn::Item::Mod(item) => item.attrs.clone(),
        syn::Item::Static(item) => item.attrs.clone(),
        syn::Item::Struct(item) => item.attrs.clone(),
        syn::Item::Trait(item) => item.attrs.clone(),
        syn::Item::TraitAlias(item) => item.attrs.clone(),
        syn::Item::Type(item) => item.attrs.clone(),
        syn::Item::Union(item) => item.attrs.clone(),
        syn::Item::Use(item) => item.attrs.clone(),
        syn::Item::Verbatim(_) => Vec::default(),
        _ => Vec::default(),
    };

    let mut res1 = attrs
        .iter()
        .filter_map(|attr| {
            attr.path
                .get_ident()
                .map(|ident| (ident.clone(), attr.clone()))
        })
        .filter(|(ident, _attr)| ident.to_string() == "typescript")
        .map(|(_, attr)| attr.tokens)
        .filter_map(|tokens| match tokens.into_iter().last().unwrap() {
            TokenTree::Group(group) => Some(group.stream()),
            TokenTree::Ident(_) => None,
            TokenTree::Punct(_) => None,
            TokenTree::Literal(_) => None,
        })
        .filter_map(|tokens| match tokens.into_iter().last().unwrap() {
            TokenTree::Group(_) => None,
            TokenTree::Ident(_) => None,
            TokenTree::Punct(_) => None,
            TokenTree::Literal(lit) => Some(rem_first_and_last(&lit.to_string()).to_string()),
        })
        .map(|typescript| FruityExport::Raw(typescript))
        .collect::<Vec<_>>();

    let mut res2 = attrs
        .iter()
        .filter_map(|attr| {
            attr.path
                .get_ident()
                .map(|ident| (ident.clone(), attr.clone()))
        })
        .filter(|(ident, _attr)| ident.to_string() == "typescript_import")
        .map(|(_, attr)| attr.tokens)
        .filter_map(|tokens| match tokens.into_iter().last().unwrap() {
            TokenTree::Group(group) => Some(group.stream().into_iter().collect::<Vec<_>>()),
            TokenTree::Ident(_) => None,
            TokenTree::Punct(_) => None,
            TokenTree::Literal(_) => None,
        })
        .filter_map(|tokens| {
            if tokens.len() != 3 {
                return None;
            }

            if let TokenTree::Ident(second_token) = &tokens[1] {
                if second_token.to_string() != "from" {
                    return None;
                }
            } else {
                return None;
            }

            let imported_items = if let TokenTree::Group(group) = &tokens[0] {
                group
                    .stream()
                    .into_iter()
                    .filter_map(|token| match token {
                        TokenTree::Group(_) => None,
                        TokenTree::Ident(ident) => Some(ident),
                        TokenTree::Punct(_) => None,
                        TokenTree::Literal(_) => None,
                    })
                    .collect::<Vec<_>>()
            } else {
                return None;
            };

            let package = if let TokenTree::Literal(lit) = &tokens[2] {
                rem_first_and_last(&lit.to_string()).to_string()
            } else {
                return None;
            };

            Some(FruityExport::ExternImports(FruityExportExternImport {
                package,
                imported_items,
            }))
        })
        .collect::<Vec<_>>();

    res1.append(&mut res2);
    res1
}

fn rem_first_and_last(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.next_back();
    chars.as_str()
}

/// Parse the derive attribute
/// For example #[derive(Clone, Default)] with result with a vector of ident
/// containing Clone and Default
pub fn parse_derive_attr(attrs: Vec<syn::Attribute>) -> Vec<syn::Ident> {
    attrs
        .into_iter()
        .filter_map(|attr| {
            attr.path
                .get_ident()
                .map(|ident| (ident.clone(), attr.clone()))
        })
        .filter(|(ident, _attr)| ident.to_string() == "derive")
        .map(|(_, attr)| attr.tokens)
        .filter_map(|tokens| match tokens.into_iter().last().unwrap() {
            TokenTree::Group(group) => Some(group.stream()),
            TokenTree::Ident(_) => None,
            TokenTree::Punct(_) => None,
            TokenTree::Literal(_) => None,
        })
        .map(|tokens| {
            tokens.into_iter().filter_map(|tokens| match tokens {
                TokenTree::Group(_) => None,
                TokenTree::Ident(ident) => Some(ident),
                TokenTree::Punct(_) => None,
                TokenTree::Literal(_) => None,
            })
        })
        .flatten()
        .collect::<Vec<_>>()
}
