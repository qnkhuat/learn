Dependencies:
- https://github.com/antirez/linenoise => handling command line input
- https://github.com/TartanLlama/libelfin/tree/fbreg => parsing debug info

DWARF : understand what is it . seems like it is a protocol to communicate between program

- use fork to exec(execl) the program in the child process and debugger in parent process
- extensively use ptrace observe and control execution of another process by reading registers
class debugger:
  run()
  handle_command()

- Process run => process will receive a SIGTRAP singal
- Command: 
  - c : continue
  - b [addr] : break

