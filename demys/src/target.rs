use libc::pid_t;
use crate::unistd;
use crate::ptrace;



pub struct TargetProgram {
  pub pid: pid_t,
  pub executable: String,
}


impl TargetProgram {
  pub fn new(pid: pid_t, path: &String) -> TargetProgram {
    TargetProgram{
      pid: pid,
      executable: (*path).clone()
    }
  }

  pub fn run(&mut self, argv: &[String]) {
    ptrace::traceme().unwrap();
    unistd::execv(&self.executable, argv);
  }

  pub fn step(&mut self) {
  }

  pub fn cont(&mut self) {
    ptrace::cont(self.pid).unwrap();
  }

  pub fn wait(&mut self) {
    unistd::wait().unwrap();
  }
  
}





























