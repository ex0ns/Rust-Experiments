use std::mem;
use std::ptr;

pub struct RawLink<T> {
    p: *mut T,
}

impl<T> RawLink<T> {

    pub fn resolve(&mut self) -> Option<&mut T> {
        if self.p.is_null() {
            None
        } else {
            Some(unsafe { mem::transmute(self.p) })
        }
    }

    pub fn none() -> RawLink<T> {
        RawLink{p: ptr::null_mut()}
    }

    pub fn some(p: &mut T) -> RawLink<T> {
       RawLink { p: p } 
    }

}
