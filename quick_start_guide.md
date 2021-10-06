# Quick start instructions

## 1. Download rusty-os source code

```git clone https://github.com/roemvaar/rusty-os.git```

## 2. Documentation

[Documentation](https://github.com/roemvaar/rusty-os/tree/main/docs)

## 3. Building the project

```rust
  cargo build --verbose
```

## 4. Running the demo application

The easiest way to run the demo application is by using QEMU. Another way to
do run rusty-os based application is by setting up the target hardware. Here
are the instructions for the (SELECTED ARM DEV KIT):

## 5. Create your own project

The easiest way to create your own rusty-os project is to base it on the demo
the demo application. Once you have the demo application running correctly,
incrementally remove the demo functions and source files and replace them
with your own application code.


Inspo: https://www.freertos.org/FreeRTOS-quick-start-guide.html
