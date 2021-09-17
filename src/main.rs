/* Rusty-OS
 * 
 * Rusty-OS is a simple RTOS for ARM-based embedded systems 
 * written in Rust.
 */

use std::{thread, time::Duration};


fn main() {
    println!("Rusty OS!");
    println!("Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.");
    
    let ticks = 0;
    
    // Ifinite loop
    loop {
        // Write your tasks here   
        println!("Awake time: {} ticks", ticks);
        ticks = ticks + 1;
    }
}
 
