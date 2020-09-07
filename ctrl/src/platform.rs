use super::feature::*;
use crate::errors::*;
use crate::hal::ec::{Controller, Device};
use crate::hal::lock::Mutex;
use crate::hal::sio::Controller as SuperIO;

fn check_platform() -> Result<()> {
    //ITE 0x2E,0x2F or 0x4E, 0x4F
    let mut sio = SuperIO::create(0x2E, 0x2F)?;
    //sio.enter_pnp()?;
    //sio.select_logical_device(0x1)?;
    let id = sio.read_word(0x20)?; //0x20: id addr
    let ver = sio.read_byte(0x22)?; //0x22: version addr
    trace!("Chip : {:#04x}, version: {:#02x}", id, ver);
    if id == 0x8528 {
        Ok(())
    } else {
        Err("only IT8528 is supported".into())
    }
}

pub struct Platform {
    ec: Mutex<Device>,
}

impl Platform {
    #[inline]
    pub fn new(ec: Device) -> Self {
        const KEY: i32 = 0x4543; // 'ec', qnap use it, if you want to run it in QTS system
        Self {
            ec: Mutex::new(KEY, ec),
        }
    }

    pub fn with_ports(cmd_port: u16, data_port: u16) -> Result<Self> {
        check_platform()?;
        let ec = Device::create(cmd_port, data_port)?;
        Ok(Self::new(ec))
    }

    pub fn with_default() -> Result<Self> {
        const CMD_PROT: u16 = 0x6c;
        const DATA_PORT: u16 = 0x68;
        Self::with_ports(CMD_PROT, DATA_PORT)
    }
}

impl Feature for Platform {
    fn with_ec<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&dyn Controller) -> Result<R>,
    {
        let ec = &*self.ec.lock()?;
        let val = f(ec)?;
        Ok(val)
    }
}

impl EupControl for Platform {}

impl FanControl for Platform {}

impl LedControl for Platform {}

impl Power for Platform {}

impl Temperature for Platform {}

impl UsbControl for Platform {}

impl Firmware for Platform {}
