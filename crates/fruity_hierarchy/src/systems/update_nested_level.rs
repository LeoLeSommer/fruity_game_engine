use crate::Parent;
use fruity_ecs::entity::entity_query::with::WithEntityReference;
use fruity_ecs::entity::entity_query::with::WithMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_ecs::system::StartupDisposeSystemCallback;
use fruity_game_engine::FruityResult;
use std::ops::Deref;

/// An internal system to update the nested level of a hierarchy component
/// It's mainly used to update the position in cascade cause the position of
/// a child must be updated after the parent
pub fn update_nested_level(
    query: Query<(WithEntityReference, WithMut<Parent>)>,
) -> FruityResult<StartupDisposeSystemCallback> {
    let handle = query.on_created(
        move |(child_entity_reference, mut child_entity_parent_component)| {
            // Set child nested level as the parent plus one
            if let Some(parent_entity_parent_component) =
                child_entity_parent_component.parent.deref()
            {
                if let Some(parent_entity_parent_component) = parent_entity_parent_component
                    .read()?
                    .get_component_by_type::<Parent>()?
                {
                    child_entity_parent_component.nested_level =
                        parent_entity_parent_component.nested_level + 1;
                } else {
                    child_entity_parent_component.nested_level = 1;
                }
            }

            // When parent is updated, we update child nested level
            let handle = child_entity_parent_component
                .parent
                .on_updated
                .add_observer(move |parent_entity| {
                    let child_entity_writer = child_entity_reference.write()?;
                    let child_entity_parent_component = child_entity_writer
                        .get_component_by_type_mut::<Parent>()
                        .unwrap();

                    // Set child nested level as the parent plus one
                    if let Some(mut child_entity_parent_component) = child_entity_parent_component {
                        if let Some(parent_entity) = parent_entity {
                            if let Some(parent_entity_parent_component) =
                                parent_entity.read()?.get_component_by_type::<Parent>()?
                            {
                                child_entity_parent_component.nested_level =
                                    parent_entity_parent_component.nested_level + 1;
                            } else {
                                child_entity_parent_component.nested_level = 1;
                            }
                        } else {
                            child_entity_parent_component.nested_level = 1;
                        }
                    }

                    Ok(())
                });

            // TODO: When parent nested level is updated, we update child nested level

            Ok(Some(Box::new(move || {
                handle.dispose_by_ref();
            })))
        },
    );

    Ok(Some(Box::new(move || {
        handle.dispose();
        Ok(())
    })))
}
