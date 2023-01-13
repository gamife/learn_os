    .section .text.entry
    .globl _start

_start:
    # 设置os刚启动时的 sp
    # 跟kernel stack 不是一个
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack_lower_bound
boot_stack_lower_bound:
    .space 4096 * 16
    .globl boot_stack_top
boot_stack_top: