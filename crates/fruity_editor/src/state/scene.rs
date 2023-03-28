use crate::state::inspector::InspectorState;
use crate::utils::file::settings_to_json_value;
use fruity_ecs::entity::entity_service::EntityService;
use fruity_ecs::entity::entity_service::EntityServiceSnapshot;
use fruity_ecs::system::SystemService;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::export_impl;
use fruity_game_engine::export_struct;
use fruity_game_engine::resource::resource_container::ResourceContainer;
use fruity_game_engine::resource::resource_reference::ResourceReference;
use fruity_game_engine::FruityResult;

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct SceneState {
    entity_service: ResourceReference<EntityService>,
    system_service: ResourceReference<SystemService>,
    inspector_state: ResourceReference<InspectorState>,
    snapshot: Option<EntityServiceSnapshot>,
}

#[export_impl]
impl SceneState {
    pub fn new(resource_container: ResourceContainer) -> Self {
        Self {
            entity_service: resource_container.require::<EntityService>(),
            system_service: resource_container.require::<SystemService>(),
            inspector_state: resource_container.require::<InspectorState>(),
            snapshot: None,
        }
    }

    #[export]
    pub fn run(&mut self) -> FruityResult<()> {
        let mut inspector_state = self.inspector_state.write();

        let entity_service = self.entity_service.read();
        self.snapshot = Some(entity_service.snapshot()?);

        // TODO: Remove
        let json = settings_to_json_value(self.snapshot.as_ref().unwrap().clone())?;
        println!("{}", json.to_string());

        inspector_state.unselect()?;
        entity_service.restore(true, self.snapshot.as_ref().unwrap().clone())?;
        std::mem::drop(entity_service);

        let system_service = self.system_service.read();
        system_service.set_paused(false)?;

        Ok(())
    }

    #[export]
    pub fn pause(&mut self) -> FruityResult<()> {
        let system_service = self.system_service.read();
        system_service.set_paused(true)?;

        Ok(())
    }

    #[export]
    pub fn is_running(&self) -> bool {
        let system_service = self.system_service.read();
        !system_service.is_paused()
    }

    #[export]
    pub fn stop(&mut self) -> FruityResult<()> {
        let mut inspector_state = self.inspector_state.write();
        let entity_service = self.entity_service.read();

        let system_service = self.system_service.read();

        if let Some(snapshot) = self.snapshot.as_ref() {
            entity_service.restore(true, snapshot.clone())?;
            inspector_state.unselect()?;
            system_service.set_paused(true)?;
            self.snapshot = None;
        }

        Ok(())
    }

    #[export]
    pub fn can_stop(&self) -> bool {
        self.snapshot.is_some()
    }
}
