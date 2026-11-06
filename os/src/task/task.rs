//! Types related to task management
pub use crate::syscall::SYSCALL_COUNT_LEN;
use super::TaskContext;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// 当前任务各系统调用的调用次数，以系统调用编号作为索引。
    pub sys_count: [usize;SYSCALL_COUNT_LEN],
}

/// The status of a task
#[derive(Copy, Clone, PartialEq)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
