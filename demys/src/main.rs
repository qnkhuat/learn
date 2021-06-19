use std::process::{exit};
use libc;
mod unistd;
mod ptrace;

fn run_child(prog: &String, argv: &[String]) {
    println!("Receive program: {}", prog);
    ptrace::traceme().unwrap();
    unistd::execv(prog, argv);
}

fn run_parent(pid: libc::pid_t) {
    println!("Parent process: {}", unistd::getpid());
    unistd::wait().unwrap();
    loop {
        let mut input = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.as_str().trim() {
                    "c" => {
                        //ptrace::cont(pid, None).expect("Failed continue process");
                        ptrace::cont(pid).unwrap();
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
    let pid = unistd::fork().unwrap();
    if pid == 0 { // child
        let cpid = unistd::getpid();
        println!("I'm a new child process has pid: {}", cpid);
        //// create a vector of zero terminated strings
        //let args = std::env::args().map(|arg| CString::new(arg).unwrap() ).collect::<Vec<CString>>();
        //// convert the strings to raw pointers
        //let c_args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<*const c_char>>();
        let args: Vec<String> = std::env::args().collect();
        let target: &String = &args[1];
        run_child(&target, &args[1..]);
    } else if pid > 0 { // Parent
        let ppid = unistd::getpid();
        println!("Parent process pid: {}, with child pid: {}", ppid, pid);
        run_parent(pid);
    } else {
        panic!("fork() failed");
    }
}
