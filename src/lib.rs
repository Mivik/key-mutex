//! A concurrent, lock-free version of `HashMap<K, Mutex<V>>` and `HashMap<K, RwLock<V>>`.
//!
//! It automatically allocates mutexes and rwlocks for you, and it automatically deallocates
//! them when they are no longer in use. The "no longer in use" condition is when the last
//! guard is dropped AND [`Empty::is_empty`] returns `true`.
//!
//! It's a bit like [Web Locks API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Locks_API)
//! when `V` is `()`.
//!
//! # Example
//!
//! ```
//! use std::sync::Arc;
//! use key_mutex::KeyMutex;
//!
//! fn main() {
//!     let locks = KeyMutex::<u32, BTreeSet<String>>::new();
//!     let mut lock = locks.lock(1).unwrap();
//!     lock.insert("Hello".to_owned());
//!     lock.insert("World".to_owned());
//!     drop(lock);
//!
//!     // Value is not empty and thus is not dropped
//!     assert_eq!(locks.len(), 1);
//!
//!     let mut lock = locks.lock(1).unwrap();
//!     assert_eq!(lock.len(), 2);
//!     lock.clear();
//!     drop(lock);
//!
//!     // Should be dropped now
//!     assert_eq!(locks.len(), 0);
//! }

mod empty;
#[macro_use]
mod inner;

#[cfg(feature = "std")]
pub mod std;
#[cfg(feature = "tokio")]
pub mod tokio;

pub use empty::Empty;
#[cfg(feature = "std")]
pub use std::*;
