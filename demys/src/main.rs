use std::process::{exit};
use libc;
mod unistd;
mod ptrace;
mod target;

fn execute_debugee(executable: &String, argv: &[String]) {
  ptrace::traceme().unwrap();
  unistd::execv(executable, argv).unwrap();
}

fn start(executable: &String, argv:&[String]) {
  let pid = unistd::fork().unwrap();
  let mut prog = target::TargetProgram::new(pid, executable);

  if pid == 0 { // child
    unsafe {
      libc::personality(libc::ADDR_NO_RANDOMIZE as u64);
    }
    execute_debugee(executable, argv);
  } else if pid > 0 { // Parent
    prog.run();
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


