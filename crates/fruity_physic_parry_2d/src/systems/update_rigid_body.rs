use crate::components::rigid_body::RigidBody;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::with::WithOptionalMut;
use fruity_ecs::entity::entity_query::Query;
use fruity_game_engine::FruityResult;
use fruity_graphic_2d::components::rotate_2d::Rotate2d;
use fruity_graphic_2d::components::translate_2d::Translate2d;

pub fn update_rigid_body(
    query: Query<(
        With<RigidBody>,
        WithOptionalMut<Translate2d>,
        WithOptionalMut<Rotate2d>,
    )>,
) -> FruityResult<()> {
    query.for_each(move |(_kinematic_rigid_body, _translate_2d, rotate_2d)| Ok(()))
}
