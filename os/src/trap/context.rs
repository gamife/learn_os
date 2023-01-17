use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext {
    pub x: [usize; 32],
    // trap的原本特权级, 需要在trap结束后恢复
    pub sstatus: Sstatus,
    // 进入trap前的最后一条指令
    // trap结束后, 需要执行spec+len_of(trap instruct)位置的指令
    pub spec: usize,
}

impl TrapContext {
    pub fn set_sp(&mut self, sp: usize) {
        self.x[2] = sp;
    }
    // 构造一个TrapContext, 其trap结束后, 执行entry位置的指令,sp指向入参sp
    pub fn app_init_context(entry: usize, user_sp: usize) -> Self {
        let mut sstatus = sstatus::read();
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            spec: entry,
        };
        //
        cx.set_sp(user_sp);
        cx
    }
}
