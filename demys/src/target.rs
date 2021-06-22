use libc::pid_t;
use crate::unistd;
use crate::ptrace;

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
    self.enabled = true;

    /* 0xCC is the machine code for int $3 on x86_64 - the interupt instruction */
    ptrace::poke_data(self.pid, self.addr, 0xcc);
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

  pub fn breakpoints(&mut self) -> Vec<Breakpoint>{
    return self.breakpoints.clone();

  }

  pub fn run(&mut self, argv: &[String]) {
    ptrace::traceme().unwrap();
    unistd::execv(&self.executable, argv).unwrap();
  }

  pub fn singlestep(&mut self) {
    ptrace::singlestep(self.pid).unwrap();
  }

  pub fn kill(&mut self) {
    unistd::kill(self.pid);
  }

  pub fn cont(&mut self) {
    println!("Before step over : 0x{:016x}", self.get_pc());
    self.step_over_breakpoint();
    println!("After step over : 0x{:016x}", self.get_pc());
    ptrace::cont(self.pid).unwrap();
    println!("Continued process : 0x{:016x}", self.get_pc());
    self.wait();
    println!("Wait again process : 0x{:016x}", self.get_pc());
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
    
    /* 0xCC is the machine code for int $3 on x86_64 - the interupt instruction */
    self.poke_data(addr, 0xCC);

    let data = self.peek_data(addr);
    let bp = Breakpoint{
      pid: self.pid,
      addr: addr, 
      orig_byte: data,
      enabled:true
    };
    
    self.breakpoints.push(bp);
  }

  pub fn step_over_breakpoint(&mut self) {
    // - 1 because execution will go pass the breakpoint
    let prev_pc_location = self.get_pc() - 1; 
    println!("Current_pc_location: 0x{:016x},  prev_pc_location: 0x{:016x}, len: {}", self.get_pc(), prev_pc_location, self.breakpoints.len());

    for i in 0..self.breakpoints.len() {
      println!("Searching :{}, addr: 0x{:016x}, enabled: {}", i, self.breakpoints[i].addr, self.breakpoints[i].enabled);
      println!("Do you even exist? :{} ", prev_pc_location == self.breakpoints[i].addr);
      //let mut bp = self.breakpoints[i].clone();
      if self.breakpoints[i].addr == prev_pc_location && self.breakpoints[i].enabled {
        self.set_pc(&prev_pc_location);
        println!("Reset the PC: 0x{:016x},  prev_pc_location: 0x{:016x}, The 2 should be equal now", self.get_pc(), prev_pc_location);

        self.breakpoints[i].disable();
        println!("PC before single step: 0x{:016x}", self.get_pc());
        self.singlestep();
        println!("PC after single step: 0x{:016x}", self.get_pc());
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

  }



