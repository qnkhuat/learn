use libc::pid_t;
use crate::unistd;
use crate::ptrace;

// *** Breakpoint ***
struct Breakpoint {
  addr: usize,
  enabled: bool,
  orig_byte: u8,
}

pub fn get_reg_by_name<'a>(regs: &libc::user_regs_struct, reg_name: &str) -> Result<u64, &'a str> {
  return match reg_name.trim().to_ascii_lowercase().as_str() {
    "rax" => Ok(regs.rax),
    "rbx" => Ok(regs.rbx),
    "rcx" => Ok(regs.rcx),
    "rdx" => Ok(regs.rdx),
    "r8"  => Ok(regs.r8),
    "r9"  => Ok(regs.r9),
    "r10" => Ok(regs.r10),
    "r11" => Ok(regs.r11),
    "r12" => Ok(regs.r12),
    "r13" => Ok(regs.r13),
    "r14" => Ok(regs.r14),
    "r15" => Ok(regs.r15),
    "rsp" => Ok(regs.rsp),
    "rbp" => Ok(regs.rbp),
    "rsi" => Ok(regs.rsi),
    "rdi" => Ok(regs.rdi),
    "rip" => Ok(regs.rip),
    "cs"  => Ok(regs.cs),
    "ds"  => Ok(regs.ds),
    "es"  => Ok(regs.es),
    "fs"  => Ok(regs.fs),
    "gs"  => Ok(regs.gs),
    "ss"  => Ok(regs.ss),
    "eflags" => Ok(regs.eflags),
    _ => Err("Reg not found")
  };
}


pub fn set_reg_by_name(regs: &mut libc::user_regs_struct, reg_name: &str, value: &u64) {
  match reg_name.trim().to_ascii_lowercase().as_str() {
    "rax" => regs.rax = *value,
    "rbx" => regs.rbx = *value,
    "rcx" => regs.rcx = *value,
    "rdx" => regs.rdx = *value,
    "r8"  => regs.r8 = *value,
    "r9"  => regs.r9 = *value,
    "r10" => regs.r10 = *value,
    "r11" => regs.r11 = *value,
    "r12" => regs.r12 = *value,
    "r13" => regs.r13 = *value,
    "r14" => regs.r14 = *value,
    "r15" => regs.r15 = *value,
    "rsp" => regs.rsp = *value,
    "rbp" => regs.rbp = *value,
    "rsi" => regs.rsi = *value,
    "rdi" => regs.rdi = *value,
    "rip" => regs.rip = *value,
    "cs"  => regs.cs = *value,
    "ds"  => regs.ds = *value,
    "es"  => regs.es = *value,
    "fs"  => regs.fs = *value,
    "gs"  => regs.gs = *value,
    "ss"  => regs.ss = *value,
    "eflags" => regs.eflags = *value,
    _ => panic!("reg :{}not found", reg_name)
  };
}

// *** Target Program ***
pub struct TargetProgram {
  pub pid: pid_t,
  pub executable: String,
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

  pub fn run(&self, argv: &[String]) {
    ptrace::traceme().unwrap();
    unistd::execv(&self.executable, argv);
  }

  pub fn step(&self) {
  }

  pub fn cont(&self) {
    ptrace::cont(self.pid).unwrap();
    self.wait();
  }
  
  pub fn peek_data(&self, addr: usize) -> u8 {
    /* align to 8 bytes */
    let loc = (addr / 8) * 8;
    let offset = addr % 8;
    let data = ptrace::peek_data(self.pid, loc).unwrap();
    let res = ((data & (0xff << (8 * offset))) >> (8 * offset)) as u8;
    //println!("output data: {}", data);
    //println!("offset: {}", offset);
    //println!("Converted data: {}", res);
    //println!("Addr: {}, Loc: {}", addr, loc);
    return res;
  }

  pub fn poke_data(&self, addr: usize, data: u8) {
    let loc = (addr / 8) * 8;
    let offset = addr % 8;
    let orig_data = ptrace::peek_data(self.pid, loc).unwrap();
    let new_data = (orig_data & !(0xff << (8 * offset))) | ((data as usize) << (8 * offset));
    //println!("orig_data: {}", orig_data);
    //println!("to_write_data: {}", data);
    //println!("Converted data: {}", new_data);

    ptrace::poke_data(self.pid, loc, new_data);
  }

  pub fn wait(&self) {
    unistd::wait().unwrap();
  }
  
  pub fn pid(&self) -> &pid_t {&self.pid}


  pub fn set_breakpoint(&mut self, addr: usize) {
    let data = self.peek_data(addr);

    /* 0xCC is the machine code for int $3 on x86_64 - the interupt instruction */
    self.poke_data(addr, 0xCC);
    let bp = Breakpoint{
      addr: addr, 
      orig_byte: data,
      enabled:true
    };
    
    self.breakpoints.push(bp);


    // Peek data and save the current data to orig_byte
    // Set the int3 instruction at the address

  }

  pub fn get_user_struct(&mut self) -> libc::user {
    unsafe {
      let mut user_struct: libc::user = std::mem::uninitialized();
      ptrace::get_user_struct(self.pid, &mut user_struct);
      return user_struct;
    }
  }

  pub fn set_user_struct(&mut self, user_struct: &libc::user) {
    unsafe {
      ptrace::set_user_struct(self.pid, user_struct);
    }
  }


}

