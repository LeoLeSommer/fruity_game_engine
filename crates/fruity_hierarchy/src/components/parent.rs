use fruity_ecs::{
    entity::EntityReference,
    serialization::{Deserialize, Serialize},
};
use fruity_game_engine::{
    any::FruityAny, export_constructor, export_impl, export_struct, script_value::ScriptObjectType,
    signal::SignalProperty, FruityResult,
};
use std::ops::Deref;

/// A component for an entity that as a parent or at least is part of the hierarchy
#[derive(Debug, Clone, Default, Serialize, Deserialize, FruityAny)]
#[export_struct]
pub struct Parent {
    /// The parent id
    pub parent: SignalProperty<Option<EntityReference>>,
}

impl fruity_ecs::component::Component for Parent {
    fn duplicate(&self) -> Box<dyn fruity_ecs::component::Component> {
        Box::new(self.clone())
    }

    fn get_component_type_id(&self) -> FruityResult<fruity_ecs::component::ComponentTypeId> {
        let order = match self.parent.deref() {
            Some(parent_entity) => {
                let parent_entity_reader = parent_entity.read()?;
                let parent_reader = parent_entity_reader.get_component_by_type::<Parent>();
                match parent_reader.as_deref() {
                    Some(parent_reader) => match parent_reader.get_component_type_id()? {
                        fruity_ecs::component::ComponentTypeId::Normal(_) => Ok(1),
                        fruity_ecs::component::ComponentTypeId::OrderedRust(_, count) => {
                            Ok(count + 1)
                        }
                    },
                    None => Ok(1),
                }
            }
            None => Ok(0),
        }?;

        Ok(fruity_ecs::component::ComponentTypeId::OrderedRust(
            ScriptObjectType::Rust(std::any::TypeId::of::<Self>()),
            order,
        ))
    }

    fn get_storage(&self) -> Box<dyn fruity_ecs::component::ComponentStorage> {
        Box::new(fruity_ecs::component::VecComponentStorage::<Self>::new())
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
