use fruity_ecs::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};

#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Rotate2D {
    pub angle: f32,
}

#[export_impl]
impl Rotate2D {
    /// Returns a new Rotate2D
    #[export_constructor]
    pub fn new(angle: f32) -> Rotate2D {
        Self { angle }
    }
}
