use libc;
use libc::pid_t; // ~i32

pub fn traceme() ->Result<(), std::io::Error>{
  unsafe {
    libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0);
  };
  return Ok(());
}

pub fn cont(pid: pid_t) -> Result<(), std::io::Error> {
  unsafe {
    libc::ptrace(libc::PTRACE_CONT, pid, 0, 0);
  };
  return Ok(());
}

pub fn singlestep(pid: pid_t) -> Result<(), std::io::Error> {
  unsafe {
    libc::ptrace(libc::PTRACE_SINGLESTEP, pid, 0, 0);
  };
  return Ok(());
}


// Read data at address
pub fn peek_data(pid: pid_t, addr: usize) -> Result<usize, i32> {
  unsafe {
    *libc::__errno_location() = 0;
    let data: i64 = libc::ptrace(libc::PTRACE_PEEKDATA, pid, addr, 0);
    if data == -1 && *libc::__errno_location() != 0 {
      return Err(*libc::__errno_location());
    } else {
      return Ok(data as usize);
    }
  }
}

// Overwrite data at address
pub fn poke_data(pid: pid_t, addr: usize, data:usize) {
  unsafe {
    libc::ptrace(libc::PTRACE_POKEDATA, pid, addr, data);
  }
}


pub fn get_user_struct(pid: pid_t, user_struct: *mut libc::user) {
  unsafe {
    libc::ptrace(libc::PTRACE_GETREGS, pid, 0, user_struct);
  }
}


pub fn set_user_struct(pid: pid_t, user_struct: *const libc::user) {
  unsafe {
    libc::ptrace(libc::PTRACE_SETREGS, pid, 0, user_struct);
  }
}

pub fn peek_user(pid: pid_t, reg_id: pid_t) -> Result<i64, i32> {
  unsafe {
    /* clear errno */
    *libc::__errno_location() = 0;
    let ret = libc::ptrace(libc::PTRACE_PEEKUSER, pid, 8 * reg_id, 0);
    if ret == -1 && *libc::__errno_location() != 0 {
      return Err(*libc::__errno_location());
    } else {
      return Ok(ret);
    }
  }
}
