use crate::components::parent::Parent;
use fruity_ecs::{
    entity::EntityService,
    query::{Query, With, WithId},
    system::StartupDisposeSystemCallback,
};
use fruity_game_engine::{inject::Ref, FruityResult};
use std::ops::Deref;

/// Delete when child when it's parent is deleted
pub fn delete_cascade(
    entity_service: Ref<EntityService>,
    query: Query<(WithId, With<Parent>)>,
) -> FruityResult<StartupDisposeSystemCallback> {
    let entity_service_reader = entity_service.read();
    let handle = entity_service_reader
        .on_deleted
        .add_observer(move |deleted_id| {
            let deleted_id = *deleted_id;
            let entity_service = entity_service.clone();

            query.for_each(move |(entity_id, parent)| {
                let is_child_of_deleted = {
                    if let Some(entity_parent_id) = parent.parent.deref() {
                        entity_parent_id.read()?.get_entity_id() == deleted_id
                    } else {
                        false
                    }
                };

                if is_child_of_deleted {
                    let entity_service = entity_service.read();
                    entity_service.remove_entity(entity_id)?;
                }

                Ok(())
            })
        });

    Ok(Some(Box::new(move || {
        handle.dispose();
        Ok(())
    })))
}
