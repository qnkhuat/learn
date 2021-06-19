use libc::{pid_t};
use std::ffi::{CString};

pub fn fork() -> Result<pid_t, std::io::Error> {
  let res = unsafe {
    libc::fork()
  };
  return Ok(res);
}

pub fn getpid() -> pid_t {
  unsafe {libc::getpid()}
}

pub fn getppid() -> pid_t {
  unsafe {libc::getppid()}
}

pub fn wait() -> Result<pid_t, std::io::Error>{
  let mut status: pid_t = 0;
  unsafe {libc::waitpid(-1, &mut status, 0);}

  return Ok(status);
}

pub fn execv(target: &String, argv: &[String]) -> Result<(), std::io::Error>{
  unsafe {
    let ctarget_m = CString::new(target.clone()).unwrap();
    let ctarget = ctarget_m.as_ptr();

    let mut vec_argv: Vec<*const i8> = Vec::new();
    vec_argv.push(ctarget);
    vec_argv.push(std::ptr::null());
    libc::execv(ctarget, vec_argv.as_ptr());
  }
  return Ok(());
}

