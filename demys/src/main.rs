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
        "cont" => {
          prog.cont();
        }
        "br" => {
          if input_split.len() < 2 {
            println!("Usage: b [addr]");
          } else {
            let addr = usize::from_str_radix(&input_split[1], 16).unwrap();
            prog.set_breakpoint(addr);
            println!("Set breakpoint at addr: {}", addr);
          }
        }
        "reg" => {
          if input_split.len() > 1 && input_split[1] == "help" {
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

          } else if input_split.len() > 2 && input_split[1] == "get" {

            let regs = prog.get_user_struct().regs;
            let reg_name = input_split[2].trim();
            let reg_value = target::get_reg_by_name(&regs, &reg_name).unwrap();
            println!("Reg {}: {:016x}", reg_name, reg_value)

          } else if input_split.len() > 3 && input_split[1] == "set" {
            
            let mut user_struct = prog.get_user_struct();
            let reg_name = input_split[2].trim();
            let set_value = u64::from_str_radix(&input_split[3], 16).unwrap();

            target::set_reg_by_name(&mut user_struct.regs, &reg_name, &set_value);
            prog.set_user_struct(&user_struct);

          } else {
            let regs = prog.get_user_struct().regs;

            println!("rax: 0x{:016x} rbx: 0x{:016x} rcx: 0x{:016x} rdx: 0x{:016x}",
                     regs.rax, regs.rbx, regs.rcx, regs.rdx);
            println!("r15: 0x{:016x} r14: 0x{:016x} r13: 0x{:016x} r12: 0x{:016x}",
                     regs.r15, regs.r14, regs.r13, regs.r12);
            println!("r11: 0x{:016x} r10: 0x{:016x} r9:  0x{:016x} r8:  0x{:016x}",
                     regs.r11, regs.r10, regs.r9, regs.r8);
            println!("rsp: 0x{:016x} rbp: 0x{:016x} rsi: 0x{:016x} rdi: 0x{:016x}",
                     regs.rsp, regs.rbp, regs.rsi, regs.rdi);
            println!("rip: 0x{:016x} cs: 0x{:04x} eflags: 0x{:08x}",
                     regs.rip, regs.cs, regs.eflags);
            println!("ss: 0x{:04x} ds: 0x{:04x} es: 0x{:04x} fs: 0x{:04x} gs: 0x{:04x}",
                     regs.ss, regs.ds, regs.es, regs.fs, regs.gs);
          }
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
