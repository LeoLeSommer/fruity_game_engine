use fruity_ecs::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};
use fruity_graphic::math::vector2d::Vector2d;

#[derive(Debug, Clone, Component, FruityAny)]
#[export_struct]
pub struct CircleCollider {
    pub center: Vector2d,
    pub radius: f32,
}

impl Default for CircleCollider {
    fn default() -> Self {
        Self {
            center: Vector2d::new(0.0, 0.0),
            radius: 1.0,
        }
    }
}

#[export_impl]
impl CircleCollider {
    /// Returns a new RectCollider
    #[export_constructor]
    pub fn new(center: Vector2d, radius: f32) -> CircleCollider {
        Self { center, radius }
    }
}
