#![allow(missing_docs)]

#[cfg(feature = "napi-module")]
pub mod napi;

#[cfg(feature = "napi-module")]
pub use crate::javascript::napi::*;

#[cfg(not(feature = "napi-module"))]
pub mod none;

#[cfg(not(feature = "napi-module"))]
pub use crate::javascript::none::*;