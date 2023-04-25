use fruity_game_engine_build::FruityBuildArgs;

#[cfg(not(target_arch = "wasm32"))]
extern crate napi_build;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    fruity_game_engine_build::fruity_build_with_args(FruityBuildArgs {
        js_file: None,
        ..Default::default()
    });
    napi_build::setup();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    fruity_game_engine_build::fruity_build_with_args(FruityBuildArgs {
        js_file: None,
        ..Default::default()
    });
}
