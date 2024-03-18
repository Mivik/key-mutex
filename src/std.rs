//! Implementations of [`KeyMutex`] and [`KeyRwLock`] using `std::sync::{Mutex, RwLock}`.

use crate::{
    inner::{KeyRef, LockMap},
    Empty,
};
use guardian::{ArcMutexGuardian, ArcRwLockReadGuardian, ArcRwLockWriteGuardian};
use std::{
    hash::Hash,
    mem::ManuallyDrop,
    ops::{Deref, DerefMut},
    sync::{self, LockResult, PoisonError},
};

fn map_guard<G, R>(result: LockResult<G>, f: impl FnOnce(G) -> R) -> LockResult<R> {
    match result {
        Ok(guard) => Ok(f(guard)),
        Err(poisoned) => Err(PoisonError::new(f(poisoned.into_inner()))),
    }
}

decl_mutex_guard!(ArcMutexGuardian);
#[derive(Clone, Default)]
pub struct KeyMutex<K: Eq + Hash, V>(LockMap<K, sync::Mutex<V>>);
impl<K: Eq + Hash + Clone, V: Empty + Default> KeyMutex<K, V> {
    pub fn new() -> Self {
        Self(LockMap::new())
    }

    pub fn lock(&self, key: K) -> LockResult<OwnedMutexGuard<K, V>> {
        let lock = self.0.obtain(key.clone());
        map_guard(ArcMutexGuardian::take(lock), |guard| OwnedMutexGuard {
            key_ref: KeyRef::new(&self.0, key),
            guard: ManuallyDrop::new(guard),
        })
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

decl_rwlock_guard!(ArcRwLockReadGuardian, ArcRwLockWriteGuardian);
#[derive(Clone, Default)]
pub struct KeyRwLock<K: Eq + Hash, V>(LockMap<K, sync::RwLock<V>>);
impl<K: Eq + Hash + Clone, V: Empty + Default> KeyRwLock<K, V> {
    pub fn new() -> Self {
        Self(LockMap::new())
    }

    pub fn read(&self, key: K) -> LockResult<OwnedReadGuard<K, V>> {
        let lock = self.0.obtain(key.clone());
        map_guard(ArcRwLockReadGuardian::take(lock), |guard| OwnedReadGuard {
            key: KeyRef::new(&self.0, key),
            guard: ManuallyDrop::new(guard),
        })
    }

    pub fn write(&self, key: K) -> LockResult<OwnedWriteGuard<K, V>> {
        let lock = self.0.obtain(key.clone());
        map_guard(ArcRwLockWriteGuardian::take(lock), |guard| {
            OwnedWriteGuard {
                key: KeyRef::new(&self.0, key),
                guard: ManuallyDrop::new(guard),
            }
        })
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

#[cfg(test)]
mod test {
    use crate::KeyMutex;
    use std::collections::BTreeSet;

    #[test]
    fn drop_only_if_empty() {
        let locks = KeyMutex::<u32, BTreeSet<String>>::new();

        let mut lock = locks.lock(1).unwrap();
        lock.insert("Hello".to_owned());
        lock.insert("World".to_owned());
        drop(lock);

        // Value is not empty and thus is not dropped
        assert_eq!(locks.len(), 1);

        let mut lock = locks.lock(1).unwrap();
        assert_eq!(lock.len(), 2);
        lock.clear();
        drop(lock);

        // Should be dropped now
        assert_eq!(locks.len(), 0);
    }
}
