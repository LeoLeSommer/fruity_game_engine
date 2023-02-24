#![feature(async_closure)]

use crate::graphic_service::WgpuGraphicService;
use fruity_game_engine::{
    export_function, module::Module, settings::Settings, typescript_import, world::World,
    FruityResult,
};
use fruity_graphic::graphic_service::GraphicService;
use std::rc::Rc;

pub mod graphic_service;
pub mod resources;
pub mod utils;
pub mod wgpu_bridge;

#[typescript_import({Signal, ResourceReference, Module} from "fruity_game_engine")]
#[typescript_import({Matrix4, Color, TextureResource, Vector2D, ShaderResource, MeshResourceSettings, ShaderResourceSettings} from "fruity_graphic")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_graphic_wgpu_module() -> Module {
    let test = Box::pin(async move |world: World, _settings: Settings| {
        let resource_container = world.get_resource_container();

        let graphic_service = WgpuGraphicService::new(resource_container.clone()).await?;
        resource_container.add::<dyn GraphicService>("graphic_service", Box::new(graphic_service));

        FruityResult::Ok(())
    });

    Module {
        name: "fruity_graphic_platform".to_string(),
        dependencies: vec!["fruity_ecs".to_string(), "fruity_windows".to_string()],
        setup_async: Some(Rc::new(Box::pin(
            async move |world: World, _settings: Settings| {
                let resource_container = world.get_resource_container();

                let graphic_service = WgpuGraphicService::new(resource_container.clone()).await?;
                resource_container
                    .add::<dyn GraphicService>("graphic_service", Box::new(graphic_service));

                FruityResult::Ok(())
            },
        ))),
        ..Default::default()
    }
}
