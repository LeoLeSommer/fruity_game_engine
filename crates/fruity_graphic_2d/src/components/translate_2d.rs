use fruity_ecs::component::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};
use fruity_graphic::math::vector2d::Vector2D;

#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Translate2D {
    pub vec: Vector2D,
}

#[export_impl]
impl Translate2D {
    /// Returns a new Camera
    #[export_constructor]
    pub fn new(vec: Vector2D) -> Translate2D {
        Self { vec }
    }
}
