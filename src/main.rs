/* Rusty-OS
 * 
 * Rusty-OS is a simple RTOS for ARM-based embedded systems 
 * written in Rust.
 */

#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World! This is Rusty-OS :)";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // println!("Rusty OS!");
    // println!("Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.");
    
    // let mut ticks = 0;
    
    // // Ifinite loop
    // loop {
    //     // Write your tasks here   
    //     println!("Awake time: {} ticks", ticks);
    //     ticks = ticks + 1;
    // }
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
