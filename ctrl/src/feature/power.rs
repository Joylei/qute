use super::Feature;
use crate::errors::*;
use crate::hal::ec::Controller;
use crate::types::{PowerRecoveryMode, ShutdownMode};
use std::io;

pub trait Power: Feature {
    /// reset button pressed or not
    fn get_reset_button(&self) -> Result<bool> {
        /// ec_sys_get_reset_button
        self.with_ec(|ec| {
            let value = ec.get_byte(0x143)?;
            Ok(value & 2 != 0)
        })
    }

    fn force_shutdown(&self, mode: ShutdownMode, timeout: u8) -> Result<()> {
        trace!("force shutdown");
        let flag = mode.into();
        self.with_ec(|ec| {
            ec.set_byte(0x245, flag)?;
            if mode == ShutdownMode::Reboot {
                if let Err(_) = util::check_rtc() {
                    if let Ok(val) = ec.get_byte(0x249) {
                        let val = val | 1;
                        ec.set_byte(0x249, val); //ignore error
                    }
                }
            }
            ec.set_byte(0x248, timeout)
        })
    }

    /// get power supply status
    fn get_power_supply_status(&self, arg1: u8) -> Result<()> {
        if arg1 < 1 || arg1 > 2 {
            return Err(format!("invalid parameter: {}", arg1).into());
        }
        self.with_ec(|ec| {
            let value = ec.get_byte(0x45)?;
            if (value >> (arg1 & 0x1f)) & 0b1 == 0 {
                return Err("bad power supply status".into());
            }
            Ok(())
        })
    }
    fn get_power_recovery_mode(&self) -> Result<PowerRecoveryMode> {
        trace!("try to get power recovery mode by EC");
        self.with_ec(|ec| {
            let value = ec.get_byte(0x16)?;
            trace!("raw value of power recovery mode: {}", value);
            Ok(value.into())
        })
    }
    fn set_power_recovery_mode(&self, mode: PowerRecoveryMode) -> Result<()> {
        trace!("try to set power recovery mode to {} by EC", mode);
        let value: u8 = mode.into();
        trace!("raw value of power recovery mode to set: {}", value);
        self.with_ec(|ec| ec.set_byte(0x16, value))
    }

    fn sata_power_on(&self, port_id: u8) -> Result<()> {
        self.with_ec(|ec| ec.set_byte(0x260, port_id))
    }
    fn sata_power_off(&self, port_id: u8) -> Result<()> {
        self.with_ec(|ec| ec.set_byte(0x261, port_id))
    }
}

mod util {
    use crate::errors::*;
    use crate::ffi;
    use libc;
    use std::fs::OpenOptions;
    use std::io;
    use std::os::raw::{c_int, c_ulong, c_void};
    use std::os::unix::fs::OpenOptionsExt;
    use std::os::unix::io::AsRawFd;

    pub fn check_rtc() -> Result<()> {
        let dev = "/dev/rtc";
        let fd = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(dev)
            .chain_err(|| format!("Failed to open {}", dev))?;
        let buf = &mut [0_u8; 8];
        let cmd = 0x80287010;

        unsafe {
            if ffi::ioctl(fd.as_raw_fd(), cmd, buf.as_mut_ptr() as *mut c_void) < 0 {
                return Err(io::Error::last_os_error())
                    .chain_err(|| "Failed to perform ioctl command on /dev/rtc");
            }
        }
        if buf[0] != 0 {
            return Ok(());
        }
        Err(format!("Failed to read from {}", dev).into())
    }
}
