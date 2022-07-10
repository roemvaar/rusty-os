// Rusty-OS - Tasks

// task_create
//
// Allocates and initializes a task descriptor, using the given priority, and the given function pointer
// as a pointer to the entry point of executable code
pub fn task_create() {
    println!("task_create()");
}

// task_get_tid
//
// Returns the task id of the calling task
pub fn task_get_tid() {
    println!("task_get_tid()");
}

// task_get_parent_tid
//
// Returns the task id of the task that created the calling task
pub fn task_get_parent_tid() {
    println!("task_get_parent_tid()");
}

// task_yield
//
// Causes a task to pause executing. The task is moved to the end of its priority queue, and will resume
// executing when next scheduled
pub fn task_yield() {
    println!("task_yield()");
}

// task_exit
//
// Causes a task to cease execution permanently. Resources owned by the task (its memory and task descriptor)
// are not reclaimed
pub fn task_exit() {
    println!("task_exit()");
}

// task_destroy
//
// Implies Exit(), but also releases the task's memory and task descriptor
pub fn task_destroy() {
    println!("task_destroy()");
}
