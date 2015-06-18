use std::mem;
use super::segment;
use super::direction;
use super::rawlink;
use super::map;

pub struct Snake {
    pub head: Option<Box<segment::Segment>>,
    pub tail: rawlink::RawLink<segment::Segment>,
}

impl Snake {

    pub fn new() -> Snake {
        let mut head = Box::new(segment::Segment::new());
        Snake { tail: rawlink::RawLink::some(&mut *head), head: Some(head) }
    }

    pub fn step(&mut self, map: &mut map::Map, dir: direction::Direction) {
        let mut new_head = false;
        match self.head {
            Some(ref mut head) => {
                if head.dir == dir {
                    head.increase();
                    head.update();
                } else {
                    new_head = true;
                }
            }
            None => panic!("Could not move an empty snake !")
        };
        if new_head { self.new_head(dir); }
        map.set_snake_head(self);
        self.move_tail(map);
        self.check_bounds();
        let mut to_eat = false;
        match self.head {
            Some(ref mut head) => {
                if map.is_food(head.x, head.y) { to_eat = true; }
            }
            None => panic!("No head")
        };
        if to_eat { self.eat(); }
    }

    fn check_bounds(&mut self) {
    }

    fn move_tail(&mut self, map: &mut map::Map) {
        self.tail = match self.tail.resolve() {
            Some(tail) => {
                let x = tail.x - (tail.size as i32 -1)*tail.dir.to_x() as i32;
                let y = tail.y - (tail.size as i32 -1)*tail.dir.to_y() as i32;
                map.move_snake_tail(x, y);
                tail.decrease()
            }
            None => {
                panic!("Snake should not be empty");
            }
        }
    }

    fn eat(&mut self) {
        match self.tail.resolve() {
            Some(ref mut tail) => {
                tail.increase();
                tail.update();
            }
            None => { panic!("Could not eat without a head"); }
        };
    }

    fn new_head(&mut self, dir: direction::Direction) {
        match self.head {
            Some(ref mut head) => {
                let mut new_head = Box::new(segment::Segment::new());
                new_head.x = head.x + dir.to_x() as i32;
                new_head.y = head.y + dir.to_y() as i32;
                new_head.dir = dir;
                head.prev = rawlink::RawLink::some(&mut *new_head);
                mem::swap(head, &mut new_head);
                head.next = Some(new_head);
            }
            None => {} 
        };
    }
}
