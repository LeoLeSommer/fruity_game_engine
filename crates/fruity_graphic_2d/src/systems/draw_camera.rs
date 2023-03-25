use crate::Camera;
use crate::Transform2D;
use fruity_ecs::entity::entity_query::with::With;
use fruity_ecs::entity::entity_query::Query;
use fruity_game_engine::inject::Ref;
use fruity_game_engine::profile_scope;
use fruity_game_engine::FruityResult;
use fruity_graphic::graphic_service::GraphicService;
use fruity_graphic::math::matrix4::Matrix4;
use fruity_graphic::math::vector2d::Vector2D;

pub fn draw_camera(
    graphic_service: Ref<dyn GraphicService>,
    query: Query<(With<Transform2D>, With<Camera>)>,
) -> FruityResult<()> {
    query.for_each(|(transform, camera)| {
        let bottom_left = transform.transform * Vector2D::new(-0.5, -0.5);
        let top_right = transform.transform * Vector2D::new(0.5, 0.5);

        let view_proj = Matrix4::from_rect(
            bottom_left.x,
            top_right.x,
            bottom_left.y,
            top_right.y,
            camera.near,
            camera.far,
        );

        // Render the scene
        {
            profile_scope!("render_scene");
            let graphic_service = graphic_service.read();
            graphic_service.render_scene(view_proj, camera.background_color, camera.target.clone());
        }

        Ok(())
    })
}
