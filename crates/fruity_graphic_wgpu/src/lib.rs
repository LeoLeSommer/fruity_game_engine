#![feature(async_closure)]

use crate::graphic_service::WgpuGraphicService;
use fruity_game_engine::{
    console_log, export_function, module::Module, settings::Settings, typescript_import,
    world::World, FruityResult,
};
use fruity_graphic::graphic_service::GraphicService;
use std::{future::Future, pin::Pin, rc::Rc};

pub mod graphic_service;
pub mod resources;
pub mod utils;
pub mod wgpu_bridge;

#[typescript_import({Signal, ResourceReference, Module} from "fruity_game_engine")]
#[typescript_import({Matrix4, Color, TextureResource, Vector2D, ShaderResource, MeshResourceSettings, ShaderResourceSettings} from "fruity_graphic")]

async fn setup_async(world: World, _settings: Settings) -> FruityResult<()> {
    let resource_container = world.get_resource_container();

    console_log("1");
    let graphic_service = WgpuGraphicService::new(resource_container.clone()).await?;
    console_log("2");
    resource_container.add::<dyn GraphicService>("graphic_service", Box::new(graphic_service));
    console_log("3");

    FruityResult::Ok(())
}

fn make_setup_async() -> impl Fn(World, Settings) -> Pin<Box<dyn Future<Output = FruityResult<()>>>>
{
    move |world: World, settings: Settings| Box::pin(setup_async(world, settings))
}

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_graphic_wgpu_module() -> Module {
    Module {
        name: "fruity_graphic_platform".to_string(),
        dependencies: vec!["fruity_ecs".to_string(), "fruity_windows".to_string()],
        setup_async: Some(Rc::new(make_setup_async())),
        ..Default::default()
    }
}
