#[cfg(feature = "napi-module")]
extern crate napi_build;

#[cfg(feature = "napi-module")]
fn main() {
  napi_build::setup();
}

#[cfg(not(feature = "napi-module"))]
fn main() {}
