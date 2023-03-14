#![feature(async_closure)]

use crate::graphic_service::WgpuGraphicService;
use fruity_game_engine::{
    export_function, module::Module, settings::Settings, typescript_import, world::World,
    FruityResult,
};
use fruity_graphic::graphic_service::GraphicService;
use std::sync::Arc;

pub mod graphic_service;
pub mod resources;
pub mod utils;
pub mod wgpu_bridge;
pub mod world_fn;

#[typescript_import({Signal, ResourceReference, Module} from "fruity_game_engine")]
#[typescript_import({Matrix4, Color, TextureResource, Vector2D, ShaderResource, MeshResourceSettings, ShaderResourceSettings} from "fruity_graphic")]

/// Returns the module, ready to be registered into the fruity_game_engine
#[export_function]
pub fn create_fruity_graphic_wgpu_module() -> Module {
    Module {
        name: "fruity_graphic_platform".to_string(),
        dependencies: vec!["fruity_ecs".to_string(), "fruity_windows".to_string()],
        setup_async: Some(Arc::new(move |world: World, _settings: Settings| {
            Box::pin(async move {
                let resource_container = world.get_resource_container();

                let graphic_service = resource_container.require::<dyn GraphicService>();
                world.add_run_frame_middleware(move |next, world| {
                    {
                        let mut graphic_service = graphic_service.write();
                        graphic_service.start_draw()?;
                    }

                    next(world.clone())?;

                    {
                        let mut graphic_service = graphic_service.write();
                        graphic_service.end_draw();
                    }

                    FruityResult::Ok(())
                });

                let graphic_service = resource_container.require::<dyn GraphicService>();
                let (instance, surface) = {
                    let graphic_service_reader = graphic_service.read();
                    let graphic_service =
                        graphic_service_reader.downcast_ref::<WgpuGraphicService>();

                    (
                        graphic_service.get_instance_arc(),
                        graphic_service.get_surface_arc(),
                    )
                };

                let state = WgpuGraphicService::initialize_async(
                    resource_container.clone(),
                    &instance,
                    &surface,
                )
                .await?;

                {
                    let mut graphic_service_writer = graphic_service.write();
                    let graphic_service =
                        graphic_service_writer.downcast_mut::<WgpuGraphicService>();
                    graphic_service.set_state(state);
                };

                FruityResult::Ok(())
            })
        })),
        setup_world_middleware: Some(Arc::new(world_fn::setup_world_middleware)),
        ..Default::default()
    }
}
