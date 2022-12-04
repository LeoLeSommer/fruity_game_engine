#![allow(missing_docs)]

#[cfg(feature = "wasm-module")]
pub mod wasm;

#[cfg(feature = "wasm-module")]
pub use crate::javascript::wasm::*;

#[cfg(feature = "napi-module")]
pub mod napi;

#[cfg(feature = "napi-module")]
pub use crate::javascript::napi::*;

#[cfg(not(any(feature = "napi-module", feature = "wasm-module")))]
pub mod none;

#[cfg(not(any(feature = "napi-module", feature = "wasm-module")))]
pub use crate::javascript::none::*;
