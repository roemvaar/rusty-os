# rusty-os

Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.

It currently supports two scheduling algorithms and one hardware devices.

The motivation to build this project is to learn/achieve the following:

- Understand the architecture of the ARM cortex M series of processors

- Develop a build system from sratch using open source tools

- Write low-level code to initialize the microcontroller: this includes linker scripts,
interrupt vector tables, etc.

- Understand the working of an operating system and implement a basic scheduler

- Implement primitives like mutexes and spinlocks from scratch

- Use CMSIS to make sure our code is reusable

- Make the project portable by separating device-specific code from kernel code

- Use CI pipeline in GitHub using Travis CI

### Characteristics

- Pre-emptive
- Multitask
- Interrupts

### Build

### Examples

### Supported Devices

It currently supports development boards from

- STMicroelectronics
- Texas Instruments
- NXP Semiconductors

### Contribute

This project is in its initial stages and we are planning to add more features to it.
We have made this open source so everyone can contribute and learn.

Look at contribute.md

### Notes

This is a hobby project in order to learn the inner components of OS design and
low-level development for ARM processors using Rust. Rusty-OS being a hobby project
can't be compared to mainstream RTOSs like The Zephyr Project, FreeRTPS, QNX, etc.
