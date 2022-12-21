//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

#![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
// #![feature(panic_info_message)]


mod lang_items;

// 初始化栈
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

/// 程序入口
#[no_mangle]
pub fn rust_main() -> !{
    loop{}
}

/// 将bss段清零
fn clear_bss(){
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        // TODO: 验证为啥用 *mut u8
        // 我猜是地址每加1, 就是一个字节, 比如, 
        // 0x01 8bit
        // 0x02 8bit
        // 所以一个字节的内存, 可以当做u8看待.
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}