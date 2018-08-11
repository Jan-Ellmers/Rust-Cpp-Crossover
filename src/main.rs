pub extern crate libc;
use libc::{c_int};



#[link(name = "test")]
extern "C" {
    fn hello_world () -> c_int;
}


fn main() {
    let x;
    unsafe {
        x = hello_world();
    }
    println!("x ist: {}", x);
}

