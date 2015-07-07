use std::mem;
use std::ptr;

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    tail: RawLink<Node<T>>,
}

#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
    previous: RawLink<Node<T>>,
}

#[derive(Debug)]
struct RawLink<T> {
    p: *mut T,
}

impl<T> Node<T> {
   
    fn new(value: T) -> Node<T> {
        Node { data: value, next: None, previous: RawLink::none() }
    }
}

impl<T> RawLink<T> {

    fn resolve<'a>(&mut self) -> Option<&'a mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    fn none() -> RawLink<T> {
        RawLink{p: ptr::null_mut()}
    }

    fn some(p: &mut T) -> RawLink<T> {
       RawLink { p: p } 
    }
    
}

impl<T> LinkedList<T> {
    
    fn new() -> LinkedList<T> {
        LinkedList { head: None, tail: RawLink::none() } 
    }

    fn push_back(&mut self, elem: T) {
        let mut new_tail = Box::new(Node::new(elem));
        match self.tail.resolve() {
            None => {
                self.tail = RawLink::some(&mut *new_tail);  
                self.head = Some(new_tail);
            }
            Some(tail) => {
                self.tail = RawLink::some(&mut *new_tail);
                new_tail.previous = RawLink::some(tail);
                tail.next = Some(new_tail);
            }
        };
    }

    fn push_front(&mut self, elem: T) {
        match self.head {
            None => {
                self.push_back(elem);
            }
            Some(ref mut head) => {
                let mut new_head = Box::new(Node::new(elem));
                new_head.previous = RawLink::none();
                head.previous = RawLink::some(&mut *new_head);
                mem::swap(head, &mut new_head);
                head.next = Some(new_head);
            }
        };
    }

    fn pop_back(&mut self) -> Option<Box<Node<T>>> {
        let result = match self.tail.resolve() {
            None => {
                None
            }
            Some(tail) => {
                match tail.previous.resolve() {
                    None => {
                        self.tail = RawLink::none();
                        self.head.take()
                    }
                    Some(prev) => {
                        tail.previous = RawLink::none();
                        self.tail = RawLink::some(prev);
                        prev.next.take()
                    }
                }
            }
        };

        return result;
    }

}

fn main() {
    let mut a = LinkedList::new();
//    a.push_back(5);
    a.push_front(50);
    a.push_front(55);
    println!("{:?}", a.pop_back());
    println!("{:?}", a);
}
