use super::Feature;
use crate::{hal::ec::Controller, Result};

pub trait Firmware: Feature {
    /// get ec version
    fn get_version(&self) -> Result<String> {
        self.with_ec(|ec| {
            let mut bytes = &mut [0_u8; 9];
            let mut cmd = 0x308;
            for i in 0..(bytes.len() - 1) {
                bytes[i] = ec.get_byte(cmd)?;
                cmd += 1;
            }
            bytes[8] = 0;
            let cstr = std::ffi::CStr::from_bytes_with_nul(bytes).unwrap();
            let ver = String::from(cstr.to_str().unwrap().trim());
            Ok(ver)
        })
    }
}
