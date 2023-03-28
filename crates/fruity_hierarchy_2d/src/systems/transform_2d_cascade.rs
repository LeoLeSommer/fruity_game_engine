use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_game_engine::FruityResult;
use fruity_graphic_2d::components::transform_2d::Transform2D;
use fruity_hierarchy::components::parent::Parent;
use std::ops::Deref;

pub fn transform_2d_cascade(
    query: Query<(With<Parent>, WithMut<Transform2D>)>,
) -> FruityResult<()> {
    query.for_each(move |(child, mut transform)| {
        // Apply the parent transform to the child
        if let Some(parent_entity) = &child.parent.deref() {
            if let Some(parent_transform) = parent_entity
                .read()?
                .get_component_by_type::<Transform2D>()?
            {
                transform.transform = parent_transform.transform * transform.transform;
            }
        }

        Ok(())
    })
}
