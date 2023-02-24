use std::{future::Future, pin::Pin};

#[cfg(not(any(feature = "napi-module", feature = "wasm-module")))]
use tokio::runtime::Builder;

#[cfg(feature = "napi-module")]
use tokio::runtime::Builder;

#[cfg(feature = "wasm-module")]
use wasm_bindgen_futures::spawn_local;

/// Wait synchronously for the end of a future
pub fn block_on(future: Pin<Box<dyn Future<Output = ()>>>) {
    #[cfg(not(any(feature = "napi-module", feature = "wasm-module")))]
    Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    #[cfg(feature = "napi-module")]
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    #[cfg(feature = "wasm-module")]
    spawn_local(future);
}
