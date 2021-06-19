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
