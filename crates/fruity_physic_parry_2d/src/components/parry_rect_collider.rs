use fruity_ecs::component::Component;
use fruity_game_engine::{any::FruityAny, export_impl, export_struct};
use nalgebra::Vector2;
use parry2d::shape::Cuboid;

#[derive(Debug, Clone, Component, FruityAny)]
#[export_struct]
pub struct ParryRectCollider {
    #[serialize_skip]
    pub(crate) shape: Cuboid,
}

impl Default for ParryRectCollider {
    fn default() -> Self {
        Self {
            shape: Cuboid::new(Vector2::new(1.0, 1.0)),
        }
    }
}

#[export_impl]
impl ParryRectCollider {}
