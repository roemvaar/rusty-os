// Rusty-OS
//
// Rusty-OS is a simple RTOS for ARM-based embedded systems 
// written in Rust

use std::{thread, time};
mod tasks;


fn main() {
    println!("Rusty OS!");
    println!("Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.");
    
    let mut ticks = 0;

    // Create tasks here
    tasks::task_create();
    tasks::task_get_tid();
    tasks::task_get_parent_tid();
    tasks::task_yield();
    tasks::task_exit();
    tasks::task_destroy();
    
    // Ifinite loop
    loop {
        println!("Awake time: {} ticks", ticks);
        ticks = ticks + 1;
        thread::sleep(time::Duration::from_secs(5));
    }
}
