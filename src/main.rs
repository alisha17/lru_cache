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
pub struct Node<T> {
    prev: *mut Node<T>,
    elem: T,
    next: *mut Node<T>,
}

impl<T> List<T> where T: Debug+PartialEq {
    pub fn new() -> Self {
        List { head: ptr::null_mut(), tail: ptr::null_mut() }
    }

    pub fn push(&mut self, elem: T) {
        let new_tail_box = Box::new(Node {
            prev: ptr::null_mut(),
            elem: elem,
            next: ptr::null_mut(),
        });

        let new_tail: *mut _ = Box::into_raw(new_tail_box);

        if !self.tail.is_null() {
            unsafe {
                (*new_tail).prev = self.tail;
                (*self.tail).next = new_tail;
            }
        } else {
            self.head = new_tail;
        }

        self.tail = new_tail;
    }

    pub fn pop(&mut self) -> Option<T> {
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
            Some(box_head.elem)
        }
    }

    pub fn cut(&mut self, elem:T) {
        let searched_node = self.search(elem);

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
    

    pub fn search(&mut self, elem:T) -> *mut Node<T>{
        let mut current = self.head;

        while !current.is_null() {
            unsafe {
                if (*current).elem == elem {
                    return current
                }
                else {
                    current = (*current).next;
                }
            }
        }
        ptr::null_mut()
    }
}

#[cfg(test)]
mod tests {

    use super::List;

    #[test]
    fn test_push_and_pop() {

       let mut list = List::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);


        // Check normal removal
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(3)); 
               
    }
    
    #[test]
    fn test_cut() {
        let mut list = List::<u32>::new();

        list.push(1);
        list.push(2);
        list.push(3);

        list.cut(2);

        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }
}


// linked btree map
// get key and value

