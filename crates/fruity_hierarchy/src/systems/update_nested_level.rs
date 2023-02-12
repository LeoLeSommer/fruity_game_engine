use crate::Parent;
use fruity_ecs::entity::entity_query::with::WithEntity;
use fruity_ecs::entity::entity_query::with::WithMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::system_service::StartupDisposeSystemCallback;
use fruity_game_engine::inject::Ref;
use fruity_game_engine::FruityResult;
use std::ops::Deref;

/// An internal system to update the nested level of a hierarchy component
/// It's mainly used to update the position in cascade cause the position of
/// a child must be updated after the parent
pub fn update_nested_level(
    entity_service: Ref<EntityService>,
    query: Query<(WithEntity, WithMut<Parent>)>,
) -> FruityResult<StartupDisposeSystemCallback> {
    let handle = query.on_created(move |(entity, mut parent)| {
        // Get the parent entity reference
        let parent_entity = if let Some(parent_id) = &parent.parent_id.deref() {
            let entity_service_reader = entity_service.read();
            entity_service_reader.get_entity(*parent_id)
        } else {
            None
        };

        // Set the nested level as the parent one plus one
        if let Some(parent_entity) = parent_entity {
            if let Some(parent_parent) = parent_entity.read().read_single_component::<Parent>() {
                parent.nested_level = parent_parent.nested_level + 1;
            } else {
                parent.nested_level = 1;
            }
        }

        // When parent is updated, we update the nested level
        let entity_service = entity_service.clone();
        let handle = parent.parent_id.on_updated.add_observer(move |parent_id| {
            let entity_writer = entity.write();
            let mut parent = entity_writer.write_single_component::<Parent>().unwrap();

            // Get the parent entity reference
            let parent_entity = if let Some(parent_id) = &parent_id {
                let entity_service_reader = entity_service.read();
                entity_service_reader.get_entity(*parent_id)
            } else {
                None
            };

            // Set the nested level as the parent one plus one
            if let Some(parent_entity) = parent_entity {
                if let Some(parent_parent) = parent_entity.read().read_single_component::<Parent>()
                {
                    parent.nested_level = parent_parent.nested_level + 1;
                } else {
                    parent.nested_level = 1;
                }
            }

            Ok(())
        });

        Some(Box::new(move || {
            handle.dispose_by_ref();
        }))
    });

    Ok(Some(Box::new(move || {
        handle.dispose();
        Ok(())
    })))
}
