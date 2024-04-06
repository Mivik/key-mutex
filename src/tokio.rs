use crate::{
    inner::{KeyRef, LockMap},
    Empty,
};
use std::{
    hash::Hash,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
};
use tokio::sync::{
    self, OwnedMutexGuard as TokioOwnedMutexGuard,
    OwnedRwLockReadGuard as TokioOwnedRwLockReadGuard,
    OwnedRwLockWriteGuard as TokioOwnedRwLockWriteGuard,
};

decl_mutex_guard!(TokioOwnedMutexGuard);
#[derive(Clone, Default)]
pub struct KeyMutex<K: Eq + Hash, V>(LockMap<K, sync::Mutex<V>>);
impl<K: Eq + Hash + Clone, V: Empty + Default + 'static> KeyMutex<K, V> {
    pub fn new() -> Self {
        Self(LockMap::new())
    }

    pub async fn lock(&self, key: K) -> OwnedMutexGuard<K, V> {
        let guard = self.0.obtain(key.clone()).lock_owned().await;
        OwnedMutexGuard {
            key_ref: KeyRef::new(&self.0, key),
            guard: ManuallyDrop::new(guard),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

decl_rwlock_guard!(TokioOwnedRwLockReadGuard, TokioOwnedRwLockWriteGuard);
#[derive(Clone, Default)]
pub struct KeyRwLock<K: Eq + Hash, V>(LockMap<K, sync::RwLock<V>>);
impl<K: Eq + Hash + Clone, V: Empty + Default + 'static> KeyRwLock<K, V> {
    pub fn new() -> Self {
        Self(LockMap::new())
    }

    pub async fn read(&self, key: K) -> OwnedReadGuard<K, V> {
        let guard = self.0.obtain(key.clone()).read_owned().await;
        OwnedReadGuard {
            key: KeyRef::new(&self.0, key),
            guard: ManuallyDrop::new(guard),
        }
    }

    pub async fn write(&self, key: K) -> OwnedWriteGuard<K, V> {
        let guard = self.0.obtain(key.clone()).write_owned().await;
        OwnedWriteGuard {
            key: KeyRef::new(&self.0, key),
            guard: ManuallyDrop::new(guard),
        }
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<K: Eq + Hash, V> Empty for KeyMutex<K, V> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl<K: Eq + Hash, V> Empty for KeyRwLock<K, V> {
    fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
