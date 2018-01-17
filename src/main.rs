use std::ptr;
use std::fmt::Debug;

fn main(){
}

#[derive(Debug)]
pub struct List<T> {
    head: *mut Node<T>,
    tail: *mut Node<T>,
}

#[derive(Debug)]
struct Node<T> {
    prev: *mut Node<T>,
    elem: T,
    next: *mut Node<T>,
}

impl<T> List<T> where T: Debug+PartialEq {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail_box = Box::new(Node {
            prev: ptr::null_mut(),
            elem: elem,
            next: ptr::null_mut(),
        });

        let new_tail = Box::into_raw(new_tail_box);

        if !self.tail.is_null() {
            unsafe {
                (*new_tail).prev = self.tail;
                (*self.tail).next = new_tail;
            }
        } else {
            (*self.head) = new_tail;
        }

        (*self.tail) = new_tail;
    }

    pub fn pop(&mut self) -> T {
            (*self.head).prev = ptr::null_mut();
            
            if self.head.is_null() {
                self.tail = ptr::null_mut();
            }
            (*self.head).elem
    }

    pub fn cut(&mut self, elem:T) {
        let mut current = self.head;
        let mut last_elem = self.tail;

        while !current.is_null() {
            if (*current).elem == elem {
                unsafe {
                   (*(*current).prev).next = (*current).next;
                   (*(*current).next).prev = (*current).prev;
                    (*current).next = ptr::null_mut();
                    (*current).prev = self.tail
                } 
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::List;

    #[test]
    fn test_push() {

       let mut list = List::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), 1);
        assert_eq!(list.pop(), 2);
        assert_eq!(list.pop(), 3);        
    }
}


// linked btree map
// get key and value

