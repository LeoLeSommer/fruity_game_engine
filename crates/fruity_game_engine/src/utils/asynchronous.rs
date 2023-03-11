use std::future::Future;

#[cfg(not(target_arch = "wasm32"))]
use tokio::runtime::Builder;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

use crate::FruityResult;

/// Wait synchronously for the end of a future
pub fn block_on<F, R>(future: F) -> FruityResult<R>
where
    F: Future<Output = FruityResult<R>> + 'static,
    R: Default,
{
    #[cfg(not(target_arch = "wasm32"))]
    let result = Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(future);

    #[cfg(target_arch = "wasm32")]
    let result = {
        spawn_local(async move {
            use crate::console_err;

            match future.await {
                Ok(_) => (),
                Err(err) => console_err(&err.to_string()),
            }
        });

        Ok(R::default())
    };

    result
}
