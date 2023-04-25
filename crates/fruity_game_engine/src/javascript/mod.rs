#![allow(missing_docs)]

use fruity_game_engine_macro::typescript;

#[typescript("type JsIntrospectObject = { [key: string]: any }")]
#[allow(dead_code)]
type JsIntrospectObjectTypescriptDecl = usize;

#[cfg(target_arch = "wasm32")]
mod wasm;

#[cfg(target_arch = "wasm32")]
pub use wasm::*;

#[cfg(not(target_arch = "wasm32"))]
mod napi_script;

#[cfg(not(target_arch = "wasm32"))]
pub use napi_script::*;
