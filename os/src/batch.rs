use core::arch::asm;

use crate::{sync::up::UPSafeCell, trap::context::TrapContext};

const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

const MAX_APP_NUM: usize = 10;
const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;

static KERNEL_STACK: KernelStack = KernelStack {
    data: [0; KERNEL_STACK_SIZE],
};
static USER_STACK: UserStack = UserStack {
    data: [0; USER_STACK_SIZE],
};

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe {
        UPSafeCell::new({
            extern "C" {fn _num_app();}
            // 把一个内存地址, 当做 指向usize类型的指针
            let num_app_ptr = _num_app as usize as *const usize;
            let num_app = num_app_ptr.read_volatile();
            let mut app_start: [usize;MAX_APP_NUM+1] = [0; MAX_APP_NUM+1];
            let app_start_raw: &[usize] = core::slice::from_raw_parts(
                // num_app_ptr + 1 * size_of::<usize>
                num_app_ptr.add(1), num_app +1 );
            app_start[0..=num_app].copy_from_slice(app_start_raw);

            AppManager{
                num_app,
                current_app: 0,
                app_start,
            }
        })
    };
}

pub struct AppManager {
    num_app: usize,
    current_app: usize,
    // 加一是为了保存最后一个app的结束地址
    app_start: [usize; MAX_APP_NUM + 1],
}

impl AppManager {
    pub fn print_app_info(&self) {
        println!("[kernel] num_app={}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x},{:#x}",
                i,
                self.app_start[i],
                self.app_start[i + 1],
            )
        }
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    unsafe fn load_next_app(&mut self) {
        self.load_app(self.get_current_app());
        self.move_to_next_app();
    }

    unsafe fn load_app(&self, app_id: usize) {
        if app_id >= self.num_app {
            println!("[kernel] All applications completed! Exit qemu!");
            use crate::boards::qemu::QEMUExit;
            crate::boards::qemu::QEMU_EXIT_HANDLE.exit_success()
        }

        println!("[kernel] Loading app_{}", app_id);
        // clear icache
        asm!("fence.i");

        core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, APP_SIZE_LIMIT).fill(0);

        let app_src = core::slice::from_raw_parts(
            self.app_start[app_id] as *const u8,
            self.app_start[app_id + 1] - self.app_start[app_id],
        );

        let app_dst = core::slice::from_raw_parts_mut(APP_BASE_ADDRESS as *mut u8, app_src.len());

        app_dst.copy_from_slice(app_src);
    }
}

pub fn init() {
    print_app_info()
}

pub fn print_app_info() {
    APP_MANAGER.exclusive_access().print_app_info();
}

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}
impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + KERNEL_STACK_SIZE
    }

    fn push_context(&self, cx: TrapContext) -> &'static mut TrapContext {
        let cx_ptr = (self.get_sp() - core::mem::size_of::<TrapContext>()) as *mut TrapContext;
        unsafe {
            *cx_ptr = cx;
            cx_ptr.as_mut().unwrap()
        }
    }
}

pub fn run_next_app() -> ! {
    let mut app_manager = APP_MANAGER.exclusive_access();
    unsafe {
        app_manager.load_next_app();
    }
    drop(app_manager);

    extern "C" {
        fn _restore(cx_add: usize);
    }
    unsafe {
        // _restore 会释放kernel stack的 TrapContext空间,
        // 同时根据 TrapContext 指定的sp和 sepc(下一条指令位置),去设置sp和pc
        _restore(KERNEL_STACK.push_context(TrapContext::app_init_context(
            APP_BASE_ADDRESS,
            USER_STACK.get_sp(),
        )) as *const _ as usize);
    }

    panic!("Unreachable in batch::run_current_app!");
}
