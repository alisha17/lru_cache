use std::ptr;
use std::fmt;
use std::fmt::Debug;

fn main(){
    let mut list = List::new();

    list.push(2);
    list.push(3);
    list.push(4);

    // list.cut(4);
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
            prev: None.unwrap(),
            elem: elem,
            next: None,
        });

        let raw_tail: *mut _ = &mut *new_tail;

        if !self.tail.is_null() {
            unsafe {
                (*self.tail).next = Some(new_tail);
                new_tail.prev = *self.tail;
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

            if self.head.is_none() {
                self.tail = ptr::null_mut();
            }

            head.elem

        })
    }
    
    // pub fn cut(&mut self, elem:T) {  
    //    let mut current = &self.head;
     
    //    let mut prev: Option<T> = None;

    //    println!("Previous element {:?}", prev);

    //    while current.is_some() {
    //        match current.as_ref() {
    //          Some(x) => println!("Current element {:?}", x.elem),
    //          None => {},
    //        }
           
    //        let next = &current.as_ref().unwrap().next;

    //        match next.as_ref() {
    //          Some(x) => println!("Next element {:?}", x.elem),
    //          None => println!("Next element None"),
    //        }
           
    //        let prev = &current.as_ref();

    //        match prev {
    //            Some(x) => println!("Previous element {:?}", prev),
    //            None => {}
    //        }

    //        current = &current.as_ref().unwrap().next;     

    //    } 
 
    // }     
}


// linked btree map
// get key and value

