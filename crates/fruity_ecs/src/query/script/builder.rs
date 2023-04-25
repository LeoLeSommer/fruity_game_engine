use super::{
    ScriptQuery, ScriptQueryParam, ScriptTuple, ScriptWith, ScriptWithEnabled,
    ScriptWithEntityReference, ScriptWithId, ScriptWithName, ScriptWithOptional, ScriptWithout,
};
use crate::entity::{EntityId, EntityLocation, EntityReference, EntityStorage};
use fruity_game_engine::{
    any::FruityAny,
    export, export_impl, export_struct,
    script_value::ScriptObjectType,
    signal::Signal,
    sync::{Arc, RwLock},
};
use std::fmt::Debug;

/// Query builder for script queries
#[derive(FruityAny)]
#[export_struct(typescript = "class ScriptQueryBuilder<Args extends any[] = []> {
  withEntity(): ScriptQueryBuilder<[...Args, EntityReference]>;
  withId(): ScriptQueryBuilder<[...Args, EntityId]>;
  withName(): ScriptQueryBuilder<[...Args, string]>;
  withEnabled(): ScriptQueryBuilder<[...Args, boolean]>;
  with<T>(scriptObjectType: ScriptObjectType): ScriptQueryBuilder<[...Args, T]>;
  withOptional<T>(
    scriptObjectType: ScriptObjectType
  ): ScriptQueryBuilder<[...Args, T | null]>;
  without(
    scriptObjectType: ScriptObjectType
  ): ScriptQueryBuilder<[...Args, null]>;
  build(): ScriptQuery<[...Args]>
}")]
pub struct ScriptQueryBuilder {
    entity_storage: Arc<RwLock<EntityStorage>>,
    on_entity_location_moved: Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
    on_created: Signal<EntityReference>,
    on_deleted: Signal<EntityId>,
    params: Vec<Box<dyn ScriptQueryParam>>,
}

#[export_impl]
impl ScriptQueryBuilder {
    /// Create the entity query
    pub fn new(
        entity_storage: Arc<RwLock<EntityStorage>>,
        on_entity_location_moved: Signal<(EntityId, Arc<RwLock<EntityStorage>>, EntityLocation)>,
        on_created: Signal<EntityReference>,
        on_deleted: Signal<EntityId>,
    ) -> Self {
        ScriptQueryBuilder {
            entity_storage,
            on_entity_location_moved,
            on_created,
            on_deleted,
            params: Vec::default(),
        }
    }

    /// Inject the entity reference as next item of the query
    #[export]
    pub fn with_entity(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(ScriptWithEntityReference {
            entity_storage: self.entity_storage.clone(),
            on_entity_location_moved: self.on_entity_location_moved.clone(),
        }));
        query
    }

    /// Inject the entity id as next item of the query
    #[export]
    pub fn with_id(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(ScriptWithId {}));
        query
    }

    /// Inject the entity name as next item of the query
    #[export]
    pub fn with_name(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(ScriptWithName {}));
        query
    }

    /// Inject the entity enabled state as next item of the query
    #[export]
    pub fn with_enabled(&self) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(ScriptWithEnabled {}));
        query
    }

    /// Inject a component as next item of the query
    #[export]
    pub fn with(&self, script_object_type: ScriptObjectType) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(ScriptWith {
            entity_storage: self.entity_storage.clone(),
            on_entity_location_moved: self.on_entity_location_moved.clone(),
            script_object_type,
        }));
        query
    }

    /// Inject an optional component as next item of the query
    #[export]
    pub fn with_optional(&self, script_object_type: ScriptObjectType) -> Self {
        let mut query = self.clone();
        query.params.push(Box::new(ScriptWithOptional {
            entity_storage: self.entity_storage.clone(),
            on_entity_location_moved: self.on_entity_location_moved.clone(),
            script_object_type,
        }));
        query
    }

    /// Filter out entities that have a component
    #[export]
    pub fn without(&self, script_object_type: ScriptObjectType) -> Self {
        let mut query = self.clone();
        query
            .params
            .push(Box::new(ScriptWithout { script_object_type }));
        query
    }

    /// Build the query
    #[export]
    pub fn build(&self) -> ScriptQuery {
        ScriptQuery::new(
            Box::new(ScriptTuple {
                params: self.params.iter().map(|param| param.duplicate()).collect(),
            }),
            &self.entity_storage.read(),
            self.on_created.clone(),
            self.on_deleted.clone(),
        )
    }
}

impl Clone for ScriptQueryBuilder {
    fn clone(&self) -> Self {
        Self {
            entity_storage: self.entity_storage.clone(),
            on_entity_location_moved: self.on_entity_location_moved.clone(),
            on_created: self.on_created.clone(),
            on_deleted: self.on_deleted.clone(),
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
