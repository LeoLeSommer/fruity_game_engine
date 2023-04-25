use fruity_ecs::component::Component;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::ResourceReference;
use fruity_game_engine::{export_constructor, export_impl, export_struct};
use fruity_graphic::math::Color;
use fruity_graphic::resources::texture_resource::TextureResource;

#[derive(Debug, Clone, Component, FruityAny)]
#[export_struct]
pub struct Camera {
    pub near: f32,
    pub far: f32,
    pub target: Option<ResourceReference<dyn TextureResource>>,
    pub background_color: Color,
}

#[export_impl]
impl Camera {
    /// Returns a new Camera
    #[export_constructor]
    pub fn new() -> Camera {
        Self::default()
    }
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            near: -1.0,
            far: 1.0,
            target: None,
            background_color: Color::default(),
        }
    }
}
