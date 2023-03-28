use std::ops::Deref;

use fruity_ecs::entity::entity_reference::EntityReference;
use fruity_ecs::serializable::{Deserialize, Serialize};
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::signal::SignalProperty;
use fruity_game_engine::{export_constructor, export_impl, export_struct, FruityResult};

/// A component for an entity that as a parent or at least is part of the hierarchy
#[derive(Debug, Clone, Default, Serialize, Deserialize, FruityAny)]
#[export_struct]
pub struct Parent {
    /// The parent id
    pub parent: SignalProperty<Option<EntityReference>>,
}

impl fruity_ecs::component::Component for Parent {
    fn get_storage(
        &self,
    ) -> Box<dyn fruity_ecs::entity::archetype::component_storage::ComponentStorage> {
        Box::new(
            fruity_ecs::entity::archetype::component_storage::VecComponentStorage::<Self>::new(),
        )
    }

    fn archetype_order(&self) -> FruityResult<u8> {
        match self.parent.deref() {
            Some(parent_entity) => {
                let parent_entity_reader = parent_entity.read()?;
                let parent_reader = parent_entity_reader.get_component_by_type::<Parent>()?;
                match parent_reader.as_deref() {
                    Some(parent_reader) => Ok(parent_reader.archetype_order()? + 1),
                    None => Ok(1),
                }
            }
            None => Ok(0),
        }
    }

    fn duplicate(&self) -> Box<dyn fruity_ecs::component::Component> {
        Box::new(self.clone())
    }
}

impl fruity_ecs::component::StaticComponent for Parent {
    fn get_component_name() -> &'static str {
        "Parent"
    }
}

#[export_impl]
impl Parent {
    /// Returns a new Parent
    #[export_constructor]
    pub fn new() -> Parent {
        Self::default()
    }
}
