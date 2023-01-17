use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};

pub fn sys_exit(exit_code: isize) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit.");
}

pub fn sys_yield() -> isize{
    println!("[kernel] Application yield");
    suspend_current_and_run_next();
    0
}