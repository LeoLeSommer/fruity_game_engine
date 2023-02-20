#[cfg(feature = "napi-module")]
extern crate napi_build;

#[cfg(feature = "napi-module")]
fn main() {
    fruity_game_engine_build::fruity_build();
    napi_build::setup();
}

#[cfg(not(feature = "napi-module"))]
fn main() {
    fruity_game_engine_build::fruity_build();
}
