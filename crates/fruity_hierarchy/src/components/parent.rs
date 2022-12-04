use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::Component;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::signal::SignalProperty;
use fruity_game_engine::{export_constructor, export_impl, export_struct};

/// A component for an entity that as a parent or at least is part of the hierarchy
#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Parent {
    /// The parent id
    pub parent_id: SignalProperty<Option<EntityId>>,

    /// The nested level of a hierarchy component
    /// It's mainly used to update the position in cascade cause the position of
    /// a child must be updated after the parent
    pub nested_level: usize,
}

#[export_impl]
impl Parent {
    /// Returns a new Parent
    #[export_constructor]
    pub fn new() -> Parent {
        Self::default()
    }
}
