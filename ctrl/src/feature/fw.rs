use super::Feature;
use crate::errors::*;
use crate::hal::ec::Controller;

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
            let cstr = std::ffi::CStr::from_bytes_with_nul(bytes).chain_err(|| "invalid data")?;
            let ver = String::from(cstr.to_str().chain_err(|| "invalid data")?.trim());
            Ok(ver)
        })
    }
}
