pub use puffin::profile_function;
use puffin::{are_scopes_on, ProfilerScope};

/// Profile a scope, you should keep the handle in the scope so it will use the drop
/// automatic system to profile the duree of the scope
pub fn profile_scope(identifier: &str) {
    // Safe cause identifier don't need to be static (from the doc)
    let identifier = unsafe { std::mem::transmute::<&str, &str>(identifier) };

    if are_scopes_on() {
        ProfilerScope::new(identifier, "system", "");
    }
}

/// Start profiling before the first frame
pub fn profile_start() {
    puffin::set_scopes_on(true);
}

/// Start profiling a new frame
pub fn profile_new_frame() {
    puffin::GlobalProfiler::lock().new_frame();
}
