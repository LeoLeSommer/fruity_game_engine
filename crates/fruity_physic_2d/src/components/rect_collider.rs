use fruity_ecs::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};
use fruity_graphic::math::vector2d::Vector2d;

#[derive(Debug, Clone, Component, FruityAny)]
#[export_struct]
pub struct RectCollider {
    pub bottom_left: Vector2d,
    pub top_right: Vector2d,
}

impl Default for RectCollider {
    fn default() -> Self {
        Self {
            bottom_left: Vector2d::new(-0.5, -0.5),
            top_right: Vector2d::new(0.5, 0.5),
        }
    }
}

#[export_impl]
impl RectCollider {
    /// Returns a new RectCollider
    #[export_constructor]
    pub fn new(bottom_left: Vector2d, top_right: Vector2d) -> RectCollider {
        Self {
            bottom_left,
            top_right,
        }
    }
}
