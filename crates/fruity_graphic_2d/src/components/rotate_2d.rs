use fruity_ecs::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};

#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Rotate2d {
    pub angle: f32,
}

#[export_impl]
impl Rotate2d {
    /// Returns a new Camera
    #[export_constructor]
    pub fn new(angle: f32) -> Rotate2d {
        Self { angle }
    }
}
