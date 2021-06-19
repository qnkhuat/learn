use std::process::{exit};
use std::io::{self, Write};
use libc;
mod unistd;
mod ptrace;
mod target;

fn debug_loop(prog: &mut target::TargetProgram) {
let mut last_input: String = String::new();
    loop {
      let mut input = String::new();
      print!("(demys) > ");
      std::io::stdout().flush().expect("DBG: failed to flush stdout");
      std::io::stdin().read_line(&mut input).expect("DBG: Couldn't read from stdin");

      if input.trim().len() == 0 {
        input = last_input.clone();
      } else {
        last_input = String::from(input.trim().clone());
      }
      let input_split = input.trim().split(' ').collect::<Vec<&str>>();

      match input_split[0] {
        "c" => {
          prog.cont();
        }
        "b" => {
          if input_split.len() < 2 {
            println!("Usage: b [addr]");
          } else {
            let addr = usize::from_str_radix(&input_split[1], 16).unwrap();
            prog.set_breakpoint(addr);
            println!("Set breakpoint at addr: {}", addr);
          }
        }
        "rh" => {
          println!("AX: Accumulator");
          println!("BX: Base");
          println!("CX: Counter");
          println!("DX: Data");
          println!("R1-15: Genral");
          println!("SP: Stack pointer");
          println!("BP: Base pointer");
          println!("SI: Source index");
          println!("DI: Destination index");
          println!("CS: Code segment");
          println!("SS: Stack Segment");
          println!("DS: Data Segment");
          println!("(E|F|G)S: Extra Segment");
        }
        "r" => {
          let regs = prog.get_user_struct().regs;
          println!("RAX: 0x{:016x} RBX: 0x{:016x} RCX: 0x{:016x} RDX: 0x{:016x}",
                   regs.rax, regs.rbx, regs.rcx, regs.rdx);
          println!("R15: 0x{:016x} R14: 0x{:016x} R13: 0x{:016x} R12: 0x{:016x}",
                   regs.r15, regs.r14, regs.r13, regs.r12);
          println!("R11: 0x{:016x} R10: 0x{:016x} R9:  0x{:016x} R8:  0x{:016x}",
                   regs.r11, regs.r10, regs.r9, regs.r8);
          println!("RSP: 0x{:016x} RBP: 0x{:016x} RSI: 0x{:016x} RDI: 0x{:016x}",
                   regs.rsp, regs.rbp, regs.rsi, regs.rdi);
          println!("RIP: 0x{:016x} CS: 0x{:04x} EFLAGS: 0x{:08x}",
                   regs.rip, regs.cs, regs.eflags);
          println!("SS: 0x{:04x} DS: 0x{:04x} ES: 0x{:04x} FS: 0x{:04x} GS: 0x{:04x}",
                   regs.ss, regs.ds, regs.es, regs.fs, regs.gs);
        }
        "q" => {
          exit(0);
        }
        _ => println!("Unknown command: {}", input),
      }
    }
}

fn start(executable: &String, argv:&[String]) {
  let pid = unistd::fork().unwrap();
  let mut prog = target::TargetProgram::new(pid, executable);

  if pid == 0 { // child
    unsafe {
      libc::personality(libc::ADDR_NO_RANDOMIZE as u64);
    }
    prog.run(argv);
  } else if pid > 0 { // Parent
    prog.wait();
    debug_loop(&mut prog);
  } else {
    panic!("fork() failed");
  }
}

fn main() {
  let args: Vec<String> = std::env::args().collect();
  if args.len() < 2 {
    println!("Usage: demys [executable]");
    exit(1);
  }

  let target: &String = &args[1];
  let argv = &args[1..];
  start(target, argv);
}
