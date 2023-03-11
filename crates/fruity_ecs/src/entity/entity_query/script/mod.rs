use crate::entity::archetype::Archetype;
use crate::entity::archetype::ArchetypeArcRwLock;
use crate::entity::entity_query::script::params::With;
use crate::entity::entity_query::script::params::WithEnabled;
use crate::entity::entity_query::script::params::WithEntity;
use crate::entity::entity_query::script::params::WithId;
use crate::entity::entity_query::script::params::WithName;
use crate::entity::entity_query::script::params::WithOptional;
use crate::entity::entity_query::EntityId;
use crate::entity::entity_reference::EntityReference;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::script_value::ScriptValue;
use fruity_game_engine::signal::ObserverHandler;
use fruity_game_engine::signal::Signal;
use fruity_game_engine::FruityError;
use fruity_game_engine::FruityResult;
use fruity_game_engine::RwLock;
use fruity_game_engine::{export, export_impl, export_struct};
use itertools::Itertools;
use std::fmt::Debug;
use std::sync::Arc;

pub(crate) mod params;

pub trait ScriptQueryParam: FruityAny + Send + Sync {
    fn duplicate(&self) -> Box<dyn ScriptQueryParam>;
    fn filter_archetype(&self, archetype: &Archetype) -> bool;
    fn get_entity_components(
        &self,
        entity_reference: EntityReference,
    ) -> FruityResult<Vec<ScriptValue>>;
}

#[derive(FruityAny)]
#[export_struct(typescript = "interface ScriptQuery<Args extends any[] = []> {
  withEntity(): ScriptQuery<[...Args, EntityReference]>;
  withId(): ScriptQuery<[...Args, EntityId]>;
  withName(): ScriptQuery<[...Args, string]>;
  withEnabled(): ScriptQuery<[...Args, boolean]>;
  with<T>(componentIdentifier: string): ScriptQuery<[...Args, T]>;
  withOptional<T>(
    componentIdentifier: string
  ): ScriptQuery<[...Args, T | null]>;
  forEach(callback: (args: Args) => void);
  onCreated(callback: (args: Args) => undefined | (() => void)): ObserverHandler;
}")]
pub struct ScriptQuery {
    pub(crate) archetypes: Arc<RwLock<Vec<ArchetypeArcRwLock>>>,
    pub(crate) on_entity_created: Signal<EntityReference>,
    pub(crate) on_entity_deleted: Signal<EntityId>,
    pub(crate) params: Vec<Box<dyn ScriptQueryParam>>,
}

#[export_impl]
impl ScriptQuery {
    #[export]
    pub fn with_entity(&self) -> ScriptQuery {
        let mut query = self.clone();
        query.params.push(Box::new(WithEntity {}));
        query
    }

    #[export]
    pub fn with_id(&self) -> ScriptQuery {
        let mut query = self.clone();
        query.params.push(Box::new(WithId {}));
        query
    }

    #[export]
    pub fn with_name(&self) -> ScriptQuery {
        let mut query = self.clone();
        query.params.push(Box::new(WithName {}));
        query
    }

    #[export]
    pub fn with_enabled(&self) -> ScriptQuery {
        let mut query = self.clone();
        query.params.push(Box::new(WithEnabled {}));
        query
    }

    #[export]
    pub fn with(&self, component_identifier: String) -> ScriptQuery {
        let mut query = self.clone();
        query.params.push(Box::new(With {
            identifier: component_identifier,
        }));
        query
    }

    #[export]
    pub fn with_optional(&self, component_identifier: String) -> ScriptQuery {
        let mut query = self.clone();
        query.params.push(Box::new(WithOptional {
            identifier: component_identifier,
        }));
        query
    }

    #[export]
    pub fn for_each(
        &self,
        callback: Arc<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
    ) -> FruityResult<()> {
        let archetypes = self.archetypes.read();
        let archetype_filter = self.archetype_filter();

        let entities = archetypes
            .iter()
            .filter(|archetype| archetype_filter(archetype))
            .map(|archetype| archetype.iter(false))
            .flatten()
            .collect::<Vec<_>>();

        entities.into_iter().try_for_each(|entity| {
            let script_params: Vec<Vec<ScriptValue>> = self
                .params
                .iter()
                .map(|param| param.get_entity_components(entity.clone()))
                .try_collect()?;

            let mut script_params = script_params.into_iter().multi_cartesian_product();

            script_params.try_for_each(|params| {
                callback(params)?;

                Result::<(), FruityError>::Ok(())
            })?;

            Result::<(), FruityError>::Ok(())
        })
    }

    /// Call a function for every entities of an query
    #[export]
    pub fn on_created(
        &self,
        callback: Arc<dyn Send + Sync + Fn(Vec<ScriptValue>) -> FruityResult<ScriptValue>>,
    ) -> FruityResult<ObserverHandler<EntityReference>> {
        // let on_entity_deleted = self.on_entity_deleted.clone();
        let archetype_filter = self.archetype_filter();
        let params = self
            .params
            .iter()
            .map(|param| param.duplicate())
            .collect::<Vec<_>>();

        Ok(self.on_entity_created.add_observer(move |entity| {
            if archetype_filter(&entity.archetype) {
                /*let entity_id = {
                    let entity_reader = entity.read();
                    entity_reader.get_entity_id()
                };*/

                let mut serialized_params = params
                    .iter()
                    .map(|param| param.get_entity_components(entity.clone()))
                    .multi_cartesian_product()
                    .flatten();

                serialized_params.try_for_each(|params| {
                    // TODO: Try to find a way to get back the result from thread safe function
                    callback(params)?;
                    /*let dispose_callback = callback(params);

                    if let Some(dispose_callback) = dispose_callback {
                        let dispose_callback = dispose_callback.create_thread_safe_callback()?;
                        on_entity_deleted.add_self_dispose_observer(
                            move |signal_entity_id, handler| {
                                if entity_id == *signal_entity_id {
                                    dispose_callback(vec![]);
                                    handler.dispose_by_ref();
                                }
                            },
                        )
                    }*/

                    Result::<(), FruityError>::Ok(())
                })
            } else {
                Ok(())
            }
        }))
    }

    fn archetype_filter(&self) -> Box<dyn Fn(&ArchetypeArcRwLock) -> bool + Send + Sync + 'static> {
        let params = self
            .clone()
            .params
            .into_iter()
            .map(|param| param)
            .collect::<Vec<_>>();

        Box::new(move |archetype| {
            for param in params.iter() {
                if !param.filter_archetype(&archetype.read()) {
                    return false;
                }
            }

            true
        })
    }
}

impl Clone for ScriptQuery {
    fn clone(&self) -> Self {
        Self {
            archetypes: self.archetypes.clone(),
            on_entity_created: self.on_entity_created.clone(),
            on_entity_deleted: self.on_entity_deleted.clone(),
            params: self.params.iter().map(|param| param.duplicate()).collect(),
        }
    }
}

impl Debug for ScriptQuery {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
