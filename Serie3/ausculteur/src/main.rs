fn print_binary(c: u8) {
    if c != 0 {
        print_binary(c >> 1);
        print!("{}", c&1);
    }
}

fn print(index: usize, c: u8) {
    print!(" Offset {} : ", index);
    print_binary(c);
    print!(" {} ", c);
    if c >= 32 && c <= 126 {
        print!("{}", c as char);
    }
    println!("");
}

fn dump_mem(size: usize, ptr: &u8) {
    let mut ptr = ptr as *const u8;
    for i in 0..size {
        print!("Address: {:p} ", ptr);
        print(i, unsafe {*ptr as u8});
        unsafe { ptr = ptr.offset(1); }
    }
}

fn main() {
    let a = 'P';
    let _b = 'O';
    dump_mem(2, &(a as u8));
}
