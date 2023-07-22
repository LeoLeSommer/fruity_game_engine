use convert_case::{Case, Casing};
use fruity_game_engine_code_parser::{parse_fruity_exports, FruityExport};
use std::io::Write;
use std::{fs::File, path::Path};

pub struct GenJsArgs {
    pub input: String,
    pub output: String,
}

pub fn gen_js(args: GenJsArgs) {
    let input_path = Path::new(&args.input);

    // This is our tokenized version of Rust file ready to process
    let input_syntax: syn::File = crate::syn_inline_mod::parse_and_inline_modules(&input_path);

    // Parse the input items
    let exports = parse_fruity_exports(input_syntax.items);

    // Generate the js file
    let mut file = File::create(args.output).unwrap();
    file.write_all(b"import { getBundle } from \"fruity_game_engine\";\n")
        .unwrap();

    // Write functions
    exports.iter().for_each(|export| match export {
        FruityExport::ExternImports(_) => (),
        FruityExport::Raw(_) => (),
        FruityExport::Enum(_) => (),
        FruityExport::Fn(function) => {
            let name = function
                .name_overwrite
                .clone()
                .unwrap_or(function.name.get_ident().unwrap().clone())
                .to_string()
                .to_case(Case::Camel);

            file.write_all(
                format!(
                    "\nexport function {}(...args) {{\n  return getBundle().{}(...args)\n}}\n",
                    &name, &name
                )
                .as_bytes(),
            )
            .unwrap();
        }
        FruityExport::Class(_) => (),
    });

    // Write constructors
    exports.iter().for_each(|export| match export {
        FruityExport::ExternImports(_) => (),
        FruityExport::Raw(_) => (),
        FruityExport::Enum(_) => (),
        FruityExport::Fn(_) => (),
        FruityExport::Class(class) => {
            if let Some(_) = class.constructor {
                let name = class
                    .name_overwrite
                    .clone()
                    .unwrap_or(class.name.clone())
                    .to_string();

                file.write_all(
                    format!(
                        "\nexport function {}(...args) {{\n  return getBundle().{}(...args)\n}}\n",
                        &name, &name
                    )
                    .as_bytes(),
                )
                .unwrap();

                file.write_all(
                    format!(
                        "\n{}.fruityGetType = function() {{\n  return getBundle().{}_getType()\n}}\n",
                        &name, &name
                    )
                    .as_bytes(),
                )
                .unwrap();
            }
        }
    });
}
