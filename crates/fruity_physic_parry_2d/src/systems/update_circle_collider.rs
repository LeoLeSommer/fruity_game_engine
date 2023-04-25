use crate::components::parry_circle_collider::ParryCircleCollider;
use fruity_ecs::query::Query;
use fruity_ecs::query::WithExtensionMut;
use fruity_game_engine::FruityResult;
use fruity_physic_2d::components::circle_collider::CircleCollider;

pub fn update_circle_collider(
    query: Query<WithExtensionMut<CircleCollider, ParryCircleCollider>>,
) -> FruityResult<()> {
    query.for_each(move |(collider, mut parry_collider)| {
        parry_collider.shape.radius = collider.radius;

        Ok(())
    })
}
