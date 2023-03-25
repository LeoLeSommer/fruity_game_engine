#[cfg(not(target_arch = "wasm32"))]
use puffin::{are_scopes_on, ProfilerScope};

/// Profile a scope, you should keep the handle in the scope so it will use the drop
/// automatic system to profile the duree of the scope
#[cfg(not(target_arch = "wasm32"))]
pub fn intern_profile_scope(identifier: &str) -> Option<ProfilerScope> {
    // Safe cause identifier don't need to be static (from the doc)
    let identifier = unsafe { std::mem::transmute::<&str, &str>(identifier) };

    if are_scopes_on() {
        Some(ProfilerScope::new(identifier, "system", ""))
    } else {
        None
    }
}

/// Start profiling before the first frame
#[cfg(not(target_arch = "wasm32"))]
pub fn profile_start() {
    puffin::set_scopes_on(true);
}

/// Start profiling a new frame
#[cfg(not(target_arch = "wasm32"))]
pub fn profile_new_frame() {
    puffin::GlobalProfiler::lock().new_frame();
}

/// Profile a scope, you should keep the handle in the scope so it will use the drop
/// automatic system to profile the duree of the scope
#[cfg(target_arch = "wasm32")]
pub fn intern_profile_scope(_: &str) {}

/// Start profiling before the first frame
#[cfg(target_arch = "wasm32")]
pub fn profile_start() {}

/// Start profiling a new frame
#[cfg(target_arch = "wasm32")]
pub fn profile_new_frame() {}
