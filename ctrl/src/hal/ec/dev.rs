//the embedded controller (EC), via CPU I/O ports 0x68 and 0x6c
//http://wiki.laptop.org/go/Revised_EC_Port_6C_Command_Protocol
//http://wiki.laptop.org/go/Ec_specification#Old_Port_6c_Command_Protocol
//https://blog.csdn.net/zhao_longwei/article/details/50454779
use super::status::Status;
use crate::errors::*;
use crate::hal::port::{open, Port};
use std::cell::RefCell;
use std::fmt;
use std::io;
use std::ops;
use std::ops::Not;
use std::thread;
use std::time::Duration;

const SPIN_INTERVAL: u64 = 0x32;

///how many times spinning for status; total time = times * SPIN_INTERVAL
const WAIT_TIMES: u32 = 20;

/// low level abstraction of EC
pub struct Device {
    pub cmd_port: RefCell<Box<dyn Port>>,
    pub data_port: RefCell<Box<dyn Port>>,
}

impl Device {
    #[inline]
    pub fn create(cmd_port: u16, data_port: u16) -> Result<Self> {
        Ok(Self::new(open(cmd_port)?, open(data_port)?))
    }

    #[inline]
    pub fn new(cmd_port: Box<dyn Port>, data_port: Box<dyn Port>) -> Self {
        Self {
            cmd_port: RefCell::new(cmd_port),
            data_port: RefCell::new(data_port),
        }
    }

    fn poll_until(&self, mask: Status, times: u32) -> Result<()> {
        let p = &mut *self.cmd_port.borrow_mut();
        for _ in 0..times {
            let status = p.read()?;
            if mask.satisfied(status) {
                return Ok(());
            }
            thread::sleep(Duration::from_millis(SPIN_INTERVAL));
        }
        Err(format!(
            "ec: timed out waiting for {} at port {:#04x}",
            mask,
            p.get_port()
        )
        .into())
    }

    /// wait completion of other command, and clear buffer
    #[inline]
    pub fn clear_buffer(&self) {
        let status = (*self.cmd_port.borrow_mut()).read().unwrap();
        if Status::OBF.satisfied(status) {
            let _ = self.read_data_port();
        }
    }

    #[inline]
    pub fn write_data_port(&self, value: u8) -> Result<()> {
        //wait for IBF=0, thus data or command was processed by EC
        self.poll_until(Status::IBF.not(), WAIT_TIMES)?;
        (*self.data_port.borrow_mut()).write(value)
    }

    #[inline]
    pub fn read_data_port(&self) -> Result<u8> {
        //wait for OBF=1, there is data to read
        self.poll_until(Status::OBF, WAIT_TIMES)?;
        (*self.data_port.borrow_mut()).read()
    }

    #[inline]
    pub fn write_cmd_port(&self, value: u8) -> Result<()> {
        //wait for IBF=0, thus data or command was processed by EC
        self.poll_until(Status::IBF.not(), WAIT_TIMES)?;
        (*self.cmd_port.borrow_mut()).write(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Status;
    #[test]
    fn Status_Idle_satisfied() {
        assert_eq!(Status::Idle.satisfied(0), true);
    }

    #[test]
    fn Status_IBF_satisfied() {
        assert_eq!(Status::IBF.satisfied(0b10), true);
    }

    #[test]
    fn Status_OBF_satisfied() {
        assert_eq!(Status::OBF.satisfied(0b01), true);
    }

    #[test]
    fn Status_NOT_OBF_satisfied() {
        assert_eq!(Status::Not(Box::new(Status::OBF)).satisfied(0), true);
        assert_eq!(Status::Not(Box::new(Status::OBF)).satisfied(0b10), true);
    }

    #[test]
    fn Status_ops_not() {
        assert_eq!((!Status::OBF).satisfied(0), true);
    }
}
