use std::process::{exit};
use std::io::{Write};
use libc;
mod unistd;
mod ptrace;
mod target;

//fn str2u64(s: &str) -> u64 {
//  u64::from_str_radix(s.trim(), 16).unwrap()
//}

//fn str2u8(s: &str) -> u8 {
//  u8::from_str_radix(s.trim(), 16).unwrap()
//}

fn str2usize(s: &str) -> usize {
  usize::from_str_radix(s.trim(), 16).unwrap()
}

fn debug_loop(prog: &mut target::TargetProgram) {
  let mut last_input: String = String::new();
  println!("Start debugging process: {}", prog.pid());
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
      "help" | "h" => {
        println!("h | help:");
        println!("\t Display help");
        println!("");

        println!("c | cont:");
        println!("\t Continue execution");
        println!("");


        println!("b | break <addr>:");
        println!("\t Set break point at address");
        println!("");


        println!("mem get <addr> : mem get 000003b1 ");
        println!("\t Get memory value at address");
        println!("");

        println!("mem set <addr> <value>: mem set 000003b1 34");
        println!("\t Set memory value at address");
        println!("");

        println!("reg:");
        println!("\t Display value of all registers");
        println!("");

        println!("reg get <reg> : reg rip");
        println!("\t Get Register value at address");
        println!("");

        println!("reg set <reg> <value>: reg rip 34");
        println!("\t Set Register value at address");
        println!("");

        println!("q | quit:");
        println!("\t Exit Demys");
        println!("");


      }
      "cont" | "c" => {
        prog.cont();
      }
      "lsb" => {
        let breakpoints = prog.breakpoints();
        for i in 0..breakpoints.len() {
          println!("br {}: 0x{:016x}, data: 0x{:016x}", i, breakpoints[i].addr, breakpoints[i].orig_byte);
        }
      }
      "b" | "break" => {
        if input_split.len() < 2 {
          println!("Usage: b [addr]");
        } else {
          let addr = str2usize(&input_split[1]);
          prog.set_breakpoint(addr);
          println!("Set breakpoint addr: 0x{:016x}", addr);
        }
      }
      "mem" => {
        if input_split.len() > 2 && input_split[1] == "get" {
          let addr = str2usize(&input_split[2]);
          let value = prog.peek_data(addr);
          println!("(0x{:016x}): 0x{:016x}", addr, value);
        } else if input_split.len() > 3 && input_split[1] == "set" {
          let addr = str2usize(&input_split[2]);
          let value = str2usize(&input_split[3]);
          prog.poke_data(addr, value);
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
          println!("IP: Instruction pointer");
          println!("DI: Destination index");
          println!("CS: Code segment");
          println!("SS: Stack Segment");
          println!("DS: Data Segment");
          println!("(E|F|G)S: Extra Segment");

        } else if input_split.len() > 2 && input_split[1] == "get" {

          let reg_name = input_split[2].trim();
          let reg_value: usize= prog.get_reg_value(&reg_name);
          println!("Reg {}: 0x{:016x}", reg_name, reg_value)

        } else if input_split.len() > 3 && input_split[1] == "set" {

          let reg_name = input_split[2].trim();

          let set_value = str2usize(&input_split[3]);

          prog.set_reg_value(&reg_name, &set_value);

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
      "pid" => {
        println!("Pid: {}", prog.pid);
      }

      "q" | "quit" => {
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
