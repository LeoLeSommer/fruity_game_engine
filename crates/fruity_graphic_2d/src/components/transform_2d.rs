use fruity_ecs::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};
use fruity_graphic::math::matrix3::Matrix3;

#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Transform2d {
    pub transform: Matrix3,
}

#[export_impl]
impl Transform2d {
    /// Returns a new Camera
    #[export_constructor]
    pub fn new() -> Transform2d {
        Self {
            transform: Default::default(),
        }
    }
}
