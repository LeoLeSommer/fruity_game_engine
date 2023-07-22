use std::sync::atomic::{AtomicBool, Ordering};

#[cfg(not(target_arch = "wasm32"))]
use puffin::{are_scopes_on, set_scopes_on, GlobalProfiler, ProfilerScope};

/// A server for the profiler. When it is released, the server is closed
#[cfg(not(target_arch = "wasm32"))]
pub struct ProfileServer(puffin_http::Server);

/// A server for the profiler. When it is released, the server is closed
#[cfg(target_arch = "wasm32")]
pub struct ProfileServer();

/// A profile scope, when it is released, the scope is closed
#[cfg(target_arch = "wasm32")]
pub struct ProfilerScope(String);

const IS_FIRST_FRAME: AtomicBool = AtomicBool::new(true);

#[cfg(target_arch = "wasm32")]
impl ProfilerScope {
    /// Create a new scope
    pub fn new(name: &str) -> ProfilerScope {
        web_sys::console::time_with_label(name);

        ProfilerScope(name.to_string())
    }
}

#[cfg(target_arch = "wasm32")]
impl Drop for ProfilerScope {
    fn drop(&mut self) {
        web_sys::console::time_end_with_label(&self.0);
    }
}

/// Profile a scope, you should keep the handle in the scope so it will use the drop
/// automatic system to profile the duree of the scope
pub fn intern_profile_scope(identifier: &str) -> Option<ProfilerScope> {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Safe cause identifier don't need to be static (from the doc)
        let identifier = unsafe { std::mem::transmute::<&str, &str>(identifier) };

        if are_scopes_on() {
            Some(ProfilerScope::new(identifier, "system", ""))
        } else {
            None
        }
    }

    #[cfg(target_arch = "wasm32")]
    {
        Some(ProfilerScope::new(identifier))
    }
}

/// Start profiling before the first frame
pub fn intern_profile_start() -> ProfileServer {
    #[cfg(not(target_arch = "wasm32"))]
    {
        // Run puffin server
        let server_addr = format!("0.0.0.0:{}", puffin_http::DEFAULT_PORT);
        eprintln!("Serving demo profile data on {server_addr}");

        #[cfg(not(target_arch = "wasm32"))]
        let puffin_server = puffin_http::Server::new(&server_addr).unwrap();

        // Activate puffin
        set_scopes_on(true);

        let server = ProfileServer(puffin_server);

        server
    }

    #[cfg(target_arch = "wasm32")]
    {
        ProfileServer()
    }
}

/// Start profiling a new frame
pub fn profile_new_frame() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        GlobalProfiler::lock().new_frame();
    }

    #[cfg(target_arch = "wasm32")]
    {
        if !IS_FIRST_FRAME.load(Ordering::Relaxed) {
            web_sys::console::time_end_with_label("global_frame");
            IS_FIRST_FRAME.store(false, Ordering::Relaxed);
        }

        web_sys::console::time_with_label("global_frame");
    }
}
