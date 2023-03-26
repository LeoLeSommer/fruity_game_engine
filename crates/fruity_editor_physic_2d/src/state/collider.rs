use fruity_ecs::{
    component::component_reference::AnyComponentReference,
    entity::entity_reference::EntityReference,
};
use fruity_editor::state::inspector::InspectorState;
use fruity_game_engine::{
    any::FruityAny,
    export_impl, export_struct,
    resource::{resource_container::ResourceContainer, resource_reference::ResourceReference},
};
use std::ops::Deref;

#[derive(Debug, FruityAny)]
#[export_struct]
pub struct ColliderState {
    inspector_state: ResourceReference<InspectorState>,
    current_editing_collider: Option<AnyComponentReference>,
}

#[export_impl]
impl ColliderState {
    pub fn new(resource_container: ResourceContainer) -> Self {
        let inspector_state = resource_container.require::<InspectorState>();
        let inspector_state_reader = inspector_state.read();

        let resource_container_2 = resource_container.clone();
        inspector_state_reader.on_selected.add_observer(move |_| {
            let collider_state = resource_container.require::<ColliderState>();
            let mut collider_state = collider_state.write();
            collider_state.current_editing_collider = None;

            Ok(())
        });

        inspector_state_reader.on_unselected.add_observer(move |_| {
            let collider_state = resource_container_2.require::<ColliderState>();
            let mut collider_state = collider_state.write();
            collider_state.current_editing_collider = None;

            Ok(())
        });

        Self {
            inspector_state,
            current_editing_collider: None,
        }
    }

    pub fn edit_collider(&mut self, component: AnyComponentReference) {
        self.current_editing_collider = Some(component);

        let mut inspector_state_writer = self.inspector_state.write();
        inspector_state_writer.temporary_display_gizmos();
    }

    pub fn is_editing_collider(&self) -> bool {
        match self.current_editing_collider {
            Some(_) => true,
            None => false,
        }
    }

    pub fn get_editing_collider(&self) -> Option<AnyComponentReference> {
        self.current_editing_collider.clone()
    }

    pub fn get_editing_entity(&self) -> Option<EntityReference> {
        Some(
            self.inspector_state
                .read()
                .get_selected()?
                .deref()
                .as_any_ref()
                .downcast_ref::<EntityReference>()?
                .clone(),
        )
    }
}
