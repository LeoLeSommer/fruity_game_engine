use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_game_engine::FruityResult;
use fruity_graphic_2d::components::transform_2d::Transform2D;
use fruity_hierarchy::components::parent::Parent;
use std::ops::Deref;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;

pub fn transform_2d_cascade(
    query: Query<(With<Parent>, WithMut<Transform2D>)>,
) -> FruityResult<()> {
    let mut current_nested_level = 1;
    while transform_2d_cascade_for_nested_level(query.clone(), current_nested_level)? {
        current_nested_level += 1;
    }

    Ok(())
}

pub fn transform_2d_cascade_for_nested_level(
    query: Query<(With<Parent>, WithMut<Transform2D>)>,
    nested_level: usize,
) -> FruityResult<bool> {
    let did_transform = Arc::new(AtomicBool::new(false));
    let did_transform_2 = did_transform.clone();

    query.for_each(move |(child, mut transform)| {
        if child.nested_level == nested_level {
            // Apply the parent transform to the child
            if let Some(parent_entity) = &child.parent.deref() {
                if let Some(parent_transform) = parent_entity
                    .read()?
                    .get_component_by_type::<Transform2D>()?
                {
                    transform.transform = parent_transform.transform * transform.transform;
                    did_transform.store(true, Relaxed);
                }
            }
        }

        Ok(())
    })?;

    Ok(did_transform_2.load(Relaxed))
}
