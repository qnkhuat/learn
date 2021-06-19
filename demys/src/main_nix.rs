use nix::unistd::{fork, ForkResult, getpid, Pid, execv};
use nix::sys::{ptrace, wait};
use std::process::{Command, exit};
use std::io::{self, Write};
use std::ffi::CStr;

struct Debugger {
    prog_name: String,
    pid: Pid,
}

impl Debugger {
    pub fn run() {}
    pub fn continue() {}
    pub fn step() {}
}

fn run_child(prog: &str) {
    println!("Receive program: {}", prog);
    ptrace::traceme().unwrap();

    //let output = Command::new(prog).output().expect("Hello world!");
    //execv(prog, vec![]);
    io::stdout().write_all(&output.stdout).unwrap();
}


fn run_parent(pid: Pid) {
    println!("Parent process: {}", getpid());
    wait::wait().unwrap();
    loop {
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(n) => {
                match input.as_str().trim() {
                    "c" => {
                        ptrace::cont(pid, None).expect("Failed continue process");
                    }
                    "q" => {
                        exit(0);
                    }
                    _ => println!("Unknown command: {}", input),
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}

fn main() {

    match unsafe{fork()} {

        Ok(ForkResult::Parent { child }) => {
            let pid = getpid();
            println!("Parent process pid: {}, with child pid: {}", pid, child);
            run_parent(child);
        }

        Ok(ForkResult::Child) => {
            let pid = getpid();
            println!("I'm a new child process has pid: {}", pid);
            run_child("./C/hello")
        }

        Err(err) => { 
            panic!("fork() failed: {}", err);
        }
        
    }
}
