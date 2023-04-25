use fruity_ecs::component::Component;
use fruity_game_engine::{any::FruityAny, export_impl, export_struct};

#[derive(Debug, Default, Clone, Component, FruityAny)]
#[export_struct]
pub struct RigidBody {}

#[export_impl]
impl RigidBody {}
