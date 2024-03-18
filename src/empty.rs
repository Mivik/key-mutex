use std::{
    collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque},
    hash::Hash,
    rc::Rc,
    sync::Arc,
};

use dashmap::{DashMap, DashSet};

pub trait Empty {
    fn is_empty(&self) -> bool;
}

impl<T: Empty> Empty for Box<T> {
    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

impl<T: Empty> Empty for Rc<T> {
    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

impl<T: Empty> Empty for Arc<T> {
    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }
}

impl Empty for () {
    fn is_empty(&self) -> bool {
        true
    }
}

impl<V> Empty for Option<V> {
    fn is_empty(&self) -> bool {
        self.is_none()
    }
}

impl<V> Empty for Vec<V> {
    fn is_empty(&self) -> bool {
        Vec::is_empty(self)
    }
}

impl<V> Empty for VecDeque<V> {
    fn is_empty(&self) -> bool {
        VecDeque::is_empty(self)
    }
}

impl<V> Empty for LinkedList<V> {
    fn is_empty(&self) -> bool {
        LinkedList::is_empty(self)
    }
}

impl<V> Empty for BinaryHeap<V> {
    fn is_empty(&self) -> bool {
        BinaryHeap::is_empty(self)
    }
}

impl<K, V> Empty for BTreeMap<K, V> {
    fn is_empty(&self) -> bool {
        BTreeMap::is_empty(self)
    }
}

impl<V> Empty for BTreeSet<V> {
    fn is_empty(&self) -> bool {
        BTreeSet::is_empty(self)
    }
}

impl<K, V> Empty for HashMap<K, V> {
    fn is_empty(&self) -> bool {
        HashMap::is_empty(self)
    }
}

impl<V> Empty for HashSet<V> {
    fn is_empty(&self) -> bool {
        HashSet::is_empty(self)
    }
}

impl<K: Eq + Hash, V> Empty for DashMap<K, V> {
    fn is_empty(&self) -> bool {
        DashMap::is_empty(self)
    }
}

impl<K: Eq + Hash> Empty for DashSet<K> {
    fn is_empty(&self) -> bool {
        DashSet::is_empty(self)
    }
}
