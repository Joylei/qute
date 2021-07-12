use super::dev::Device;
use crate::Result;

/// high level abstraction of  EC, only for it8528
pub trait Controller {
    fn get_byte(&self, cmd: u16) -> Result<u8>;
    fn set_byte(&self, cmd: u16, value: u8) -> Result<()>;
}

#[inline(always)]
fn send_command(ec: &Device, cmd1: u8, cmd2: u8, cmd3: u8) -> Result<()> {
    ec.write_cmd_port(cmd1)?;
    ec.write_data_port(cmd2)?;
    ec.write_data_port(cmd3)
}

impl Controller for Device {
    fn get_byte(&self, cmd: u16) -> Result<u8> {
        self.clear_buffer();
        let byte0 = (cmd >> 8 & 0xff) as u8;
        let byte1 = (cmd & 0xff) as u8;
        send_command(self, 0x88, byte0, byte1)?;
        self.read_data_port()
    }

    fn set_byte(&self, cmd: u16, value: u8) -> Result<()> {
        //self.ec.clear_buffer();
        let byte0 = (cmd >> 8 & 0xff) as u8;
        let byte1 = (cmd & 0xff) as u8;
        send_command(self, 0x88, byte0 | 0x80, byte1)?;
        self.write_data_port(value)
    }
}
