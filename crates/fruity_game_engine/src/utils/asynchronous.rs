use std::{future::Future, pin::Pin};

#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Builder;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

/// Wait synchronously for the end of a future
pub fn block_on(future: Pin<Box<dyn Future<Output = ()>>>) {
    #[cfg(not(target_arch = "wasm32"))]
    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    #[cfg(target_arch = "wasm32")]
    spawn_local(future);
}
