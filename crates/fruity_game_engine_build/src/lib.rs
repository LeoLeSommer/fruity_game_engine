use gen_js::GenJsArgs;
use gen_ts::GenTsArgs;
use npm_rs::{NodeEnv, NpmEnv};
mod gen_js;
mod gen_ts;
mod syn_inline_mod;

pub struct FruityBuildArgs {
    pub input: String,
    pub js_file: String,
    pub ts_file: String,
}

impl Default for FruityBuildArgs {
    fn default() -> Self {
        Self {
            input: "src/lib.rs".to_string(),
            js_file: "index.js".to_string(),
            ts_file: "index.d.ts".to_string(),
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

    gen_js::gen_js(GenJsArgs {
        input: args.input.clone(),
        output: args.js_file.clone(),
    });
    gen_ts::gen_ts(GenTsArgs {
        input: args.input.clone(),
        output: args.ts_file.clone(),
    });
}
