/* Rusty-OS
 *
 * Rusty-OS is a simple RTOS for ARM-based embedded systems
 * written in Rust.
 */

#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rusty_os::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Rusty OS!");
    println!("Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.");

    rusty_os::init();

    x86_64::instructions::interrupts::int3();

    #[cfg(test)]
    test_main();

    let mut ticks = 0;

    println!("It did not crash!");

    loop {}

    // loop {
    //     // Write your tasks here
    //     println!("Awake time: {} ticks", ticks);
    //     ticks = ticks + 1;
    // }
}

// This function is called on panic
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
