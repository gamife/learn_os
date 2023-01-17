use crate::{sync::up::UPSafeCell, config::MAX_APP_NUM, loader::{get_num_app, init_app_trap_tx}, task::{task::TaskStatus, context::TaskContext}, boards::qemu::{QEMU_EXIT_HANDLE, QEMUExit}};

use self::{task::TaskControlBlock, switch::_switch};

pub mod context;
pub mod switch;
pub mod task;

pub struct TaskManager{
    num_app: usize,
    inner: UPSafeCell<TaskManagerInner>,
}

pub struct TaskManagerInner{
    tasks: [TaskControlBlock;MAX_APP_NUM],
    current_task: usize,
}

lazy_static!{
    pub static ref TASK_MANAGER: TaskManager = {
        let num_app = get_num_app();
        let mut tasks = [TaskControlBlock{
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::Uninit,
        }; MAX_APP_NUM];

        for (i,task) in tasks.iter_mut().enumerate(){
            task.task_cx = TaskContext::go_restore(init_app_trap_tx(i));
            task.task_status = TaskStatus::Ready;
        }

        TaskManager{
            num_app,
            inner: unsafe{
                UPSafeCell::new(TaskManagerInner { tasks, current_task: 0 })
            }
        }
    };
}

impl TaskManager{
    pub fn run_first_app(&self){
        let mut inner = self.inner.exclusive_access();
        inner.current_task = 0;
        let task0 = &mut inner.tasks[0];
        task0.task_status = TaskStatus::Running;
        let next_task_ctx_ptr = &task0.task_cx as *const TaskContext;
        drop(inner);

        let _unused = &mut TaskContext::zero_init() as *mut TaskContext;
        unsafe{
            _switch(_unused, next_task_ctx_ptr);
        }
        panic!("unreachable in run_first_task");
    }
    pub fn find_next_ready_task(&self) -> Option<usize>{
        let tasks = self.inner.exclusive_access();
        // 0..tasks.current_task+1 包括current_task, 因为如果只有一个task了,执行yield, 还是运行current_task
        for i in (tasks.current_task+1 .. self.num_app).into_iter().chain( 0..tasks.current_task+1){
            if tasks.tasks[i].task_status == TaskStatus::Ready{
                return Some(i);
            }
        }
        None
    }

    pub fn run_next_app(&self){
        if let Some(next) = self.find_next_ready_task(){
            let mut inner = self.inner.exclusive_access();
            let current = inner.current_task;
            let current_task_ctx_ptr = &mut inner.tasks[current].task_cx as *mut TaskContext;
            let next_task_ctx_ptr = &inner.tasks[next].task_cx as *const TaskContext;
            inner.current_task = next;
            drop(inner);

            unsafe{
                switch::_switch(current_task_ctx_ptr, next_task_ctx_ptr);
            }
        }else{
            println!("[kernel] All application completed. Exit qeum.");
            QEMU_EXIT_HANDLE.exit_success();
        }
    }

    pub fn mark_current_suspended(&self){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Ready;
    }
    pub fn mark_current_exited(&self){
        let mut inner = self.inner.exclusive_access();
        let current = inner.current_task;
        inner.tasks[current].task_status = TaskStatus::Exited;
    }
}

pub fn run_first_task(){
    TASK_MANAGER.run_first_app();
}
pub fn exit_current_and_run_next(){
    mark_current_exited();
    run_next_app();
}
pub fn suspend_current_and_run_next(){
    mark_current_suspended();
    run_next_app();
}

fn run_next_app(){
    TASK_MANAGER.run_next_app();
}

fn mark_current_exited(){
    TASK_MANAGER.mark_current_exited()
}
fn mark_current_suspended(){
    TASK_MANAGER.mark_current_suspended()
}





