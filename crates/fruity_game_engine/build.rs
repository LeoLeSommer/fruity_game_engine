#[cfg(feature = "napi-module")]
extern crate napi_build;

#[cfg(feature = "napi-module")]
fn main() {
    fruity_game_engine_build::fruity_build_with_args(fruity_game_engine_build::FruityBuildArgs {
        js_file: None,
        ..Default::default()
    });
    napi_build::setup();
}

#[cfg(not(feature = "napi-module"))]
fn main() {
    fruity_game_engine_build::fruity_build_with_args(fruity_game_engine_build::FruityBuildArgs {
        js_file: None,
        ..Default::default()
    });
}
