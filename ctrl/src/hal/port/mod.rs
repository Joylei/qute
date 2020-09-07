use crate::errors::*;
use std::fmt;
use std::io;

pub trait ReadByte {
    fn read(&mut self) -> Result<u8>;
}

pub trait WriteByte {
    fn write(&mut self, value: u8) -> Result<()>;
}

pub trait Port: ReadByte + WriteByte + fmt::Display {
    fn get_port(&self) -> u16;
}

mod asm;
mod file;
/// re-export
pub use asm::AsmPort;
pub use file::FilePort;

/// factory method
pub fn open(port: u16) -> Result<Box<dyn Port>> {
    // let is_root = unsafe { libc::geteuid() == 0 };
    // if is_root {
    let v = AsmPort::open(port)?;
    return Ok(Box::new(v));
    // }

    //let v = FileAccessor::open(port)?;
    //Ok(Box::new(v))
}
