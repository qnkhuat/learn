Dependencies:
- https://github.com/antirez/linenoise => handling command line input
- https://github.com/TartanLlama/libelfin/tree/fbreg => parsing debug info

# Principle
- use fork to exec(execl) the program in the child process and debugger in parent process
- extensively use ptrace observe and control execution of another process by reading registers
- Set TRACEME on the child process => mark the child process as being traced. and the everytime the child process stop, a signal will be sent to parent process.
- Use wait on parent process and listen to singal from the trace

# Breakpoints
- There are 2 types of breakpoints: hardwand and software

## Software breakpoints
- On x86 we overwite the instruction address with `int 3` instruction to halt the program
- When the processor execute an `int 3` insruction => control is passed to the breakpoint interrupt handler, which mean signal process with a SIGTRAP


# What I learned
- When a program run the OS will allocate a virutal memory space for it, we can check the segment mapping at /proc/[pid]/maps
- We can find address of instruction of a program by objdump the binary, find the address of instruction we want to set. add it to the base address of program in the segment mapping
- ELF(Executable and Linkable Format) : A format specifies how to store code, static data, debug info, strings in a binary.
- DWARF : the debug information format commonly used with ELF. (`dwarfdump` is a tool like objdump but for dwarf)
