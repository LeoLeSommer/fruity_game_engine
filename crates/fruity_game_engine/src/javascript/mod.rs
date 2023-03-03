#![allow(missing_docs)]

use fruity_game_engine_macro::typescript;

// TODO: Remove it
#[typescript("type JsIntrospectObject = { [key: string]: any }")]
#[allow(dead_code)]
struct JsIntrospectObjectTypescriptDecl {}

#[cfg(target_arch = "wasm32")]
pub mod wasm;

#[cfg(target_arch = "wasm32")]
pub use crate::javascript::wasm::*;

#[cfg(not(target_arch = "wasm32"))]
pub mod napi;

#[cfg(not(target_arch = "wasm32"))]
pub use crate::javascript::napi::*;
