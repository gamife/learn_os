.altmacro
.macro SAVE_GP n 
    sd x\n, \n*8(sp)
.endm 
.macro LOAD_GP n  
    ld x\n, \n*8(sp)
.endm 

    .section .text
    .globl _alltraps
    .globl _restore
    .align 2
_alltraps:
    # 进入trap前,sp=>(user sp) sscratch=>(kernel sp)
    # csrrw rd,csr,rs: 将csr的值读到rd寄存器, 同时将rs写到csr, 是原子操作
    csrrw sp, sscratch, sp
    addi sp,sp, -34*8
    sd x1, 1*8(sp)
    # 跳过sp(x2), 保存完其他寄存器后,最后再保存sp
    sd x3, 3*8(sp)
    # skip tp(x4), application does not use it
    # save x5~x31
    .set n, 5
    # 使用.rept伪指令, 需要在文件开头加 .altmacro
    .rept 27
        SAVE_GP %n
        .set n, n+1
    .endr

    # t0(x5) 这几个寄存器的值刚刚保存过了
    # csr 必须先读到通用寄存器里, 再写入内存
    csrr t0, sstatus
    csrr t1, sepc
    sd t0, 32*8(sp)
    sd t1, 33*8(sp)

    # 保存user sp到 kernel stack
    csrr t2, sscratch
    sd t2, 2*8(sp)

    # 把kernel sp 当做入参, trap_handler(cx: &mut TrapContext)-.&mut TrapContext
    mv a0, sp
    call trap_handler
_restore:
    # a0是 trap_handler的返回值, 把a0作为kernel sp

    mv sp, a0

    ld t0, 33*8(sp)
    ld t1, 32*8(sp)
    ld t2, 2*8(sp)
    csrw sepc, t0
    csrw sstatus, t1
    # 将sscratch恢复为 user sp
    csrw sscratch, t2

    # 从kernel stack 恢复寄存器
    ld x1, 1*8(sp)
    ld x3, 3*8(sp)
    .set n, 5
    .rept 27
        LOAD_GP %n
        .set n, n+1
    .endr

    # 释放 kernel stack
    addi sp, sp, 34*8
    # 执行完这条指令后, sp=>(user sp) sscratch=>(kernel sp)
    csrrw sp, sscratch, sp
    sret



