extern crate libc;
use super::{Port, ReadByte, WriteByte};
use crate::errors::*;
use crate::ffi;
use std::fmt;

pub struct AsmPort {
    port: u16,
    //io: cpuio::UnsafePort<u8>,
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
            //io: unsafe { cpuio::UnsafePort::new(port) },
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
        let value = unsafe { inb(self.port) };
        Ok(value)
    }
}

impl WriteByte for AsmPort {
    #[inline]
    fn write(&mut self, value: u8) -> Result<()> {
        unsafe {
            outb(value, self.port);
        }
        Ok(())
    }
}

// https://github.com/emk/toyos-rs

#[inline]
unsafe fn outb(value: u8, port: u16) {
    llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
}

#[inline]
unsafe fn inb(port: u16) -> u8 {
    // The registers for the `in` and `out` instructions are always the
    // same: `a` for value, and `d` for the port address.
    let result: u8;
    llvm_asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    result
}
