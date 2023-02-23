use crate::graphic_service::GraphicService;
use fruity_game_engine::{
    export, export_trait,
    resource::{resource_container::ResourceContainer, Resource},
    settings::Settings,
    FruityError, FruityResult,
};
use std::{fs::File, io::Read};

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
) -> FruityResult<()> {
    // Get the graphic service state
    let graphic_service = resource_container.require::<dyn GraphicService>();
    let graphic_service = graphic_service.read();

    // Get the resource path
    let path = settings.get("path", String::default());

    // read the whole file
    let mut reader = File::open(&path)
        .map_err(|_| FruityError::GenericFailure(format!("Could not read file {}", &path)))?;
    let mut buffer = Vec::new();
    if let Err(err) = reader.read_to_end(&mut buffer) {
        return Err(FruityError::GenericFailure(err.to_string()));
    }

    // Parse settings
    let settings = read_texture_settings(&settings);

    // Build the resource
    let resource = graphic_service.create_texture_resource(identifier, &buffer, settings)?;
    resource_container.add::<dyn TextureResource>(identifier, resource);

    Ok(())
}

pub fn read_texture_settings(_settings: &Settings) -> TextureResourceSettings {
    TextureResourceSettings {}
}
