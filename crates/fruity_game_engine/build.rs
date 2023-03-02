#[cfg(not(target_arch = "wasm32"))]
extern crate napi_build;

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    fruity_game_engine_build::fruity_build();
    napi_build::setup();
}

#[cfg(target_arch = "wasm32")]
fn main() {
    fruity_game_engine_build::fruity_build();
}
