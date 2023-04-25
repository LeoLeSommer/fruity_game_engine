use crate::Graphic2dService;
use crate::Sprite;
use crate::Transform2D;
use fruity_ecs::query::Query;
use fruity_ecs::query::With;
use fruity_ecs::query::WithId;
use fruity_game_engine::inject::Ref;
use fruity_game_engine::FruityResult;
use fruity_graphic::graphic_service::MaterialParam;
use maplit::hashmap;

pub fn draw_sprite(
    graphic_2d_service: Ref<Graphic2dService>,
    query: Query<(WithId, With<Transform2D>, With<Sprite>)>,
) -> FruityResult<()> {
    query.for_each(|(entity_id, transform, sprite)| {
        let graphic_2d_service = graphic_2d_service.read();

        if let Some(material) = &sprite.material {
            graphic_2d_service.draw_quad(
                entity_id.0,
                material.clone(),
                hashmap! {
                    "transform".to_string() => MaterialParam::Matrix4(transform.transform.into()),
                },
                sprite.z_index,
            );
        }

        Ok(())
    })
}
