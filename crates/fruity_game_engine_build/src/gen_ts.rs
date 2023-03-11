use convert_case::{Case, Casing};
use fruity_game_engine_code_parser::{
    parse_fruity_exports, FruityExport, FruityExportArg, FruityExportClassField,
    FruityExportClassFieldName,
};
use std::io::Write;
use std::{fs::File, path::Path};

pub struct GenTsArgs {
    pub input: String,
    pub output: String,
}

pub fn gen_ts(args: GenTsArgs) {
    let input_path = Path::new(&args.input);

    // This is our tokenized version of Rust file ready to process
    let input_syntax: syn::File = crate::syn_inline_mod::parse_and_inline_modules(&input_path);

    // Parse the input items
    let exports = parse_fruity_exports(input_syntax.items);

    // Generate the ts file
    let mut file = File::create(args.output).unwrap();
    exports
        .into_iter()
        .for_each(|export| write_fruity_export(export, &mut file));
}

fn write_fruity_export(export: FruityExport, file: &mut File) {
    let mut exports = Vec::<String>::new();

    // Generate exports
    match export {
        FruityExport::ExternImports(extern_import) => {
            exports.push("import {\n".to_string());
            extern_import
                .imported_items
                .iter()
                .for_each(|item| exports.push(format!("  {},\n", &item.to_string())));
            exports.push(format!("}} from \"{}\"\n\n", &extern_import.package));
        }
        FruityExport::Raw(raw) => {
            exports.push(format!("export {}\n\n", &raw));
        }
        FruityExport::Enum(enumeration) => {
            if let Some(typescript_overwrite) = enumeration.typescript_overwrite {
                exports.push(format!("export {}\n", &typescript_overwrite));
            } else {
                let name = enumeration
                    .name_overwrite
                    .clone()
                    .unwrap_or(enumeration.name.clone())
                    .to_string();

                let variants_str = enumeration
                    .variants
                    .into_iter()
                    .map(|variant| format!("\"{}\"", &variant.to_string().to_case(Case::Camel),))
                    .filter(|ty| ty.as_str() != "")
                    .collect::<Vec<_>>()
                    .join(" | ");

                exports.push(format!("export type {} = {}\n", &name, &variants_str));
            }
        }
        FruityExport::Fn(function) => {
            if let Some(typescript_overwrite) = function.typescript_overwrite {
                exports.push(format!("export {}\n", &typescript_overwrite));
            } else {
                let name = function
                    .name_overwrite
                    .clone()
                    .unwrap_or(function.name.get_ident().unwrap().clone())
                    .to_string()
                    .to_case(Case::Camel);

                let args_str = generate_args_str(&function.args, "");

                let return_str = match function.return_ty {
                    syn::ReturnType::Default => "".to_string(),
                    syn::ReturnType::Type(_, ty) => {
                        format!(": {}", rust_type_to_ts_type(&ty, false, ""))
                    }
                };

                exports.push(format!(
                    "export function {}({}){}\n",
                    &name, &args_str, &return_str
                ));
            }
        }
        FruityExport::Class(class) => {
            if let Some(typescript_overwrite) = class.typescript_overwrite {
                exports.push(format!("export {}\n", &typescript_overwrite));
            } else {
                let class_name = class
                    .name_overwrite
                    .clone()
                    .unwrap_or(class.name.clone())
                    .to_string();

                let export_type = match class.constructor {
                    Some(_) => "class",
                    None => "interface",
                };

                exports.push(format!("export {} {} {{\n", &export_type, &class_name));

                let mut member_exports = Vec::<String>::new();

                // Generate field exports
                let fields_str = generate_fields_str(&class.fields, &class_name);
                exports.push(fields_str);

                // Generate constructor exports
                if let Some(constructor) = class.constructor {
                    if let Some(typescript_overwrite) = constructor.typescript_overwrite {
                        member_exports.push(typescript_overwrite);
                    } else {
                        let args_str = generate_args_str(&constructor.args, &class_name);

                        member_exports.push(format!("constructor({})", args_str));
                    }
                }

                // Generate method exports
                class.methods.into_iter().for_each(|method| {
                    if let Some(typescript_overwrite) = method.typescript_overwrite {
                        member_exports.push(typescript_overwrite);
                    } else {
                        let name = method
                            .name_overwrite
                            .clone()
                            .unwrap_or(method.name.clone())
                            .to_string()
                            .to_case(Case::Camel);

                        let args_str = generate_args_str(&method.args, &class_name);

                        let return_str = match method.return_ty {
                            syn::ReturnType::Default => "".to_string(),
                            syn::ReturnType::Type(_, ty) => {
                                format!(": {}", rust_type_to_ts_type(&ty, false, &class_name))
                            }
                        };

                        member_exports.push(format!("{}({}){}", name, args_str, return_str));
                    }
                });

                // Write member exports
                let member_exports_string = member_exports
                    .into_iter()
                    .map(|export| format!("  {}\n", &export))
                    .collect::<Vec<_>>()
                    .join("");
                exports.push(member_exports_string);
                exports.push("}\n\n".to_string());
            }
        }
    }

    // Write all exports
    file.write_all(exports.join("").as_bytes()).unwrap();
}

fn generate_args_str(args: &Vec<FruityExportArg>, self_ident: &str) -> String {
    let mut has_next_optional = true;

    let mut reversed_args = args.clone();
    reversed_args.reverse();

    let reversed_args = reversed_args
        .into_iter()
        .map(|arg| {
            let is_optional = is_rust_type_optional(&arg.ty);

            let result = if is_optional && has_next_optional {
                format!(
                    "{}?: {}",
                    &arg.name.to_string().to_case(Case::Camel),
                    rust_type_to_ts_type(&arg.ty, true, self_ident)
                )
            } else {
                has_next_optional = false;
                format!(
                    "{}: {}",
                    &arg.name.to_string().to_case(Case::Camel),
                    rust_type_to_ts_type(&arg.ty, true, self_ident)
                )
            };

            result
        })
        .collect::<Vec<_>>();

    let mut args = reversed_args;
    args.reverse();

    args.join(", ")
}

fn generate_fields_str(fields: &Vec<FruityExportClassField>, self_ident: &str) -> String {
    fields
        .iter()
        .filter(|field| field.public)
        .map(|field| {
            let is_optional = is_rust_type_optional(&field.ty);
            let field_type = rust_type_to_ts_type(&field.ty, true, self_ident);

            if is_optional {
                match &field.name {
                    FruityExportClassFieldName::Named(name) => {
                        format!(
                            "  {}?: {}",
                            &name.to_string().to_case(Case::Camel),
                            field_type
                        )
                    }
                    FruityExportClassFieldName::Unnamed(name) => {
                        format!("  {}?: {}", &name.to_string(), field_type)
                    }
                }
            } else {
                match &field.name {
                    FruityExportClassFieldName::Named(name) => {
                        format!(
                            "  {}: {}",
                            &name.to_string().to_case(Case::Camel),
                            field_type
                        )
                    }
                    FruityExportClassFieldName::Unnamed(name) => {
                        format!("  {}: {}", &name.to_string(), field_type)
                    }
                }
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
        + "\n"
}

/// arg_or_return_type should be true if arg
fn rust_type_to_ts_type(ty: &syn::Type, arg_or_return_type: bool, self_ident: &str) -> String {
    match ty {
        syn::Type::Array(arr) => {
            format!(
                "{}[]",
                rust_type_to_ts_type(&arr.elem, arg_or_return_type, self_ident)
            )
        }
        syn::Type::BareFn(_) => unimplemented!(),
        syn::Type::Group(_) => unimplemented!(),
        syn::Type::ImplTrait(_) => unimplemented!(),
        syn::Type::Infer(_) => unimplemented!(),
        syn::Type::Macro(_) => unimplemented!(),
        syn::Type::Never(_) => "void".to_string(),
        syn::Type::Paren(paren) => {
            rust_type_to_ts_type(&paren.elem, arg_or_return_type, self_ident)
        }
        syn::Type::Path(path) => {
            let last_segment = path.path.segments.iter().last().unwrap();
            let ident = format_type_ident(&last_segment.ident);
            format_type_generics(
                &ident,
                &last_segment.arguments,
                arg_or_return_type,
                self_ident,
            )
        }
        syn::Type::Ptr(ptr) => rust_type_to_ts_type(&ptr.elem, arg_or_return_type, self_ident),
        syn::Type::Reference(reference) => {
            rust_type_to_ts_type(&reference.elem, arg_or_return_type, self_ident)
        }
        syn::Type::Slice(slice) => {
            format!(
                "{}[]",
                rust_type_to_ts_type(&slice.elem, arg_or_return_type, self_ident)
            )
        }
        syn::Type::TraitObject(trait_object) => trait_object
            .bounds
            .clone()
            .into_iter()
            .filter_map(|bound| match bound {
                syn::TypeParamBound::Trait(bound) => Some(rust_type_to_ts_type(
                    &syn::Type::Path(syn::TypePath {
                        qself: None,
                        path: bound.path,
                    }),
                    arg_or_return_type,
                    self_ident,
                )),
                syn::TypeParamBound::Lifetime(_) => None,
            })
            .filter(|ty| ty.as_str() != "")
            .collect::<Vec<_>>()
            .join(" | "),
        syn::Type::Tuple(tuple) => {
            if tuple.elems.len() == 0 {
                "void".to_string()
            } else {
                format!(
                    "[{}]",
                    tuple
                        .elems
                        .iter()
                        .map(|ty| rust_type_to_ts_type(&ty, arg_or_return_type, self_ident))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        }
        syn::Type::Verbatim(_) => unimplemented!(),
        _ => unimplemented!(),
    }
}

/// Check if a syn type is Option<...>
fn is_rust_type_optional(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Path(path) => {
            let last_segment = path.path.segments.iter().last().unwrap();
            last_segment.ident.to_string() == "Option"
        }
        _ => false,
    }
}

fn format_type_ident(ident: &syn::Ident) -> String {
    let ident = ident.to_string();

    match ident.as_str() {
        "bool" => "boolean",
        "char" => "string",
        "i8" => "number",
        "i16" => "number",
        "i32" => "number",
        "i64" => "number",
        "isize" => "number",
        "u8" => "number",
        "u16" => "number",
        "u32" => "number",
        "u64" => "number",
        "usize" => "number",
        "f32" => "number",
        "f64" => "number",
        "str" => "string",
        "String" => "string",
        _ => &ident,
    }
    .to_string()
}

/// arg_or_return_type should be true if arg
fn format_type_generics(
    ident: &str,
    ab: &syn::PathArguments,
    arg_or_return_type: bool,
    self_ident: &str,
) -> String {
    match ident {
        "FruityResult" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "Send" => "".to_string(),
        "Sync" => "".to_string(),
        "Pin" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "Box" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "Future" => "Promise<unknown>".to_string(),
        "Rc" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "Arc" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "Self" => self_ident.to_string(),
        "Vec" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    format!(
                        "{}[]",
                        rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                    )
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "Range" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    format!(
                        "[{}, {}]",
                        rust_type_to_ts_type(ty, arg_or_return_type, self_ident),
                        rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                    )
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "HashSet" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    format!(
                        "{}[]",
                        rust_type_to_ts_type(ty, arg_or_return_type, self_ident)
                    )
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "HashMap" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty1) = &ab.args[0] {
                    if let syn::GenericArgument::Type(ty2) = &ab.args[1] {
                        format!(
                            "{{[key: {}]: {}}}",
                            rust_type_to_ts_type(ty1, arg_or_return_type, self_ident),
                            rust_type_to_ts_type(ty2, arg_or_return_type, self_ident)
                        )
                    } else {
                        unreachable!()
                    }
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        "Option" => {
            if let syn::PathArguments::AngleBracketed(ab) = ab {
                if let syn::GenericArgument::Type(ty) = ab.args.first().unwrap() {
                    let type_string = rust_type_to_ts_type(ty, arg_or_return_type, self_ident);

                    if arg_or_return_type {
                        format!("{type_string} | null | undefined")
                    } else {
                        format!("{type_string} | null")
                    }
                } else {
                    unreachable!()
                }
            } else {
                unreachable!()
            }
        }
        _ => match ab {
            syn::PathArguments::None => ident.to_string(),
            syn::PathArguments::AngleBracketed(ab) => {
                let generics = ab
                    .args
                    .iter()
                    .filter_map(|arg| match arg {
                        syn::GenericArgument::Lifetime(_) => None,
                        syn::GenericArgument::Type(ty) => {
                            Some(rust_type_to_ts_type(ty, arg_or_return_type, self_ident))
                        }
                        syn::GenericArgument::Const(_) => None,
                        syn::GenericArgument::Binding(_) => None,
                        syn::GenericArgument::Constraint(_) => None,
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                format!("{}<{}>", ident, generics)
            }
            syn::PathArguments::Parenthesized(parenthesized) => {
                let args = parenthesized
                    .inputs
                    .iter()
                    .map(|input| rust_type_to_ts_type(input, true, self_ident))
                    .enumerate()
                    .map(|(index, ty)| format!("arg{}: {}", index, ty))
                    .collect::<Vec<_>>()
                    .join(", ");

                let return_ty = match &parenthesized.output {
                    syn::ReturnType::Default => "void".to_string(),
                    syn::ReturnType::Type(_, ty) => rust_type_to_ts_type(&ty, false, self_ident),
                };

                format!("(({}) => {})", args, return_ty)
            }
        },
    }
}
