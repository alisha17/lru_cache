mod btree;

use std::collections::BTreeMap;
mod linkedlist;

pub struct LruCache {
    map: BTreeMap<i64, linkedlist::List<T>>,
    count: usize,
}

impl LruCache {
    pub fn new(capacity: usize) -> LruCache {
        LruCache {
            map: BTreeMap::new(),
            count: capacity,
        }
    }
}

//     pub fn insert(key: , value:) -> 
// }