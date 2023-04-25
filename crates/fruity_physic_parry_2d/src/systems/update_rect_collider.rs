use crate::ParryRectCollider;
use fruity_ecs::query::Query;
use fruity_ecs::query::WithExtensionMut;
use fruity_game_engine::FruityResult;
use fruity_physic_2d::components::rect_collider::RectCollider;
use nalgebra::Vector2;

pub fn update_rect_collider(
    query: Query<WithExtensionMut<RectCollider, ParryRectCollider>>,
) -> FruityResult<()> {
    query.for_each(move |(collider, mut parry_collider)| {
        let half_extents = (collider.top_right - collider.bottom_left) / 2.0;
        parry_collider.shape.half_extents = Vector2::new(half_extents.x, half_extents.y);

        Ok(())
    })
}
