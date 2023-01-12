//! The main module and entrypoint
//!
//! The operating system and app also starts in this module. Kernel code starts
//! executing from `entry.asm`, after which [`rust_main()`] is called to
//! initialize various pieces of functionality [`clear_bss()`]. (See its source code for
//! details.)
//!
//! We then call [`println!`] to display `Hello, world!`.

// #![deny(missing_docs)]
// #![deny(warnings)]
#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

// 初始化栈
use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

/// 程序入口
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logging::init();
    print_mem_layout();
    println!("hello word");
    error!("log color");

    panic!("Shutdown machine!");
}

/// 将bss段清零
fn clear_bss() {
    extern "C" {
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

/// 打印内存布局
fn print_mem_layout() {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack_top();
        fn boot_stack_lower_bound();
    }
    warn!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    warn!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    warn!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    warn!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    warn!(
        "boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
}
