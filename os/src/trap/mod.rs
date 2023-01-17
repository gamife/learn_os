use core::arch::global_asm;

use riscv::register::{
    scause::{self, Exception, Trap},
    stval, stvec,
    utvec::TrapMode,
};

use crate:: syscall::syscall;

use self::context::TrapContext;

pub mod context;

global_asm!(include_str!("trap.S"));

extern "C"{
    // 需要sp执行kernel sp
    pub fn _restore();
}


pub fn init() {
    extern "C" {
        fn _alltraps();
    }
    unsafe {
        // stvec 是保存trap handler的地址的寄存器
        // 所以_alltraps就是trap的入口
        stvec::write(_alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause_r = scause::read();
    let stval_r = stval::read();
    match scause_r.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            // 这个trap是由"ecall"(码长为4)指令调用的, 所以trap结束后,要回到ecall的下一条指令的话, 要加4
            cx.spec += 4;
            // 这里和user app的syscall方法对应
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            panic!("[kernel] Cannot coutine trap");
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            panic!("[kernel] Cannot coutine trap");
        }
        _ => {
            panic!(
                "Unsupported trap {:?}, stval = {:#x}",
                scause_r.cause(),
                stval_r
            );
        }
    }
    cx
}
