## 记录
1. task的_switch, 是在kernel stack切换的, 将kernel_sp1切换到 kernel_sp2, 对于一条执行流来说, 像是卡在了内核的_switch函数里, 实际上是在_switch函数里, 保存当前执行流的task_context,   
恢复另一条执行流的task_context(切换了kernel_sp), 处理器处理另一条执行流去了.
2. task_context保存在全局变量里
3. 有n个app, 就有n个kernel stack 和 n个user stack
4. 初始化的时候, 每个kernel stack 都构造了 trap_context, 
```sh
trap_context{
    sp -> u_sp,
    sstatus -> SPP::USER,
    sepc -> app_start_addr,
}
```
并且在全局变量里, 每个task_context
```sh
task_context1{
    sp -> k_sp_1
    a0 -> _restore()
}
```

## 第一次运行app
1. S -> U
2. sscratch -> ksp
3. sp -> usp
4.1 pc -> a0 ret
4.2 pc -> sepc sret