use crate::ui::context::UIContext;
use crate::utils::file::get_file_type_from_path;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::{export_impl, export_struct};
use fruity_graphic::resources::texture_resource::TextureResource;
use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::Arc;

struct FileTypeEntry {
    get_thumbnail: Arc<
        dyn Fn(&UIContext, &str) -> Option<ResourceReference<dyn TextureResource>> + Send + Sync,
    >,
    on_selected: Arc<dyn Fn(&UIContext, &str) + Send + Sync>,
}

#[derive(FruityAny)]
#[export_struct]
pub struct FileExplorerService {
    resource_container: ResourceContainer,
    file_types: HashMap<String, FileTypeEntry>,
}

#[export_impl]
impl FileExplorerService {
    pub fn new(resource_container: ResourceContainer) -> Self {
        FileExplorerService {
            resource_container,
            file_types: HashMap::new(),
        }
    }

    pub fn register_file_type(
        &mut self,
        file_type: &str,
        get_thumbnail: impl Fn(&UIContext, &str) -> Option<ResourceReference<dyn TextureResource>>
            + Send
            + Sync
            + 'static,
        on_selected: impl Fn(&UIContext, &str) + Send + Sync + 'static,
    ) {
        self.file_types.insert(
            file_type.to_string(),
            FileTypeEntry {
                get_thumbnail: Arc::new(get_thumbnail),
                on_selected: Arc::new(on_selected),
            },
        );
    }

    pub fn get_thumbnail(
        &self,
        ctx: &UIContext,
        file_path: &str,
    ) -> ResourceReference<dyn TextureResource> {
        match self.inner_get_thumbnail(ctx, file_path) {
            Some(thumbnail) => thumbnail,
            None => self
                .resource_container
                .get::<dyn TextureResource>("Editor/Icons/unknown")
                .unwrap(),
        }
    }

    pub fn notify_selected(&self, ctx: &UIContext, file_path: &str) {
        self.inner_notify_selected(ctx, file_path);
    }

    // TODO: There should be a way to use the ? without having to do that
    fn inner_notify_selected(&self, ctx: &UIContext, file_path: &str) -> Option<()> {
        let file_type = get_file_type_from_path(file_path)?;
        let file_type = self.file_types.get(&file_type)?;
        (file_type.on_selected)(ctx, file_path);

        Some(())
    }

    fn inner_get_thumbnail(
        &self,
        ctx: &UIContext,
        file_path: &str,
    ) -> Option<ResourceReference<dyn TextureResource>> {
        let file_type = get_file_type_from_path(file_path)?;
        let file_type = self.file_types.get(&file_type)?;
        (file_type.get_thumbnail)(ctx, file_path)
    }
}

impl Debug for FileExplorerService {
    fn fmt(
        &self,
        _formatter: &mut std::fmt::Formatter<'_>,
    ) -> std::result::Result<(), std::fmt::Error> {
        Ok(())
    }
}
