
    .align 3
    .section .data
    .global _num_app
_num_app:
    #quad 把参数当做8字节整数
    .quad 3
    .quad app_0_start
    .quad app_1_start
    .quad app_2_start
    .quad app_2_end

        .section .data
        .global app_0_start
        .global app_0_end
    app_0_start:
        # 把对应文件的内容复制到这个位置
        .incbin "../user/target/riscv64gc-unknown-none-elf/release/00write_a.bin"
    app_0_end:

        .section .data
        .global app_1_start
        .global app_1_end
    app_1_start:
        # 把对应文件的内容复制到这个位置
        .incbin "../user/target/riscv64gc-unknown-none-elf/release/01write_b.bin"
    app_1_end:

        .section .data
        .global app_2_start
        .global app_2_end
    app_2_start:
        # 把对应文件的内容复制到这个位置
        .incbin "../user/target/riscv64gc-unknown-none-elf/release/02write_c.bin"
    app_2_end:
