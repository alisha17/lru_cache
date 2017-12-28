use std::collections::btree_map;

pub struct LruCache {
    map: btree_map<i64, String>,
    max_size: usize,
}

impl LruCache {
    pub fn new(capacity: usize) -> LruCache {
         LruCache {
            map: btree_map::new(),
            max_size: capacity,
        }
    }

    pub fn insert(&mut self, k: i64, v: String) -> Option<String> {
        let old_val = self.map.insert(k, v);
        if self.len() > self.capacity() {
            self.remove_lru();
        }
        old_val
    }

    // pub fn remove_lru() -> 

}

// linked btree map
// get key and value