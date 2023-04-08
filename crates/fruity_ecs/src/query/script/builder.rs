use super::{
    params::{Tuple, With, WithEnabled, WithEntity, WithId, WithName, WithOptional, Without},
    ScriptQuery, ScriptQueryParam,
};
use crate::entity::entity_service::EntityService;
use fruity_game_engine::{
    any::FruityAny, export, export_impl, export_struct,
    resource::resource_reference::ResourceReference,
};
use std::fmt::Debug;

#[derive(FruityAny)]
#[export_struct(typescript = "interface ScriptQueryBuilder<Args extends any[] = []> {
  withEntity(): ScriptQueryBuilder<[...Args, EntityReference]>;
  withId(): ScriptQueryBuilder<[...Args, EntityId]>;
  withName(): ScriptQueryBuilder<[...Args, string]>;
  withEnabled(): ScriptQueryBuilder<[...Args, boolean]>;
  with<T>(componentIdentifier: string): ScriptQueryBuilder<[...Args, T]>;
  withOptional<T>(
    componentIdentifier: string
  ): ScriptQueryBuilder<[...Args, T | null]>;
  without(
    componentIdentifier: string
  ): ScriptQueryBuilder<[...Args, null]>;
  build(): ScriptQuery<[...Args]>
}")]
pub struct ScriptQueryBuilder {
    entity_service: ResourceReference<EntityService>,
    params: Vec<Box<dyn ScriptQueryParam>>,
}

#[export_impl]
impl ScriptQueryBuilder {
    /// Create the entity query
    pub fn new(entity_service: ResourceReference<EntityService>) -> Self {
        ScriptQueryBuilder {
            entity_service,
            params: Vec::default(),
        }
    }

    #[export]
    pub fn with_entity(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(WithEntity {}));
        query
    }

    #[export]
    pub fn with_id(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(WithId {}));
        query
    }

    #[export]
    pub fn with_name(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(WithName {}));
        query
    }

    #[export]
    pub fn with_enabled(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(WithEnabled {}));
        query
    }

    #[export]
    pub fn with(&self, component_identifier: String) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(With {
            identifier: component_identifier,
        }));
        query
    }

    #[export]
    pub fn with_optional(&self, component_identifier: String) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(WithOptional {
            identifier: component_identifier,
        }));
        query
    }

    #[export]
    pub fn without(&self, component_identifier: String) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(Without {
            identifier: component_identifier,
        }));
        query
    }

    #[export]
    pub fn build(&self) -> ScriptQuery {
        let entity_service_reader = self.entity_service.read();
        ScriptQuery::new(
            &entity_service_reader,
            Box::new(Tuple {
                params: self.params.iter().map(|param| param.duplicate()).collect(),
            }),
        )
    }
}

impl Clone for ScriptQueryBuilder {
    fn clone(&self) -> Self {
        Self {
            entity_service: self.entity_service.clone(),
            params: self.params.iter().map(|param| param.duplicate()).collect(),
        }
    }
}

impl Debug for ScriptQueryBuilder {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
