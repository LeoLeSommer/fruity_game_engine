use fruity_ecs_macro::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};

/// A component to enable or disable an entity
#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Enabled(pub bool);

#[export_impl]
impl Enabled {
    /// Returns a new RectCollider
    #[export_constructor]
    pub fn new(enabled: bool) -> Enabled {
        Self(enabled)
    }
}
