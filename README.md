[![crates.io](https://img.shields.io/crates/d/rusty-os.svg)](https://crates.io/crates/rusty-os)
[![crates.io](https://img.shields.io/crates/v/rusty-os.svg)](https://crates.io/crates/rusty-os)

# `rusty-os`

> Rusty-OS is a simple RTOS for ARM-based embedded systems written in Rust.

This OS is based on the series of posts by Phillip Oppermann, go check his
[blog](https://os.phil-opp.com/) and [github](https://github.com/phil-opp/blog_os) for further explanation on how to build an OS usign Rust.

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
| x86_64 | QEMU x86 64 bits |

Future work includes adding support for different platforms. ARM support is
in progress.

## Build

This project requires a nightly version of Rust because it uses some unstable
features. At least nightly _2020-07-15_ is required for building. You might
need to run `rustup update nightly --force` to update to the latest nightly
even if some components such as `rustfmt` are missing it.

You can build the project by running:

```
$ cargo build
```

To create a bootable disk image from the compiled kernel, you need to install
the [`bootimage`] tool:

```
$ cargo install bootimage
```

After installing, you can create the bootable disk image by running:

```
$ cargo bootimage
```

This creates a bootable disk image in the `target/x86_64/debug` directory.

## Run

```
$ cargo run
```

[QEMU] and the [`bootimage`] tool need to be installed for this.

## Contribute

Contributions are accepted. Make sure to open a merge request.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.


## Notes

This is a hobby project in order to learn the inner components of RTOS design and
low-level development for ARM processors using Rust. Rusty-OS being a hobby project
can't be compared to mainstream RTOSs like The Zephyr Project, FreeRTOS, QNX, etc. And again, special thanks to Phillip Oppermann for paving the way for the
other embedded software enthusiasts with his work. "Standing on the shoulders of giants".
