extern crate libc;

pub fn traceme() {
    unsafe {
        libc::ptrace(libc::PTRACE_TRACEME, 0, 0, 0);
    }
}

pub fn cont(pid: i32) {
    unsafe {
        libc::ptrace(libc::PTRACE_CONT, pid, 0, 0);
    }
}


