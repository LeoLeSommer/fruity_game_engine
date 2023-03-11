use crate::graphic_service::GraphicService;
use fruity_game_engine::{
    export, export_trait,
    resource::{resource_container::ResourceContainer, Resource},
    settings::Settings,
    utils::file::read_file_to_bytes_async,
    FruityResult,
};
use std::{future::Future, pin::Pin};

pub struct TextureResourceSettings {}

#[export_trait]
pub trait TextureResource: Resource {
    #[export]
    fn get_size(&self) -> (u32, u32);
}

pub fn load_texture(
    identifier: &str,
    settings: Settings,
    resource_container: ResourceContainer,
) -> Pin<Box<dyn Send + Future<Output = FruityResult<()>>>> {
    let identifier = identifier.to_string();
    Box::pin(async move {
        // Get the resource path
        let path = settings.get("path", String::default());

        // read the whole file
        let buffer = read_file_to_bytes_async(&path).await?;

        // Parse settings
        let settings = read_texture_settings(&settings);

        // Get the graphic service state
        let graphic_service = resource_container.require::<dyn GraphicService>();
        let graphic_service = graphic_service.read();

        // Build the resource
        let resource = graphic_service.create_texture_resource(&identifier, &buffer, settings)?;
        resource_container.add::<dyn TextureResource>(&identifier, resource);

        Ok(())
    })
}

pub fn read_texture_settings(_settings: &Settings) -> TextureResourceSettings {
    TextureResourceSettings {}
}
