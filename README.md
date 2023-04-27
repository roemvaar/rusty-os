[![crates.io](https://img.shields.io/crates/d/rusty-os.svg)](https://crates.io/crates/rusty-os)
[![crates.io](https://img.shields.io/crates/v/rusty-os.svg)](https://crates.io/crates/rusty-os)

# `rusty-os`

> Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.

This OS is based on the series of posts by Phillip Oppermann, go check his
[blog](https://os.phil-opp.com/) and [github](https://github.com/phil-opp/blog_os) for further explanation on how to build an OS.

## [Documentation](https://docs.rs/crate/rusty-os)

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  http://www.apache.org/licenses/LICENSE-2.0)
- MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Platform Support

Support for different platforms ("targets"):

| target | notes |
|---|---|
| aarch64  | QEMU ARM Cortex-M |
| x86_64 | QEMU x86 64 bits |


## Features

- Pre-emptive
- Multitask
- Interrupts
- Shared-memory

## Build

``
$ qemu-system-x86_64 -drive format=raw,file=target/x86_64/debug/bootimage-rusty-os.bin
``

``
$ cargo build
``

## Run

``
$ cargo run
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
