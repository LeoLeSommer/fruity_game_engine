use fruity_ecs::entity::entity::EntityId;
use fruity_ecs::Component;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::fruity_export;
use fruity_game_engine::signal::SignalProperty;

fruity_export! {
    /// A component for an entity that as a parent or at least is part of the hierarchy
    #[derive(Debug, Clone, Default, Component, FruityAny)]
    pub struct Parent {
        /// The parent id
        pub parent_id: SignalProperty<Option<EntityId>>,

        /// The nested level of a hierarchy component
        /// It's mainly used to update the position in cascade cause the position of
        /// a child must be updated after the parent
        pub nested_level: usize,
    }
}
