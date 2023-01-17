#![no_std]
#![feature(panic_info_message)]
#![feature(linkage)]

use syscall::{sys_exit, sys_write, sys_yield};

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

pub mod console;
mod lang_items;
mod logging;
pub mod syscall;

/// 程序入口
#[no_mangle]
#[link_section = ".text.entry"]

pub extern "C" fn _start() -> ! {
    clear_bss();
    logging::init();
    exit(main());
    panic!("unreachable after sys_exit");
}

#[linkage = "weak"]
#[no_mangle]
pub fn main() -> i32 {
    error!("no main function");
    panic!("cannot find main!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }

    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

pub fn write(fd: usize, buffer: &[u8]) -> isize {
    sys_write(fd, buffer)
}


pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}

pub fn yield_() -> isize {
    sys_yield()
}
