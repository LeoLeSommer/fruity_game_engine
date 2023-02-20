use fruity_ecs::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};
use fruity_graphic::math::vector2d::Vector2d;

#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Translate2d {
    pub vec: Vector2d,
}

#[export_impl]
impl Translate2d {
    /// Returns a new Camera
    #[export_constructor]
    pub fn new(vec: Vector2d) -> Translate2d {
        Self { vec }
    }
}
