use std::mem;
use super::segment;
use super::direction;
use super::rawlink;

pub struct Snake {
    pub x: i32,
    pub y: i32,
    pub head: Option<Box<segment::Segment>>,
    pub tail: rawlink::RawLink<segment::Segment>,
}

impl Snake {

    pub fn new() -> Snake {
        let mut head = Box::new(segment::Segment::new());
        Snake { tail: rawlink::RawLink::some(&mut *head), head: Some(head), x: 0, y: 0 }
    }

    pub fn step(&mut self, dir: direction::Direction) {
        let mut new_head = false;
        match self.head {
            Some(ref mut head) => {
                if head.dir == dir {
                    head.increase();
                } else {
                    new_head = true;
                }
            }
            None => panic!("Could not move an empty snake !")
        };
        if new_head { self.new_head(dir); }
        self.x += dir.to_x() as i32;
        self.y += dir.to_y() as i32;
        self.move_tail();
        self.check_bounds();
    }

    fn check_bounds(&mut self) {
    }

    fn move_tail(&mut self) {
        self.tail = match self.tail.resolve() {
            Some(tail) =>  tail.decrease(),
            None => {
                panic!("Snake should not be empty");
            }
        }
    }

    pub fn eat(&mut self) {
        match self.head {
            Some(ref mut head) => head.increase(),
            None => { panic!("Could not eat without a head"); }
        };
    }

    fn new_head(&mut self, dir: direction::Direction) {
        match self.head {
            Some(ref mut head) => {
                let mut new_head = Box::new(segment::Segment::new());
                new_head.dir = dir;
                head.prev = rawlink::RawLink::some(&mut *new_head);
                mem::swap(head, &mut new_head);
                head.next = Some(new_head);
            }
            None => {} 
        };
    }
}
