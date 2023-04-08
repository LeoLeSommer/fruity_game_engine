use crate::component::Component;
use fruity_game_engine::{any::FruityAny, export_constructor, export_impl, export_struct};

/// A component to name an entity
#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Name(pub String);

#[export_impl]
impl Name {
    /// Returns a new RectCollider
    #[export_constructor]
    pub fn new(string: String) -> Name {
        Self(string)
    }
}
