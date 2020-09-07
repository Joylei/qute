extern crate cpuio;
extern crate libc;
use super::{Port, ReadByte, WriteByte};
use crate::errors::*;
use crate::ffi;
use std::fmt;

pub struct AsmPort {
    port: u16,
    io: cpuio::UnsafePort<u8>,
}

impl AsmPort {
    pub fn open(port: u16) -> Result<Self> {
        unsafe {
            //if libc::iopl(3) != 0 {
            if ffi::ioperm(port.into(), 1, 1) != 0 {
                return Err(format!("Failed to obtain io permission at port {:#04x}", port).into());
            }
        }
        let value = Self {
            port,
            io: unsafe { cpuio::UnsafePort::new(port) },
        };
        Ok(value)
    }
}

impl Port for AsmPort {
    #[inline]
    fn get_port(&self) -> u16 {
        self.port
    }
}

impl fmt::Display for AsmPort {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AsmPort[{:#04x}]", self.port)
    }
}

impl ReadByte for AsmPort {
    #[inline]
    fn read(&mut self) -> Result<u8> {
        let value = unsafe { self.io.read() };
        Ok(value)
    }
}

impl WriteByte for AsmPort {
    #[inline]
    fn write(&mut self, value: u8) -> Result<()> {
        unsafe {
            self.io.write(value);
        }
        Ok(())
    }
}
