# rusty-os

Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.

Supported systems:

| Arch |  Supported |
|---|---|
| ARM Cortex-M  | Yes |


## Features

- Pre-emptive
- Multitask
- Interrupts
- Shared-memory

## Build

``
$ cargo build
``

## Contribute

Contributions are accepted. Make sure to open a merge request.

## Future Development

Future development includes adding new features and support for other
systems such as RISC-V.

## Notes

This is a hobby project in order to learn the inner components of RTOS design and
low-level development for ARM processors using Rust. Rusty-OS being a hobby project
can't be compared to mainstream RTOSs like The Zephyr Project, FreeRTOS, QNX, etc.
