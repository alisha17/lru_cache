use std::ptr;
use std::fmt::Debug;
use std::collections::BTreeMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::hash::Hash;

// fn main(){
// }

#[derive(Debug)]
pub struct Cache<K, V> {
    head: *mut Node<K, V>,
    tail: *mut Node<K, V>,
    map: BTreeMap<u64, *mut Node<K, V>>, // Store the nodes (pointers)
    capacity: usize // Maximum capacity of the cache
}

#[derive(Debug)]
pub struct Node<K, V> {
    prev: *mut Node<K, V>,
    next: *mut Node<K, V>,
    key_hash: u64,
    key: K,
    value: V
}


impl<K, V> Cache<K, V> where K:Hash, V: Debug+PartialEq {

    // Create a new LRU cache
    pub fn new(capacity: usize) -> Self {
        Cache { 
                head: ptr::null_mut(), 
                tail: ptr::null_mut(), 
                map: BTreeMap::new(),
                capacity: capacity,
        }
    }

    // Push a new element to the cache
    pub fn push(&mut self, key: K , value: V) {

        let mut hasher = DefaultHasher::new();
        hasher.write(key);
        let hashed_key = hasher.finish();

        let new_tail_box = Box::new(Node {
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
            key_hash: hashed_key,
            key: key,
            value: value,
        });

        let new_tail: *mut _ = Box::into_raw(new_tail_box);

        self.map.insert(hashed_key , new_tail);
                
        // In case the cache is already full
        // if self.map.len() > self.capacity {
            
        // }
        // else {
            // If the cache is non-empty
            if !self.tail.is_null() {
                unsafe {
                    (*new_tail).prev = self.tail;
                    (*self.tail).next = new_tail;
                }
            } 
            else {
                // If the cache is empty
                self.head = new_tail;
            }
    // }

        self.tail = new_tail;
    }

    // Pop the last element from the cache
    pub fn pop(&mut self) -> Option<V> {
        unsafe{
            (*self.head).prev = ptr::null_mut();
        }

        if self.head.is_null() {
            self.tail = ptr::null_mut();
            None
        }
        else {
            let box_head = unsafe { Box::from_raw(self.head) };
            self.head = box_head.next;
            self.pop_from_map(box_head.key_hash);
            Some(box_head.value)
        }
    }

     pub fn pop_from_map(&mut self, key_hash: u64) {
        let value_node = self.map.remove(&key_hash);
        Box::from_raw(value_node.unwrap());
     }

    // fn cleanup(&mut self, capacity: usize) {
    //     while self.map.len() >= capacity {
    //         let poped = self.pop();

    //         if poped != None {
    //             self.map.remove(poped);
    //             let x = Box::from_raw(poped);
    //         }
    //     }
    // }

    pub fn cut(&mut self, key: K) {
        let searched_node = self.search(key);

        if searched_node == ptr::null_mut() {
            println!("The element does not exist!");
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

    pub fn search(&mut self, key: K) -> *mut Node<K, V>{
        let mut hasher = DefaultHasher::new();
        hasher.write(key);
        let hashed_key = hasher.finish();

        match self.map.get_mut(&hashed_key) {
            // Dereference the raw pointer
            Some(value) => return *value,
            None => ptr::null_mut()
        }
    }
}

#![feature(core_intrinsics)]

use std::intrinsics::type_name;

fn test_type<T>(_: T) {
    println!("{:?}", unsafe { type_name::<T>() });
}

fn main() {
    test_type(5);
}

#[cfg(test)]
mod tests {

    use super::Cache;
    use std::ptr;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    #[test]
    fn test_push_and_pop() {
        
        let mut list = Cache::<u32, u32 >::new(20);

        list.push(10, 1);
        list.push(20, 2);
        list.push(30, 3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));        
    }

    #[test]
    fn test_push_cache_full() {

        let mut list = Cache::<u32, u32>::new(3);

        list.push(10, 1);
        list.push(20, 2);
        list.push(30, 3);
        list.push(40, 4);

        // Check normal removal
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(4));
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
    fn test_hash_equal() {

        let mut hasher = DefaultHasher::new();
        hasher.write_u64(1);
        let hashed_key = hasher.finish();

        let mut hasher2 = DefaultHasher::new();
        hasher2.write_u64(1);
        let hashed_key2 = hasher2.finish();

        assert_eq!(hashed_key, hashed_key2);
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

        assert_eq!(cache.pop(), Some(2));
        assert_eq!(cache.pop(), Some(3));
        assert_eq!(cache.pop(), Some(1));

    }

    #[test]
    fn test_cut_last_elem() {

        let mut cache = Cache::<u32, u32>::new(20);

        cache.push(10, 1);
        cache.push(20, 2);
        cache.push(30, 3);

        cache.cut(30);

        assert_eq!(cache.pop(), Some(1));
        assert_eq!(cache.pop(), Some(2));
        assert_eq!(cache.pop(), Some(3));

    }  

    #[test]
    fn test_cut() {

        let mut cache = Cache::<u32, u32>::new(20);

        cache.push(10, 1);
        cache.push(20, 2);
        cache.push(30, 3);

        cache.cut(20);

        assert_eq!(cache.pop(), Some(1));
        assert_eq!(cache.pop(), Some(3));
        assert_eq!(cache.pop(), Some(2));

    }
}

