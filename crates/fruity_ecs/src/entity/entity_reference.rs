use crate::component::component_reference::ComponentReference;
use crate::entity::archetype::ArchetypeArcRwLock;
use crate::entity::entity_guard::EntityReadGuard;
use crate::entity::entity_guard::EntityWriteGuard;
use fruity_game_engine::any::FruityAny;
use fruity_game_engine::export;
use fruity_game_engine::fruity_export;
use fruity_game_engine::RwLockReadGuard;
use fruity_game_engine::RwLockWriteGuard;
use std::fmt::Debug;
use std::rc::Rc;

use super::entity::EntityId;

fruity_export! {
/// A reference over an entity stored into an Archetype
    #[derive(Clone, FruityAny)]
    pub struct EntityReference {
        pub(crate) entity_id: usize,
        pub(crate) archetype: ArchetypeArcRwLock,
    }

    impl EntityReference {
        /// Get a read access to the entity
        pub fn read(&self) -> EntityReadGuard {
            let archetype_reader = self.archetype.read();
            let guard = archetype_reader
                .lock_array
                .get(self.entity_id)
                .unwrap()
                .read();

            // TODO: Find a way to remove it
            let guard =
                unsafe { std::mem::transmute::<RwLockReadGuard<()>, RwLockReadGuard<()>>(guard) };

            EntityReadGuard {
                entity_id: self.entity_id,
                _guard: Rc::new(guard),
                archetype_reader: Rc::new(archetype_reader),
            }
        }

        /// Get a write access to the entity
        pub fn write(&self) -> EntityWriteGuard {
            let archetype_reader = self.archetype.read();
            let guard = archetype_reader
                .lock_array
                .get(self.entity_id)
                .unwrap()
                .write();

            // TODO: Find a way to remove it
            let guard =
                unsafe { std::mem::transmute::<RwLockWriteGuard<()>, RwLockWriteGuard<()>>(guard) };

            EntityWriteGuard {
                entity_id: self.entity_id,
                _guard: Rc::new(guard),
                archetype_reader: Rc::new(archetype_reader),
            }
        }

        /// Get all components
        pub fn get_components(&self) -> Vec<ComponentReference> {
            self.archetype.clone().get_entity_components(self.entity_id)
        }

        /// Get components with a given type
        ///
        /// # Arguments
        /// * `component_identifier` - The component identifier
        ///
        pub fn get_components_by_type_identifier(
            &self,
            component_identifier: &str,
        ) -> Vec<ComponentReference> {
            self.archetype
                .clone()
                .get_entity_components_from_type(self.entity_id, component_identifier)
        }

        /// Get entity id
        #[export]
        pub fn get_entity_id(&self) -> EntityId {
            self.archetype.clone().get_entity_components(self.entity_id)
        }

        /// Get entity name
        #[export]
        pub fn get_name(&self) -> String {
            self.archetype.clone().get_entity_components(self.entity_id)
        }

        /// Get entity enabled
        #[export]
        pub fn is_enabled(&self) -> bool {
            self.archetype.clone().get_entity_components(self.entity_id)
        }
    }
}

impl Debug for EntityReference {
    fn fmt(&self, _: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}
