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

      match input.trim() {
        "c" => {
          prog.cont();
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
