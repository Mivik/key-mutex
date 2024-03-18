use dashmap::DashMap;
use std::{hash::Hash, mem::ManuallyDrop, ops::Deref, sync::Arc};

use crate::Empty;

pub struct LockMap<K, L>(Arc<DashMap<K, Arc<L>>>);
impl<K: Eq + Hash, L> LockMap<K, L> {
    pub fn new() -> Self {
        Self(Arc::new(DashMap::new()))
    }

    #[inline]
    pub fn remove_if(&self, key: &K, f: impl FnOnce(&Arc<L>) -> bool) {
        self.0.remove_if(key, |_, v| f(v));
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl<K: Eq + Hash, L: Default> LockMap<K, L> {
    pub fn obtain(&self, key: K) -> Arc<L> {
        self.0.entry(key).or_insert_with(Arc::default).clone()
    }
}

impl<K, L> Clone for LockMap<K, L> {
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}
impl<K: Eq + Hash, L> Default for LockMap<K, L> {
    fn default() -> Self {
        Self(Arc::default())
    }
}

pub struct KeyRef<K, L> {
    map: LockMap<K, L>,
    pub key: K,
}
impl<K: Eq + Hash, L> KeyRef<K, L> {
    pub fn new(map: &LockMap<K, L>, key: K) -> Self {
        Self {
            map: map.clone(),
            key,
        }
    }

    pub fn on_drop<V: Empty, G: Deref<Target = V>>(&self, guard: &mut ManuallyDrop<G>) {
        if guard.is_empty() {
            let guard = unsafe { ManuallyDrop::take(guard) };
            self.map.remove_if(&self.key, |v| {
                std::mem::drop(guard);
                Arc::strong_count(v) == 1
            });
        } else {
            unsafe { ManuallyDrop::drop(guard) };
        }
    }
}

macro_rules! decl_mutex_guard {
    ($guard:ident) => {
        pub struct OwnedMutexGuard<K: Eq + Hash, V: Empty + 'static> {
            key_ref: KeyRef<K, sync::Mutex<V>>,
            guard: ManuallyDrop<$guard<V>>,
        }

        impl<K: Eq + Hash, V: Empty> OwnedMutexGuard<K, V> {
            pub fn key(&self) -> &K {
                &self.key_ref.key
            }
        }

        impl<K: Eq + Hash, V: Empty> Deref for OwnedMutexGuard<K, V> {
            type Target = V;

            fn deref(&self) -> &Self::Target {
                self.guard.deref()
            }
        }

        impl<K: Eq + Hash, V: Empty> DerefMut for OwnedMutexGuard<K, V> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.guard.deref_mut()
            }
        }

        impl<K: Eq + Hash, V: Empty> Drop for OwnedMutexGuard<K, V> {
            fn drop(&mut self) {
                self.key_ref.on_drop(&mut self.guard);
            }
        }
    };
}

macro_rules! decl_rwlock_guard {
    ($read_guard:ident, $write_guard:ident) => {
        pub struct OwnedReadGuard<K: Eq + Hash, V: Empty + 'static> {
            key: KeyRef<K, sync::RwLock<V>>,
            guard: ManuallyDrop<$read_guard<V>>,
        }

        impl<K: Eq + Hash, V: Empty> OwnedReadGuard<K, V> {
            pub fn key(&self) -> &K {
                &self.key.key
            }
        }

        impl<K: Eq + Hash, V: Empty> Deref for OwnedReadGuard<K, V> {
            type Target = V;

            fn deref(&self) -> &Self::Target {
                self.guard.deref()
            }
        }

        impl<K: Eq + Hash, V: Empty> Drop for OwnedReadGuard<K, V> {
            fn drop(&mut self) {
                self.key.on_drop(&mut self.guard);
            }
        }

        pub struct OwnedWriteGuard<K: Eq + Hash, V: Empty + 'static> {
            key: KeyRef<K, sync::RwLock<V>>,
            guard: ManuallyDrop<$write_guard<V>>,
        }

        impl<K: Eq + Hash, V: Empty> OwnedWriteGuard<K, V> {
            pub fn key(&self) -> &K {
                &self.key.key
            }
        }

        impl<K: Eq + Hash, V: Empty> Deref for OwnedWriteGuard<K, V> {
            type Target = V;

            fn deref(&self) -> &Self::Target {
                self.guard.deref()
            }
        }

        impl<K: Eq + Hash, V: Empty> DerefMut for OwnedWriteGuard<K, V> {
            fn deref_mut(&mut self) -> &mut Self::Target {
                self.guard.deref_mut()
            }
        }

        impl<K: Eq + Hash, V: Empty> Drop for OwnedWriteGuard<K, V> {
            fn drop(&mut self) {
                self.key.on_drop(&mut self.guard);
            }
        }
    };
}
