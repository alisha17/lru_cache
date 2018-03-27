use std::ptr;
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::Hash;

// The purpose of a cache is to store in memory a precomputed set of values.
// We assume that the derivation function for some reason is slow. Perhaps it's
// a complex string operation, or it's IO intensive. By cache the results of
// this derivation function, then we can quickly serve results from the cache
// when required instead of calling the derivation function. Of course this
// assumes our lookup is FASTER than the derivation function. So for example
// a derivation function that stores results from add/subtract is likely not
// a good idea because the act of computing that value is very likely to be
// faster than our lookups.
pub struct Cache<K, V> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
    // Store the nodes (pointers)
    map: BTreeMap<u64, *mut Node<K, V>>,
    // Maximum capacity of the cache, stored in number of items
    // In the future it may be better to store this as bytes, and then use the
    // Sized trait to work out how big things are (and it lets you store
    // variable size items).
    capacity: usize,
    // The derivation function. If a key isn't found, this derives it
    // and returns what the value should be. This could serve as a lookup
    // into a database, a calculation of a value or more.
    //
    // We store the copy of this here at create time because we don't want
    // the deriviation fn to change half way through the life of the cache
    // because that may cause key-value collisions in exisitng items, and
    // other weird behaviours. Better to do it correct and simple.
    derive_fn: fn(&K) -> Option<V>,
}

pub struct Node<K, V> {
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
    key_hash: u64,
    key: K,
    value: V
}


impl<K, V> Cache<K, V> where
    K: Hash + PartialEq,
    V: PartialEq
{

    // Create a new LRU cache, defining a maximum number of elements
    // that can be stored in the Cace. Additionally store a reference to the
    // cache derivation function so that missing values can be derived and
    // inserted.
    pub fn new(capacity: usize, derive_fn: fn(&K) -> Option<V>) -> Self {
        Cache {
            head: ptr::null_mut(),
            tail: ptr::null_mut(),
            map: BTreeMap::new(),
            capacity: capacity,
            derive_fn: derive_fn,
        }
    }

    // Search the LRU cache and if the key is present, cut and place the record at the end
    // If it's not present, we need to execute some function to derive the value.
    // We then return the value that was found (or created) as an option to indicate
    // possible abscence.
    //
    // This is really the only external interface required in a cache :)
    //
    // So there are five possible states
    // * The value is found in the cache -> Check the keys match, if they do,
    //     return Some(&V)
    // * The value is found in the cache -> Check the keys match, if they don't,
    //     then we clear the colliding value and derive a new one. If a value is
    //     derived, then add it to cache and return Some(&V)
    // * The value is found in the cache -> Check the keys match, if they don't,
    //     then we clear the colliding value and derive a new one. If no derivation
    //     found then return None
    // * The value is not found in the cache -> call the derive_fn, get Some<V>,
    //     insert the value to the cache, then return Some(&V)
    // * The value is not found in the cache -> carr the derive_fn, get None,
    //     return None
    //
    pub fn search_lru(&mut self, key: K) -> Option<&V> {
        // Derive the hash of the key.
        // lookup that hash
        let searched_node = self.search(key);
        if searched_node.is_null() {
            // Ruh roh - we didn't find it. We better derive a value, and insert it
            // We need to check the content of the derivation also, as it may ALSO be
            // None!

            // Remember push needs our hash too!
            None
        } else {
            // We found a value! Yay! Now we need to move it to the "most used"
            // position of the list.
            self.cut(key);
            None
        }
    }

    // Change the maximum capacity of the cache. A size of 0 is valid, it just
    // means we do nothing but return content from the derivation fn. If capacity
    // changes down, then we need to trim possibly excess values. If it goes up
    // it's just a simple value set, and future searches will have more capacity.
    pub fn resize(&mut self, new_capacity: usize) {
        if new_capacity < self.capacity {
            // Do the resize via cleanup.
        }
        self.capacity = new_capacity;
    }

    /* INTERNAL IMPLEMENTATION DETAILS */

    // Push a new element to the cache
    // This means we have a new node to create and insert. We already have the
    // hash from the search function.
    fn push(&mut self, key_hash: u64, key: K , value: V) {

        // First make space in the cache (if needed)
        self.cleanup(self.capacity);

        let new_tail_box = Box::new(Node {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            key_hash: key_hash,
            key: key,
            value: value,
        });

        let new_tail: *mut _ = Box::into_raw(new_tail_box);

        self.map.insert(hash , new_tail);

        if self.tail.is_null() {
            // If the cache is empty
            self.head = new_tail;
        } else {
            //If the cache is non-empty
            unsafe {
                (*new_tail).prev = self.tail;
                (*self.tail).next = new_tail;
            }
        }

        self.tail = new_tail;
    }

    // Pop the last element from the cache
    // In order to uphold Rust's memory safety, at this point because we are
    // going to dispose of the content, we re-box the node, and then hand the
    // node back to the caller (so that it has the hashes, key, value.
    fn pop(&mut self) -> Option<Node<K, V>> {

            if self.head.is_null() {
                // Don't change tail here! We should set tail mut on
                // an actual pop.
                // self.tail = ptr::null_mut();
                None
            } else {

                let box_head = unsafe {
                    // Retake ownership from a raw pointer into a box. Now this
                    // will be freed properly when we go out of scope.
                    Box::from_raw(self.head)
                };
                // Update the head to our next pointer.
                self.head = box_head.next;
                if self.head.is_null() {
                    // There are no more nodes! Tail must also be null too!
                    self.tail = ptr::null_mut();
                } else {
                    // There is more content, update our head to have no
                    // previous
                    unsafe{
                        (*self.head).prev = ptr::null_mut();
                    }
                }
                // Remove the pointer from the map. We don't need to capture
                // this value because it's just a pointer, and we already have
                // it boxed. So only the pointer value goes out of scope.
                self.map.remove(box_head.key_hash);
                // Now make the node safe, don't leave dangling bits
                // We don't want anyone to be able to find our internal
                // details when we give this back!
                box_head.prev = ptr::null_mut();
                box_head.next = ptr::null_mut();
                // Done! Return the node ...
                Some(box_head)
        }
    }

    // Cut the node if the key is present in the map and place it at the front (i.e. 
    // recently used)
    fn cut(&mut self, key: K) {
        let searched_node = self.search(key);

        if searched_node == ptr::null_mut() {
        }
        else {
            if self.head == searched_node {
                unsafe {
                    let new_node = self.head;
                    self.head = (*self.head).next;
                    (*self.head).prev = ptr::null_mut();
                    let second_last = self.tail;
                    self.tail = new_node;
                    (*self.tail).prev = second_last;
                    (*self.tail).next = ptr::null_mut();
                    (*second_last).next = new_node;
                }
            }
            else if self.tail == searched_node {}
            else {
                unsafe {
                    let mut current = (*self.head).next;

                    if current == searched_node {
                        // Deleting the node from the list
                        (*(*current).next).prev = (*current).prev;
                        (*(*current).prev).next = (*current).next;
                        (*current).prev = self.tail;
                        (*self.tail).next = current;
                        (*current).next = ptr::null_mut();
                        self.tail = current;
                    }
                    else {
                        current = (*current).next;
                  }   
                }
            }
        }
    }

    // In case the cache is full, cleanup() pops the least recently used node if length
    // of BTreemap is greater than the length of the cache and removes it from the map
    //
    // We need to take capacity here as an argument due to the design of the
    // resize function - we need to be able to take in an external value.
    fn cleanup(&mut self, capacity: usize) {
        while self.map.len() > capacity {
            // We now own the node, so it will be freed here when we go out of
            // scope
            let _popped = self.pop();
            // We don't need to do anything else, because the pop function
            // cleans up the map references and our list values.
        }
    }
 
    // Derives the hash from a key value.
    fn key_to_hash(&mut self, key: &K) -> u64 {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        hasher.finish()
    }

    // Search if the key is present in the cache and returns the value, i.e.
    // the node
    fn search(&mut self, key: K) -> *mut Node<K, V>{
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();;

        match self.map.get_mut(&hash) {
            // Dereference the raw pointer
            Some(value) => return *value,
            None => ptr::null_mut()
        }
    }
 
    // Verify the state of the cache after every operation; more
    // conditons to be added.
    // This is NOT mut, because we never mutate the tree! We read-only
    // check it :)
    fn verify(&self) {

        // First, do we have any values in the cache?
        if self.map.len() > 0 {

            // Assert that size < capacity

            // If we do, then check that head / tail assertions match
            unsafe {
                assert_eq!((*self.head).prev, ptr::null_mut());
                assert_eq!((*self.tail).next, ptr::null_mut());
            }
            // Walk the cache validating all the hashes of values

            // Walk the list forward AND backward to make sure that everything
            // is correctly linked
        } else {
            // If empty, validate that all values are NULL
            assert_eq!(self.head, ptr::null_mut());
            assert_eq!(self.tail, ptr::null_mut());
        }

    } 
}



#[cfg(test)]
mod tests {

    use super::Cache;
    use std::ptr;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;


    #[test]
    fn test_push_cache_full() {

        let mut list = Cache::<u32, u32>::new(3);

        list.push(10, 1);
        list.push(20, 2);
        list.push(30, 3);
        list.push(40, 4);

        assert_eq!(list.pop(), Some((20, 2)));
        assert_eq!(list.pop(), Some((30, 3)));
        assert_eq!(list.pop(), Some((40, 4)));
    }

    #[test]
    fn test_hash_not_equal() {

        let mut hasher = DefaultHasher::new();
        hasher.write_u64(1);
        let hashed_key = hasher.finish();

        hasher.write_u64(2);
        let hashed_key2 = hasher.finish();

        assert_ne!(hashed_key, hashed_key2);
    }

    #[test]
    fn test_search_not_equal_key() {

        let mut cache = Cache::<u32, u32>::new(20);

        cache.push(10, 1);
        cache.push(20, 2);
        cache.push(30, 3);

        assert_ne!(cache.search(10), ptr::null_mut());
    }

    #[test]
    fn test_cut_first_elem() {

        let mut cache = Cache::<u32, u32>::new(20);

        cache.push(10, 1);
        cache.push(20, 2);
        cache.push(30, 3);

        cache.cut(10);

        assert_eq!(cache.pop(), Some((20, 2)));
        assert_eq!(cache.pop(), Some((30, 3)));
        assert_eq!(cache.pop(), Some((10, 1)));
    }

    #[test]
    fn test_cut_last_elem() {

        let mut cache = Cache::<u32, u32>::new(20);

        cache.push(10, 1);
        cache.push(20, 2);
        cache.push(30, 3);

        cache.cut(30);

        assert_eq!(cache.pop(), Some((10, 1)));
        assert_eq!(cache.pop(), Some((20, 2)));
        assert_eq!(cache.pop(), Some((30, 3)));
    }  

    #[test]
    fn test_cut() {

        let mut cache = Cache::<u32, u32>::new(20);

        cache.push(10, 1);
        cache.push(20, 2);
        cache.push(30, 3);

        cache.cut(20);

        assert_eq!(cache.pop(), Some((10, 1)));
        assert_eq!(cache.pop(), Some((30, 3)));
        assert_eq!(cache.pop(), Some((20, 2)));
    }

    #[test]
    fn test_search_lru() {
        let mut cache = Cache::<u32, u32>::new(20);

        cache.push(10, 1);
        cache.push(20, 2);
        cache.push(30, 3);

        cache.search_lru(20);

        assert_eq!(cache.pop(), Some((10, 1)));
        assert_eq!(cache.pop(), Some((30, 3)));
        assert_eq!(cache.pop(), Some((20, 2)));
    }
}
