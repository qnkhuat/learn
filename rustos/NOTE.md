# Chapter 1 - Freestanding RUST

## Stack Unwinding
Is a technique to clean memory when a panic ocurrs. If we don't implement this then when a program get errors, it won't clean the data it initiated.

In this implementation we will disable it bc we are building our own OS, there are no OS API to call to clean it up.

## Lang Items
Lang items are a way for the stdlib (and libcore) to define types, traits, functions, and other items which the compiler needs to know about.

## Bare Metal target
Bare metal meaning we don't run our program on top of any OS, instead we run it directly with the CPU

That means we need to compile our program to a host system that doesn't are bare metal.
In this post we use thumbv7em-none-eabihf ( an embbeded ARM ) as the compiled host

# Chapter 2 - Minimal Rust kernel
Boot procedure:
- Turn on
- CPU load a default isntruction to load a Power On Self Test firmware
- The firmware then looks for a bootloader: BIOS for window and EFI for MacOSX
- the bootloader then try to load kernel to memory

We are not going to write a bootloader in this tutorial bc it involves non-insightful steps like "write this magic value to a register". Instead we use bootimage to automatically prepends a bootloader to our kernel.

Behidn the scenes the bootimage do:
- compiles our kernel to an ELF file.
- compiles the bootloader dependency as a standalone executable.
- links the bytes of the kernel ELF file to the bootloader.


## Printing to Screen
The easist way to print text to the screen is using [VGA text buffer](https://en.wikipedia.org/wiki/VGA-compatible_text_mode). It's a special memory area mapped to the VGA hardware that contains the contents displayed on screen. Default is a 80x25 grid

With VGA text buffer we can:
- Set blink
- Set background, foreground colors
- Use Ascii 
- Load font


# Chapter 3 - VGA text mode
VGA text buffer layout

Bit(s)	|  Value
0-7     |  ASCII code point
8-11	  |  Foreground color
12-14	  |  Background color
15	    |  Blink


Default dimension: 25x80

Memory mapped Address: 0xb8000

By writting to this address we will control what to display
 

# Resources
[Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)


# Env note
rust nightly
target triple: thumbv7em-none-eabihf

