use crate::Rotate2D;
use crate::Scale2D;
use crate::Transform2D;
use crate::Translate2D;
use fruity_ecs::query::Query;
use fruity_ecs::query::WithMut;
use fruity_ecs::query::WithOptional;
use fruity_game_engine::FruityResult;
use fruity_graphic::math::matrix3::Matrix3;

pub fn update_transform_2d(
    query: Query<(
        WithMut<Transform2D>,
        WithOptional<Translate2D>,
        WithOptional<Rotate2D>,
        WithOptional<Scale2D>,
    )>,
) -> FruityResult<()> {
    query.for_each(|(mut transform, translate_2d, rotate_2d, scale_2d)| {
        transform.transform = Matrix3::new_identity();

        if let Some(translate_2d) = translate_2d {
            transform.transform = transform.transform * Matrix3::new_translation(translate_2d.vec);
        }

        if let Some(rotate_2d) = rotate_2d {
            transform.transform = transform.transform * Matrix3::new_rotation(rotate_2d.angle);
        }

        if let Some(scale_2d) = scale_2d {
            transform.transform = transform.transform * Matrix3::new_scaling(scale_2d.vec);
        }

        Ok(())
    })
}
