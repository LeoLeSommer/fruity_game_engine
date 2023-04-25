#[cfg(target_arch = "wasm32")]
mod sync_web;

#[cfg(target_arch = "wasm32")]
pub use sync_web::*;

#[cfg(not(target_arch = "wasm32"))]
mod sync_native;

#[cfg(not(target_arch = "wasm32"))]
pub use sync_native::*;
