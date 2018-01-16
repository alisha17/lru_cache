use std::ptr;
use std::fmt::Debug;

fn main(){
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
pub struct List<T> {
    head: Link<T>,
    tail: *mut Node<T>,
}

#[derive(Debug)]
struct Node<T> {
    prev: *mut Node<T>,
    elem: T,
    next: Link<T>,
}

impl<T> List<T> where T: Debug {
    pub fn new() -> Self {
        List { head: None, tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let mut new_tail = Box::new(Node {
            prev: ptr::null_mut(),
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                new_tail.prev = self.tail;
                (*self.tail).next = Some(new_tail);
            }
        } else {
            self.head = Some(new_tail);
        }

        self.tail = raw_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|head| {
            let head = *head;
            self.head = head.next;

            match self.head.as_mut() {
                Some(x) => x.prev = ptr::null_mut(),
                None => {},
            }
            
            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem

        })
    }

    // pub fn cut(&mut self, elem:T) {
    //     let mut current = self.head;
    //     let mut last_elem = self.tail;

    //     while current.is_some() {
    //         if current.as_ref().elem == elem {
    //             let current_elem = current.as_ref().elem;
    //             (current_elem.prev).next = current_elem.next;
    //             (current_elem.next).prev = current_elem.prev;
    //             current_elem.next = None;
    //             current_elem.prev = self.tail;
    //         }
    //     }
    // }
    
}
#[cfg(test)]
mod tests {

    use super::List;

    #[test]
    fn test_push() {

       let mut list = List::<u32>::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3));        
        assert_eq!(list.pop(), None);

    }
}


// linked btree map
// get key and value

