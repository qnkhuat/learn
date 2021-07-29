# Chapter 1 - Freestanding RUST
## Summary
Setting up a RUST dev env where we don't have any OS dependent in our compiled binary

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
## Summary
Set up an OS kernel where we can boot, print on either simulation(QEMU) or real machine(x86_64)

## Booting

Boot procedure:
- Turn on
- CPU load a default isntruction to load a Power On Self Test firmware
- The firmware then looks for a bootloader: BIOS for window and EFI for MacOSX
- the bootloader then try to load kernel to memory

We are not going to write a bootloader in this tutorial bc it involves non-insightful steps like "write this magic value to a register". Instead we use bootimage to automatically prepends a bootloader to our kernel.

Behidndthe scenes the bootimage do:
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

## Summary
Learn about how do we programmatically print characters to a display

## How to pritn
The way we write string is we write to a memory mapped I/O address.

In our case the base address of display is 0xb80000 wit a default dimension: 25x80 => that means starting from 0xb8000 to 0xb8000 + 25*80

This address is not mapped to RAM but directly some memroy on a VGA device

By writting to this address we will control what to display

VGA text buffer layout

Bit(s)	|  Value
--------|------------------
0-7     |  ASCII code point
8-11	  |  Foreground color
12-14	  |  Background color
15	    |  Blink

## Note about port-mapped I/O
This is especially true on x86_64: that peripherals often has 2 communication channel: memory mapped I/O and port-mapped I/O

In constast with memory-mapped I/O, port-mapped I/O use a seperate I/O bus for communication. Each peripherals has one or more port numbers.

We'll use the port-mapped I/O to to exit the QEMU by writing a value to its port

# Chapter 4 - Testing
After running test inside our OS, we need a way to communicate the results back to our machine.

One way is to create a TCP network interface, but this is complicated. Instead we are going to use serial port - an old interface standard.

There are lots of UART models on X86 but fortunately most of them are compaptible with 16550 UART, so we will that model for our testing framework


# Chapter 5 - CPU Exceptions
## Summary
What happens and how do we handle exception at OS level

## How it works
While runnning if a program yield an exception signal (E.g: Try to divide by 0) the CPU will interrupt and calls a specific excepiton handler funciton.

On x86 there are many exception types: Page fault, invalid opcode, general production fault, double fault, ....

In order to catch and handle exceptions, we have to set up a Interrupt Descriptor Table (IDT)

This table is a map of exception code with its handler

Entry of IDT table layout: 

Type	|  Name	                    |  Description
------|---------------------------|-----------------------------------------------------------
u16	  |  Function Pointer [0:15]	|  The lower bits of the pointer to the handler function.
u16	  |  GDT selector	            |  Selector of a code segment in the global descriptor table.
u16	  |  Options	                |  (see below)
u16	  |  Function Pointer [16:31]	|  The middle bits of the pointer to the handler function.
u32	  |  Function Pointer [32:63]	|  The remaining bits of the pointer to the handler function.
u32	  |  Reserved

We depend a lot on the x86-iterrupt packages to setup the exception handlers for us

## Calling convention
It specify the details of a function call. For example, they specify where function parameters are placed (e.g. in registers or on the stack) and how results are returned.

In rust we define this with the keyword `extern "C" fn` meaning this funciton will be called with C calling convention.

With our exception we need an call convention that when an exception occur, it guarantees to preserve all the internal parameters of function ( C callin convention doesn't do this ). This is exactly what "x86-interrupt" do.

## Further reading
[Handling Exceptions using naked Functions](https://os.phil-opp.com/edition-1/extra/naked-exceptions/)

# Resources
[Operators and Symbols](https://doc.rust-lang.org/book/appendix-02-operators.html)
