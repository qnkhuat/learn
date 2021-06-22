use libc::pid_t;
use crate::unistd;
use crate::ptrace;
use std::io::{Write};
use std::process::{exit};

fn str2usize(s: &str) -> usize {
  usize::from_str_radix(s.trim(), 16).unwrap()
}


// *** Breakpoint ***
#[derive(Clone)]
pub struct Breakpoint {
  pid: pid_t,
  pub addr: usize,
  pub orig_byte: usize, // the tutorial use u8. Why?
  pub enabled: bool,
}

impl Breakpoint {
  pub fn enable(&mut self) {
    let data = ptrace::peek_data(self.pid, self.addr).unwrap();
    self.orig_byte = data;
    /* 0xCC is the machine code for int $3 on x86_64 - the interupt instruction */
    ptrace::poke_data(self.pid, self.addr, 0xcc);
    self.enabled = true;
  }

  pub fn disable(&mut self) {
    ptrace::poke_data(self.pid, self.addr, self.orig_byte);
    self.enabled = false;
  }

}

// *** Target Program ***
pub struct TargetProgram {
  pub pid: pid_t,
  executable: String,
  breakpoints: Vec<Breakpoint>,
}

impl TargetProgram {
  pub fn new(pid: pid_t, path: &String) -> TargetProgram {
    TargetProgram{
      pid: pid,
      executable: (*path).clone(),
      breakpoints: Vec::new()
    }
  }

  pub fn initialize_base_address(&mut self) {
    let maps_filename = std::fmt::format(format_args!("/proc/{}/maps", self.pid));
    let maps = std::fs::read_to_string(maps_filename).expect("Failed to read mmaps file");
    //println!(maps);
  }

  pub fn breakpoints(&mut self) -> Vec<Breakpoint>{
    return self.breakpoints.clone();
  }

  pub fn singlestep(&mut self) {
    ptrace::singlestep(self.pid).unwrap();
  }

  pub fn kill(&mut self) {
    unistd::kill(self.pid);
  }

  pub fn cont(&mut self) {
    self.step_over_breakpoint();
    ptrace::cont(self.pid).unwrap();
    self.wait();
  }
  
  pub fn peek_data(&mut self, addr: usize) -> usize {
    let data: usize = ptrace::peek_data(self.pid, addr).unwrap();
    return data;
  }

  pub fn poke_data(&mut self, addr: usize, data: usize) {
    ptrace::poke_data(self.pid, addr, data);
  }

  pub fn wait(&mut self) -> i32{
    return unistd::wait().unwrap() as i32;
  }
  
  pub fn pid(&mut self) -> &pid_t {&self.pid}

  pub fn get_pc(&mut self) -> usize {
    return self.get_reg_value("rip");
  }

  pub fn set_pc(&mut self, value: &usize){
    self.set_reg_value("rip", value);
  }

  pub fn set_breakpoint(&mut self, addr: usize) {

    let mut bp = Breakpoint{
      pid: self.pid,
      addr: addr, 
      orig_byte: 0,
      enabled: false
    };
    bp.enable();
    
    self.breakpoints.push(bp);
  }

  pub fn step_over_breakpoint(&mut self) {
    // - 1 because execution will go pass the breakpoint
    let prev_pc_location = self.get_pc() - 1; 
    println!("Current_pc_location: 0x{:016x},  prev_pc_location: 0x{:016x}, len: {}", self.get_pc(), prev_pc_location, self.breakpoints.len());

    for i in 0..self.breakpoints.len() {
      if self.breakpoints[i].addr == prev_pc_location && self.breakpoints[i].enabled {
        self.set_pc(&prev_pc_location);
        self.breakpoints[i].disable();
        self.singlestep();
        self.wait();
        self.breakpoints[i].enable();
        return;
      }
    }
  }

  pub fn get_user_struct(&mut self) -> libc::user {
    unsafe {
      let mut user_struct: libc::user = std::mem::uninitialized();
      ptrace::get_user_struct(self.pid, &mut user_struct);
      return user_struct;
    }
  }

  pub fn set_user_struct(&mut self, user_struct: &libc::user) {
    ptrace::set_user_struct(self.pid, user_struct);
  }

  pub fn get_reg_value(&mut self, reg_name: &str) -> usize{
    let regs = self.get_user_struct().regs;
    return match reg_name.trim().to_ascii_lowercase().as_str() {
      "rax" => regs.rax,
      "rbx" => regs.rbx,
      "rcx" => regs.rcx,
      "rdx" => regs.rdx,
      "r8"  => regs.r8,
      "r9"  => regs.r9,
      "r10" => regs.r10,
      "r11" => regs.r11,
      "r12" => regs.r12,
      "r13" => regs.r13,
      "r14" => regs.r14,
      "r15" => regs.r15,
      "rsp" => regs.rsp,
      "rbp" => regs.rbp,
      "rsi" => regs.rsi,
      "rdi" => regs.rdi,
      "rip" => regs.rip,
      "cs"  => regs.cs,
      "ds"  => regs.ds,
      "es"  => regs.es,
      "fs"  => regs.fs,
      "gs"  => regs.gs,
      "ss"  => regs.ss,
      "eflags" => regs.eflags,
      _ => panic!("Reg : {} not found", reg_name)
    } as usize;
  }

  pub fn set_reg_value(&mut self, reg_name: &str, value: &usize) {
    let mut user_struct = self.get_user_struct();
    let value_u64 = *value as u64;
    match reg_name.trim().to_ascii_lowercase().as_str() {
      "rax" => user_struct.regs.rax = value_u64,
      "rbx" => user_struct.regs.rbx = value_u64,
      "rcx" => user_struct.regs.rcx = value_u64,
      "rdx" => user_struct.regs.rdx = value_u64,
      "r8"  => user_struct.regs.r8 = value_u64,
      "r9"  => user_struct.regs.r9 = value_u64,
      "r10" => user_struct.regs.r10 = value_u64,
      "r11" => user_struct.regs.r11 = value_u64,
      "r12" => user_struct.regs.r12 = value_u64,
      "r13" => user_struct.regs.r13 = value_u64,
      "r14" => user_struct.regs.r14 = value_u64,
      "r15" => user_struct.regs.r15 = value_u64,
      "rsp" => user_struct.regs.rsp = value_u64,
      "rbp" => user_struct.regs.rbp = value_u64,
      "rsi" => user_struct.regs.rsi = value_u64,
      "rdi" => user_struct.regs.rdi = value_u64,
      "rip" => user_struct.regs.rip = value_u64,
      "cs"  => user_struct.regs.cs = value_u64,
      "ds"  => user_struct.regs.ds = value_u64,
      "es"  => user_struct.regs.es = value_u64,
      "fs"  => user_struct.regs.fs = value_u64,
      "gs"  => user_struct.regs.gs = value_u64,
      "ss"  => user_struct.regs.ss = value_u64,
      "eflags" => user_struct.regs.eflags = value_u64,
      _ => panic!("reg :{} not found", reg_name)
    }
      self.set_user_struct(&user_struct);
    }

  pub fn run(&mut self) {
    self.wait();
    let mut last_input: String = String::new();
    println!("Start debugging process: {}", self.pid());
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
          self.cont();
        }
        "lsb" => {
          let breakpoints = self.breakpoints();
          for i in 0..breakpoints.len() {
            println!("br {}: 0x{:016x}, data: 0x{:016x}", i, breakpoints[i].addr, breakpoints[i].orig_byte);
          }
        }
        "b" | "break" => {
          if input_split.len() < 2 {
            println!("Usage: b [addr]");
          } else {
            let addr = str2usize(&input_split[1]);
            self.set_breakpoint(addr);
            println!("Set breakpoint addr: 0x{:016x}", addr);
          }
        }
        "mem" => {
          if input_split.len() > 2 && input_split[1] == "get" {
            let addr = str2usize(&input_split[2]);
            let value = self.peek_data(addr);
            println!("(0x{:016x}): 0x{:016x}", addr, value);
          } else if input_split.len() > 3 && input_split[1] == "set" {
            let addr = str2usize(&input_split[2]);
            let value = str2usize(&input_split[3]);
            self.poke_data(addr, value);
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
            let reg_value: usize= self.get_reg_value(&reg_name);
            println!("Reg {}: 0x{:016x}", reg_name, reg_value)

          } else if input_split.len() > 3 && input_split[1] == "set" {

            let reg_name = input_split[2].trim();

            let set_value = str2usize(&input_split[3]);

            self.set_reg_value(&reg_name, &set_value);

          } else {
            let regs = self.get_user_struct().regs;

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
          println!("Pid: {}", self.pid);
        }

        "q" | "quit" => {
          exit(0);
        }
        _ => println!("Unknown command: {}", input),
      }
    }
  }
}

