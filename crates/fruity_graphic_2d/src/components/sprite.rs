use fruity_ecs::component::Component;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::ResourceReference;
use fruity_game_engine::{export_constructor, export_impl, export_struct};
use fruity_graphic::resources::material_resource::MaterialResource;
use fruity_graphic::resources::texture_resource::TextureResource;

#[derive(Debug, Clone, Default, Component, FruityAny)]
#[export_struct]
pub struct Sprite {
    pub material: Option<ResourceReference<dyn MaterialResource>>,
    pub texture: Option<ResourceReference<dyn TextureResource>>,
    pub z_index: i32,
}

#[export_impl]
impl Sprite {
    /// Returns a new Camera
    #[export_constructor]
    pub fn new(
        material: Option<ResourceReference<dyn MaterialResource>>,
        texture: Option<ResourceReference<dyn TextureResource>>,
        z_index: i32,
    ) -> Sprite {
        Self {
            material,
            texture,
            z_index,
        }
    }
}
