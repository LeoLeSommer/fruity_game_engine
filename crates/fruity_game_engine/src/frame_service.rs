use crate::{any::FruityAny, export_impl, export_struct, resource::ResourceContainer};
pub use fruity_game_engine_macro::export;
use std::fmt::Debug;

#[cfg(not(target_arch = "wasm32"))]
fn now_in_seconds() -> f64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");

    since_the_epoch.as_secs_f64()
}

#[cfg(target_arch = "wasm32")]
fn now_in_seconds() -> f64 {
    js_sys::Date::now() / 1000.0
}

/// A service for frame management
// #[cfg(target_arch = "wasm32")]
#[derive(FruityAny, Debug)]
#[export_struct]
pub struct FrameService {
    first_frame_instant: f64,
    last_frame_instant: f64,
    delta: f64,
}

#[export_impl]
// #[cfg(target_arch = "wasm32")]
impl FrameService {
    /// Returns a FrameService
    pub fn new(_resource_container: ResourceContainer) -> FrameService {
        FrameService {
            delta: 0.0,
            first_frame_instant: now_in_seconds(),
            last_frame_instant: now_in_seconds(),
        }
    }

    /// A function that needs to be called on new frame
    /// Intended to be used in the render pipeline
    pub fn begin_frame(&mut self) {
        let now = now_in_seconds();
        self.delta = now - self.last_frame_instant;
        self.last_frame_instant = now;
    }

    /// Get the time before the previous frame as seconds
    #[export]
    pub fn get_delta(&self) -> f64 {
        self.delta
    }

    /// Get the time elapsed since the app launched
    #[export]
    pub fn get_elapsed(&self) -> f64 {
        now_in_seconds() - self.first_frame_instant
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_frame_service() {
        let mut frame_service = FrameService::new(ResourceContainer::new());

        frame_service.begin_frame();
        std::thread::sleep(std::time::Duration::from_millis(100));
        frame_service.begin_frame();

        assert!(frame_service.get_delta() > 0.1);
    }
}
