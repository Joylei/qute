use super::port::{open, Port};
use crate::Result;

pub struct Controller {
    index_port: Box<dyn Port>,
    data_port: Box<dyn Port>,
    pnp: bool,
}

impl Controller {
    #[inline]
    pub fn new(index_port: Box<dyn Port>, data_port: Box<dyn Port>) -> Self {
        Self {
            index_port,
            data_port,
            pnp: false,
        }
    }
    pub fn create(index_port: u16, data_port: u16) -> Result<Self> {
        let index_port = open(index_port)?;
        let data_port = open(data_port)?;
        Ok(Self::new(index_port, data_port))
    }

    #[inline]
    pub fn read_byte(&mut self, reg: u8) -> Result<u8> {
        self.index_port.write(reg)?;
        self.data_port.read()
    }

    #[inline]
    pub fn read_word(&mut self, reg: u8) -> Result<u16> {
        let hi = self.read_byte(reg)? as u16;
        let lo = self.read_byte(reg + 1)? as u16;
        Ok((hi << 8) | lo)
    }

    /// select logical device
    #[inline]
    pub fn select_logical_device(&mut self, ldn: u8) -> Result<()> {
        self.index_port.write(0x07)?;
        self.data_port.write(ldn)
    }

    #[inline]
    pub fn select_special_function(&mut self, reg1: u8, reg2: u8) -> Result<()> {
        self.index_port.write(reg1)?;
        self.data_port.write(reg2)
    }

    #[inline]
    pub fn enter_pnp(&mut self) -> Result<()> {
        //ITE special
        self.index_port.write(0x87)?;
        self.index_port.write(0x01)?;
        self.index_port.write(0x55)?;
        self.index_port.write(0x55)?;
        self.pnp = true;
        Ok(())
    }

    #[inline]
    fn exit_pnp(&mut self) -> Result<()> {
        if self.pnp {
            //ITE special
            self.index_port.write(0x02)?;
            self.data_port.write(0x02)?;
        }
        Ok(())
    }
}

impl Drop for Controller {
    fn drop(&mut self) {
        self.exit_pnp();
    }
}
