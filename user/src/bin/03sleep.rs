#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{get_time, yield_};

#[no_mangle]
fn main() -> i32 {
    let mut current_timer = get_time();
    let wait_for = current_timer + 3000_000;
    println!("current_timer:{current_timer}");
    while current_timer < wait_for {
        yield_();
        current_timer = get_time()
    }
    println!("current_timer:{current_timer}");
    println!("Test sleep OK!");
    0
}