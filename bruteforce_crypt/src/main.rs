#![feature(libc)]
extern crate libc;
use std::ffi::CStr;
use std::ffi::CString;
use libc::c_char;
use std::error::Error;
use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

#[link(name = "crypt")]
extern {
    pub fn crypt(key: *const u8, salt: *const u8) -> *const i8;
}

unsafe fn my_crypt(salt: &[u8] /* 0-ended */, key: &[u8] /* 0-ended */) -> &'static CStr /* 'static is a lie */ {
    let ptr = crypt(key.as_ptr(), salt.as_ptr());
    assert!(!ptr.is_null());
    CStr::from_ptr(ptr)
}

fn increase_vec_size(vec: &mut Vec<u8>) {
    for i in 0..vec.len() {
        vec[i] = 'a' as u8;
    }
    vec.push(0);
}

fn increment_vec(vec: &mut Vec<u8>, idx: usize) -> bool {
    if idx >= vec.len() - 1 {
        return false;
    }
    
    if vec[idx] == 'z' as u8 {
        vec[idx] = 'a' as u8;
        return increment_vec(vec, idx+1);
    }
    
    vec[idx] = vec[idx] + 1;
   
    true
}

fn create_hash_from_file(path: &str) ->  HashMap<String, String> {
    let mut h = HashMap::new();

    let path = Path::new(path);
    let display = path.display();

    let mut file = match File::open(&path) {
            Err(why) => panic!("couldn't open {}: {}", display, Error::description(&why)),
            Ok(file) => file,
        };

    let mut s = String::new();
    let _ = file.read_to_string(&mut s);
    let u = s.split("\n")
        .map(|name| name.split(':').collect::<Vec<_>>())
        .filter(|x| x.len() == 2)
        .collect::<Vec<Vec<_>>>();

    for x in u {
        h.insert(x[1].to_string(), x[0].to_string());
    }
    h
}

fn main() {
    let max_size = 8;
    let mut s = vec![0];
    let h = create_hash_from_file("hello");

    for size in 1..(max_size+1) {
        println!("Trying size: {}", size);
        increase_vec_size(&mut s);

        while increment_vec(&mut s, 0) {
            let crypted: &str = unsafe { std::str::from_utf8_unchecked(my_crypt(b"50\0", &s).to_bytes()) };
            
            match h.get(crypted) {
                Some(user) => println!("Password for {} is {}", user, std::str::from_utf8(&s).unwrap()),
                None => {}
            }
        }
    }
}
