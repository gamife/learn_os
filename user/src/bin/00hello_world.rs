#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;
#[macro_use]
extern crate log;

#[no_mangle]
fn main() -> i32 {
    println!("hello world");
    error!("hello world");
    0
}
