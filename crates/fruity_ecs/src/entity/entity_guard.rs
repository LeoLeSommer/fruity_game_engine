use super::archetype::Entity;
use super::archetype::EntityMut;
use fruity_game_engine::RwLockReadGuard;
use fruity_game_engine::RwLockWriteGuard;
use std::fmt::Debug;
use std::ops::Deref;

/// RAII structure used to release the shared read access of a lock when dropped.
///
/// This structure is created by the [`read`] methods on [`EntityRwLock`].
///
/// [`read`]: EntityRwLock::read
///
pub struct EntityReadGuard<'a> {
    pub(crate) _entity_guard: RwLockReadGuard<'a, ()>,
    pub(crate) entity: Entity<'a>,
}

impl<'a> Debug for EntityReadGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Deref for EntityReadGuard<'a> {
    type Target = Entity<'a>;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}

/// RAII structure used to release the exclusive write access of a lock when dropped.
///
/// This structure is created by the [`write`] methods on [`EntityRwLock`].
///
/// [`write`]: EntityRwLock::write
///
pub struct EntityWriteGuard<'a> {
    pub(crate) _entity_guard: RwLockWriteGuard<'a, ()>,
    pub(crate) entity: EntityMut<'a>,
}

impl<'a> Debug for EntityWriteGuard<'a> {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        Ok(())
    }
}

impl<'a> Deref for EntityWriteGuard<'a> {
    type Target = EntityMut<'a>;

    fn deref(&self) -> &Self::Target {
        &self.entity
    }
}
