/* Rusty-OS
 * 
 * Rusty-OS is a simple RTOS for ARM-based embedded systems 
 * written in Rust.
 */

#![no_std]
#![no_main]


use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Rusty OS!");
    println!("Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.");
    
    let mut ticks = 0;
    
    // Ifinite loop
    loop {
        // Write your tasks here   
        println!("Awake time: {} ticks", ticks);
        ticks = ticks + 1;
    }

    loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
