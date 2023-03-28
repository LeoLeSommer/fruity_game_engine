use fruity_game_engine_build::FruityBuildArgs;

#[cfg(not(target_arch = "wasm32"))]
extern crate napi_build;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    napi_build::setup();
    fruity_game_engine_build::fruity_build_with_args(FruityBuildArgs {
        js_file: None,
        ..Default::default()
    });
}

#[cfg(target_arch = "wasm32")]
fn main() {
    fruity_game_engine_build::fruity_build_with_args(FruityBuildArgs {
        js_file: None,
        ..Default::default()
    });
}
