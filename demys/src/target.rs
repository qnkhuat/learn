use libc::pid_t;
use crate::unistd;
use crate::ptrace;

// *** Breakpoint ***
struct Breakpoint {
  addr: usize,
  enabled: bool,
  orig_byte: u8,
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

}

