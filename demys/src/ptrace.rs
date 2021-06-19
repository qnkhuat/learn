use libc;

pub fn traceme() ->Result<(), std::io::Error>{
  unsafe {
    libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0);
  };
  return Ok(());
}

pub fn cont(pid: libc::pid_t) -> Result<(), std::io::Error>{
  unsafe {
    libc::ptrace(libc::PTRACE_CONT, pid, 0, 0);
  };
  return Ok(());
}
