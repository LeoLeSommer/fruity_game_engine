use crate::entity::archetype::Archetype;
use crate::entity::entity_query::script::ScriptQueryParam;
use crate::entity::entity_reference::EntityReference;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::convert::FruityInto;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::FruityResult;

#[derive(FruityAny, Clone)]
pub struct WithEntity {}

impl ScriptQueryParam for WithEntity {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> FruityResult<Vec<ScriptValue>> {
        Ok(vec![entity_reference.fruity_into()?])
    }
}

#[derive(FruityAny, Clone)]
pub struct WithId {}

impl ScriptQueryParam for WithId {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> FruityResult<Vec<ScriptValue>> {
        let entity_reader = entity_reference.read();
        Ok(vec![entity_reader.get_entity_id().fruity_into()?])
    }
}

#[derive(FruityAny, Clone)]
pub struct WithName {}

impl ScriptQueryParam for WithName {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> FruityResult<Vec<ScriptValue>> {
        let entity_reader = entity_reference.read();
        Ok(vec![entity_reader.get_name().fruity_into()?])
    }
}

#[derive(FruityAny, Clone)]
pub struct WithEnabled {}

impl ScriptQueryParam for WithEnabled {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> FruityResult<Vec<ScriptValue>> {
        let entity_reader = entity_reference.read();
        Ok(vec![entity_reader.is_enabled().fruity_into()?])
    }
}

#[derive(FruityAny, Clone)]
pub struct With {
    pub identifier: String,
}

impl ScriptQueryParam for With {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, archetype: &Archetype) -> bool {
        archetype.identifier.contains(&self.identifier)
    }

    fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> FruityResult<Vec<ScriptValue>> {
        entity_reference
            .get_components_by_type_identifier(&self.identifier)
            .into_iter()
            .map(|component| component.fruity_into())
            .try_collect::<Vec<_>>()
    }
}

#[derive(FruityAny, Clone)]
pub struct WithOptional {
    pub identifier: String,
}

impl ScriptQueryParam for WithOptional {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam> {
        Box::new(self.clone())
    }

    fn filter_archetype(&self, _archetype: &Archetype) -> bool {
        true
    }

    fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> FruityResult<Vec<ScriptValue>> {
        let components = entity_reference
            .get_components_by_type_identifier(&self.identifier)
            .into_iter()
            .map(|component| component.fruity_into())
            .try_collect::<Vec<_>>()?;

        if components.len() > 0 {
            Ok(components)
        } else {
            Ok(vec![ScriptValue::Null])
        }
    }
}
