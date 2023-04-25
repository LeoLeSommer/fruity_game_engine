use crate::components::rigid_body::RigidBody;
use fruity_ecs::query::Query;
use fruity_ecs::query::With;
use fruity_ecs::query::WithOptionalMut;
use fruity_game_engine::FruityResult;
use fruity_graphic_2d::components::rotate_2d::Rotate2D;
use fruity_graphic_2d::components::translate_2d::Translate2D;

pub fn update_rigid_body(
    query: Query<(
        With<RigidBody>,
        WithOptionalMut<Translate2D>,
        WithOptionalMut<Rotate2D>,
    )>,
) -> FruityResult<()> {
    query.for_each(move |(_kinematic_rigid_body, _translate_2d, rotate_2d)| Ok(()))
}
