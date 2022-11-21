use crate::any::FruityAny;
use crate::fruity_export;
use crate::resource::resource_container::ResourceContainer;
use crate::resource::Resource;
pub use fruity_game_engine_macro::export;
use std::fmt::Debug;
use std::time::Instant;

fruity_export! {
    /// A service for frame management
    #[derive(FruityAny, Resource)]
    pub struct FrameService {
        last_frame_instant: Instant,
        /// delelelel
        pub delta: f32,
    }

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

        /// Get the time before the previous frame
        #[export(name = "patate")]
        pub fn set_delta(&mut self, value: f32) {
            self.delta = value;
        }
    }
}

impl Debug for FrameService {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    Ok(())
  }
}
