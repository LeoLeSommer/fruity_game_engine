use crate::graphic_service::WgpuGraphicService;
use fruity_game_engine::{
    settings::Settings,
    world::{SetupWorldMiddlewareNext, World},
    FruityResult,
};
use fruity_graphic::graphic_service::GraphicService;

pub fn setup_world_middleware(
    world: World,
    settings: Settings,
    next: SetupWorldMiddlewareNext,
) -> FruityResult<()> {
    // Run next
    next(world.clone(), settings.clone())?;

    let resource_container = world.get_resource_container();

    let graphic_service = WgpuGraphicService::new(resource_container.clone())?;
    resource_container.add::<dyn GraphicService>("graphic_service", Box::new(graphic_service));

    Ok(())
}
