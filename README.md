
# KeyMutex

## What is KeyMutex?

`KeyMutex<K, V>` is basically a lock-free `HashMap<K, Mutex<V>>` (or `RwLock`) with a few extra features.

## Why do I need it?

Imagine you have a bunch of users identified by their IDs, and you want to have a lock for each user. That's when a `KeyMutex<K, ()>` comes in handy. The difference from `DashMap<K, Mutex<()>>` is that `KeyMutex` will automatically release the entry when the last mutex guard is dropped.

Things might get trickier when you want to, say, maintain a user's active sessions. You need to have a `KeyMutex<K, Vec<Session>>`, and you certainly don't want the entry be dropped unless the sessions are empty. Don't worry, `KeyMutex` handles this for you. The entry will be dropped only when `Empty::is_empty` returns `true`, which is implemented for all collections in the standard library and also, of course, for `KeyMutex`.
