use crate::any::FruityAny;
use crate::resource::resource_container::ResourceContainer;
use crate::resource::Resource;
use crate::{export_impl, export_struct};
pub use fruity_game_engine_macro::export;
use std::fmt::Debug;
use std::time::Instant;

/// A service for frame management
#[derive(FruityAny, Resource, Debug)]
#[export_struct]
pub struct FrameService {
    last_frame_instant: Instant,
    delta: f32,
}

#[export_impl]
impl FrameService {
    /// Returns a FrameService
    pub fn new(_resource_container: ResourceContainer) -> FrameService {
        FrameService {
            delta: 0.0,
            last_frame_instant: Instant::now(),
        }
    }

    /// A function that needs to be called on new frame
    /// Intended to be used in the render pipeline
    pub fn begin_frame(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame_instant);
        self.delta = delta.as_secs_f32();
        self.last_frame_instant = now;
    }

    /// Get the time before the previous frame
    #[export]
    pub fn get_delta(&self) -> f32 {
        self.delta
    }
}
