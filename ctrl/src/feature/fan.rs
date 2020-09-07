use super::Feature;
use crate::errors::*;
use crate::hal::ec::Controller;
use std::io;

pub trait FanControl: Feature {
    fn get_fan_status(&self, fan_id: u8) -> Result<&'static str> {
        trace!("try to get fan status for fan {} by EC", fan_id);
        let cmd: u16 = match fan_id {
            idx if idx <= 4 => 0x242,
            idx if idx == 6 || idx == 7 => 0x244,
            idx if idx >= 0x14 && idx <= 0x19 => 0x259,
            idx if idx >= 0x1e && idx <= 0x23 => 0x25a,
            idx => return Err(format!("fan control: invalid fan id {}", idx).into()),
        };

        self.with_ec(|ec| {
            let temp = (fan_id & 0xf) as u8;
            let value = ec.get_byte(cmd)?;
            let status = match fan_id {
                idx if idx < 6 => (value >> temp & 1) == 0,
                idx if idx >= 6 && idx <= 7 => (value >> (temp - 6) & 1) == 0,
                idx if idx >= 0x14 && idx <= 0x19 => (value >> (temp - 0x14) & 1) == 0,
                _ => (value >> (temp - 0x1e) & 1) == 0,
            };
            Ok(if status { "NG" } else { "OK" })
        })
    }

    fn get_fan_speed(&self, fan_id: u8) -> Result<u16> {
        trace!("try to get speed for fan {} by EC", fan_id);
        let (cmd1, cmd2) = match fan_id as u16 {
            idx if idx <= 4 => ((idx + 0x312) * 2, idx * 2 + 0x625),
            idx if idx == 6 || fan_id == 7 => (0x223, 0x24b),
            idx if idx >= 0x14 && idx <= 0x19 => ((idx + 0x30a) * 2, (idx - 6) * 2 + 0x621),
            10 => (0x65b, 0x65a),
            0xb => (0x65e, 0x65d),
            idx if idx >= 0x14 && idx <= 0x19 => ((idx + 0x30e) * 2, (idx - 0x14) * 2 + 0x645),
            idx if idx >= 0x1e && idx <= 0x23 => ((idx + 0x2f8) * 2, (idx - 0x1e) * 23 + 0x62d),
            idx => return Err(format!("fan control: invalid fan id {}", idx).into()),
        };

        self.with_ec(|ec| {
            let v1 = ec.get_byte(cmd1)? as u16;
            let v2 = ec.get_byte(cmd2)? as u16;
            let speed = (v1 << 8) | v2;
            Ok(speed)
        })
    }

    /// set fan speed by pwm
    fn set_fan_speed(&self, fan_id: u8, speed: u8) -> Result<()> {
        trace!("set speed to {} for fan {} by EC", speed, fan_id);
        let fan_speed = (((speed as u16) * 0x64) / 0xFF) as u8;

        let (cmd1, cmd2) = match fan_id {
            idx if idx <= 4 => (0x220, 0x22e),
            idx if idx == 6 || idx == 7 => (0x223, 0x24b),
            idx if idx >= 0x14 && idx <= 0x19 => (0x221, 0x22f),
            idx if idx >= 0x1e && idx <= 0x23 => (0x222, 0x23b),
            idx => return Err(format!("fan control: invalid fan id {}", idx).into()),
        };

        self.with_ec(|ec| {
            ec.set_byte(cmd1, 0x10)?;
            ec.set_byte(cmd2, fan_speed)?;

            Ok(())
        })
    }

    fn get_fan_pwm(&self, fan_id: u8) -> Result<u8> {
        trace!("get pwm for fan {}", fan_id);
        let cmd = match fan_id {
            idx if idx <= 4 => 0x22e,
            idx if idx == 6 || idx == 7 => 0x24b,
            idx if idx >= 0x14 && idx <= 0x19 => 0x22f,
            idx if idx >= 0x1e && idx <= 0x23 => 0x23b,
            idx => return Err(format!("fan control: invalid fan id {}", idx).into()),
        };

        self.with_ec(|ec| {
            let value = ec.get_byte(cmd)? as u16;
            let res = (value * 0x100 - value) / 100;
            Ok(res as u8)
        })
    }

    /// set the slope of the Fan control of EC fw
    fn set_fan_control_slope(&self, fan_id: u8, slope: u8) -> Result<()> {
        trace!("set control slope to {} for fan {}", slope, fan_id);
        let cmd = match fan_id {
            idx if idx <= 4 => 0x296,
            idx if idx == 6 || idx == 7 => 0x295,
            idx if idx == 10 || idx == 0xb => {
                return Err(format!(
                    "fan control: set the fan slope to power fan {} is not supported",
                    idx
                )
                .into())
            }
            idx => return Err(format!("fan control: invalid fan id {}", idx).into()),
        };

        self.with_ec(|ec| ec.set_byte(cmd, slope))
    }
}
