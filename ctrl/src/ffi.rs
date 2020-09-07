use std::os::raw::{c_int, c_ulong, c_void};

/// some functions of crate libc does not work under Alpine linux; so link libc here
extern "C" {
    pub fn iopl(level: c_int) -> c_int;
    pub fn ioperm(port: c_ulong, count: c_ulong, enabled: c_int) -> c_int;
    pub fn ioctl(fd: c_int, cmd: c_ulong, buf: *mut c_void) -> c_int;
}
