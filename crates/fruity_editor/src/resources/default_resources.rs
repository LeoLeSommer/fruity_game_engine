use fruity_game_engine::{
    resource::resource_container::ResourceContainer, settings::Settings,
    utils::encode_decode::encode_base_64, FruityError, FruityResult,
};
use maplit::hashmap;

pub async fn load_default_resources_async(
    resource_container: ResourceContainer,
) -> FruityResult<()> {
    load_default_icons_async(resource_container.clone()).await
}

pub async fn load_default_icons_async(resource_container: ResourceContainer) -> FruityResult<()> {
    load_icon_async(
        &resource_container,
        "Editor/Icons/unknown",
        include_bytes!("unknown_thumbnail.png"),
    )
    .await?;

    load_icon_async(
        &resource_container,
        "Editor/Icons/folder",
        include_bytes!("folder_thumbnail.png"),
    )
    .await?;

    load_icon_async(
        &resource_container,
        "Editor/Icons/settings",
        include_bytes!("settings_thumbnail.png"),
    )
    .await?;

    Ok(())
}

pub async fn load_icon_async(
    resource_container: &ResourceContainer,
    name: &str,
    bytes: &[u8],
) -> FruityResult<()> {
    let settings = Settings::Object(hashmap! {
        "type".to_string() => Settings::String("texture".to_string()),
        "bytes".to_string() => Settings::String(encode_base_64(bytes.to_vec()).map_err(|err| FruityError::GenericFailure(err.to_string()))?)
    });

    resource_container
        .load_resource_async(name.to_string(), "texture".to_string(), settings)
        .await
}
