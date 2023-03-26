use fruity_editor::resources::default_resources::{load_default_icons_async, load_icon_async};
use fruity_game_engine::{resource::resource_container::ResourceContainer, FruityResult};

pub async fn load_default_resources_async(
    resource_container: ResourceContainer,
) -> FruityResult<()> {
    load_default_icons_async(resource_container.clone()).await
}

pub async fn load_default_icons(resource_container: ResourceContainer) -> FruityResult<()> {
    load_icon_async(
        &resource_container,
        "Editor/Icons/material",
        include_bytes!("material_thumbnail.png"),
    )
    .await?;

    load_icon_async(
        &resource_container,
        "Editor/Icons/shader",
        include_bytes!("shader_thumbnail.png"),
    )
    .await?;

    Ok(())
}
