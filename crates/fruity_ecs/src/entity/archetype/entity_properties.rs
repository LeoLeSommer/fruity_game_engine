use crate::entity::archetype::EntityId;
use fruity_game_engine::convert::FruityFrom;

/// This store all the information that are common accross all entities
#[derive(Debug, Clone, FruityFrom, Default)]
pub struct EntityProperties {
    /// The entity id
    pub entity_id: EntityId,

    /// the entity name
    pub name: String,

    /// If false, the entity will be ignored by the systems
    pub enabled: bool,
}
