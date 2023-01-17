use super::context::TaskContext;


#[derive(Copy,Clone)]
pub struct TaskControlBlock{
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
}


#[derive(Debug,Copy,Clone,PartialEq, Eq)]
pub enum TaskStatus {
    Uninit,
    Ready,
    Running,
    Exited,
}