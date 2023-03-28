use fruity_ecs::system::{StartupDisposeSystemCallback, SystemService};
use fruity_game_engine::{inject::Const, FruityResult};

pub fn pause_at_startup(
    system_service: Const<SystemService>,
) -> FruityResult<StartupDisposeSystemCallback> {
    system_service.set_paused(true)?;

    Ok(None)
}
