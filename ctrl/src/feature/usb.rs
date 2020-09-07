use super::Feature;
use crate::errors::*;
use crate::hal::ec::Controller;

pub trait UsbControl: Feature {
    /// usb copy button pressed or not
    fn get_usb_button(&self) -> Result<bool> {
        trace!("try to get usb button by EC");
        self.with_ec(|ec| {
            let value = ec.get_byte(0x143)?;
            Ok((value & 4) != 0)
        })
    }
}
