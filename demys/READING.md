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
  - the `debug_line` section: will tell you which range of lines belogn to which mem address
  - the `debug_info` section: give information about types, functions, vars

- Set breakpoint funcrtion by name

## Set breakpoint by line number
- look into the `debug_line` section to find the range of this line belong to
- set breakpoint at base address of the range + offset line number

## Set breakpoint by function name
- Look into `debug_info` section and find subsection contains `DW_AT_name` that match function anme
- In the same section the `DW_AT_low_pc` will tell us the address of function => set braekpoint there

## How to read content of variables
- find in `debug_info` the subsection contains variable we need to set
- the variable address will be at  `DW_AT_frame_base + DW_AT_location`

