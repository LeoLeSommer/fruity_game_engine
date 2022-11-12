use crate::any::FruityAny;
use crate::resource::resource_container::ResourceContainer;
use crate::resource::Resource;
use napi_derive::napi;
use std::fmt::Debug;
use std::time::Instant;

/// A service for frame management
#[derive(FruityAny, Resource)]
#[napi]
pub struct FrameService {
  last_frame_instant: Instant,
  delta: f32,
}

impl Debug for FrameService {
  fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
    Ok(())
  }
}

#[napi]
impl FrameService {
  /// Returns a FrameService
  pub fn new(_resource_container: ResourceContainer) -> FrameService {
    FrameService {
      delta: 0.0,
      last_frame_instant: Instant::now(),
    }
  }

  /// Get the time before the previous frame
  #[napi]
  pub fn get_delta(&self) -> f32 {
    self.delta
  }

  /// A function that needs to be called on new frame
  /// Intended to be used in the render pipeline
  pub fn begin_frame(&mut self) {
    let now = Instant::now();
    let delta = now.duration_since(self.last_frame_instant);

    self.delta = delta.as_secs_f32();
    self.last_frame_instant = now;
  }
}
