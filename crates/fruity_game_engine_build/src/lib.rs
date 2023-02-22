use gen_js::GenJsArgs;
use gen_ts::GenTsArgs;
use npm_rs::{NodeEnv, NpmEnv};
mod gen_js;
mod gen_ts;
mod syn_inline_mod;

pub struct FruityBuildArgs {
    pub input: String,
    pub js_file: Option<String>,
    pub ts_file: Option<String>,
}

impl Default for FruityBuildArgs {
    fn default() -> Self {
        Self {
            input: "src/lib.rs".to_string(),
            js_file: Some("index.js".to_string()),
            ts_file: Some("index.d.ts".to_string()),
        }
    }
}

pub fn fruity_build() {
    fruity_build_with_args(FruityBuildArgs::default())
}

pub fn fruity_build_with_args(args: FruityBuildArgs) {
    NpmEnv::default()
        .with_node_env(&NodeEnv::from_cargo_profile().unwrap_or_default())
        .init_env()
        .install(None)
        .exec()
        .unwrap();

    if let Some(js_file) = &args.js_file {
        gen_js::gen_js(GenJsArgs {
            input: args.input.clone(),
            output: js_file.clone(),
        });
    }

    if let Some(ts_file) = &args.ts_file {
        gen_ts::gen_ts(GenTsArgs {
            input: args.input.clone(),
            output: ts_file.clone(),
        });
    }
}
