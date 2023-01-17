.altmacro
.macro SAVE_SN n 
    sd s\n, (\n+2)*8(a0)
.endm 
.macro LOAD_SN n  
    ld s\n, (\n+2)*8(a1)
.endm 

    .section .text
    .globl _switch
    .align 2
    # 当前sp需要是当前执行流的sp
_switch:
    # __switch(
    #     current_task_cx_ptr: *mut TaskContext, 
    #     next_task_cx_ptr: *const TaskContext
    # )
    # pub struct TaskContext(
    #     ra: usize,
    #     sp: usize,
    #     s: [usize;12],
    # )
    // 保佑当前执行流的 ra(返回地址),sp,callee save register
    sd ra, 0*8(a0)
    sd sp, 1*8(a0)
    .set n, 0
    .rept 12
        SAVE_SN %n
        .set n, n+1
    .endr
    .set n, 0

    .rept 12
        LOAD_SN %n
        .set n, n+1
    .endr
    ld ra, 0*8(a1)
    // 切换栈, 准确的说切换kernel_stack, 因为_switch函数只会在trap里,也就是kernel_stack里
    ld sp, 1*8(a1)
    ret
  



