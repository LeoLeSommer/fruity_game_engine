pub use puffin::profile_function;
use puffin::{are_scopes_on, ProfilerScope};

/// Profile a scope, you should keep the handle in the scope so it will use the drop
/// automatic system to profile the duree of the scope
pub fn profile_scope(identifier: &str) -> Result<Option<ProfilerScope>, ()> {
    // Safe cause identifier don't need to be static (from the doc)
    let identifier = unsafe { std::mem::transmute::<&str, &str>(identifier) };

    let profiler_scope = if are_scopes_on() {
        Some(ProfilerScope::new(identifier, "system", ""))
    } else {
        None
    };

    Ok(profiler_scope)
}
