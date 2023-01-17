
// 1. 一次普通的函数调用, 需要保存ra和 callee save regiter
// 但是任务的切换, 会更改栈, 所以还要保存当前的sp
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TaskContext{
    // 函数返回的pc地址
    ra: usize,
    // 函数调用时的sp
    sp: usize,
    // 被调用者保存的寄存器, 函数返回后需要恢复这些寄存器
    s: [usize;12],
}

impl TaskContext{
    pub fn zero_init() -> Self{
        Self { ra: 0, sp: 0, s: [0;12] }
    }
    pub fn go_restore(kernel_sp: usize) -> Self{
        extern "C"{
            // 需要sp执行kernel sp
            pub fn _restore();
        }
        
        Self { ra: _restore as usize , sp: kernel_sp, s: [0;12] }
    }
}