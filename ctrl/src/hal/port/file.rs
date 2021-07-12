use super::{Port, ReadByte, WriteByte};
use crate::ffi;
use libc;
use std::fmt;
use std::fs;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use std::os::unix::fs::OpenOptionsExt;
use std::os::unix::io::AsRawFd;
use std::thread;
use std::time::Duration;

pub struct FilePort {
    port: u16,
    fp: fs::File,
}

impl FilePort {
    pub fn open(port: u16) -> Result<Self> {
        unsafe {
            //if libc::iopl(3) != 0 {
            if ffi::ioperm(port.into(), 1, 1) != 0 {
                return Err(format!("Failed to obtain io permission at port {:#04x}", port).into());
            }
        }
        let file = "/dev/port";
        let mut fp = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(file)
            .chain_err(|| format!("Failed to open port: {}", port))?;

        fp.seek(SeekFrom::Start(port.into()))
            .chain_err(|| format!("Failed to open port: {}", port))?;
        Ok(Self { port, fp })
    }
}

impl Port for FilePort {
    fn get_port(&self) -> u16 {
        self.port
    }
}

impl fmt::Display for FilePort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FilePort[{:#04x}]", self.port)
    }
}

const WAIT_INTERVAL: u64 = 0x32;
impl ReadByte for FilePort {
    fn read(&mut self) -> Result<u8> {
        let buf = &mut [0_u8; 1];
        //self.fp.read_exact(&mut buf[0..1])?;
        for _ in 0..20 {
            let count = unsafe {
                libc::read(
                    self.fp.as_raw_fd(),
                    buf.as_mut_ptr() as *mut libc::c_void,
                    buf.len(),
                )
            };
            if count > 0 {
                return Ok(buf[0]);
            }
            thread::sleep(Duration::from_millis(5));
        }
        Err("time out while reading a byte".into())
    }
}

impl WriteByte for FilePort {
    fn write(&mut self, value: u8) -> Result<()> {
        let buf = &[value];
        //self.fp.write(&mut buf)?;
        unsafe {
            libc::write(
                self.fp.as_raw_fd(),
                buf.as_ptr() as *const libc::c_void,
                buf.len(),
            );
        }
        Ok(())
    }
}
