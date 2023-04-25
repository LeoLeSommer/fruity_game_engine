use fruity_ecs::component::Component;
use fruity_game_engine::{any::FruityAny, export_impl, export_struct};
use parry2d::shape::Ball;

#[derive(Debug, Clone, Component, FruityAny)]
#[export_struct]
pub struct ParryCircleCollider {
    #[serialize_skip]
    pub(crate) shape: Ball,
}

impl Default for ParryCircleCollider {
    fn default() -> Self {
        Self {
            shape: Ball::new(1.0),
        }
    }
}

#[export_impl]
impl ParryCircleCollider {}
