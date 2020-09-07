use super::Feature;
use crate::errors::*;
use crate::hal::ec::Controller;

pub trait Temperature: Feature {
    fn get_temperature(&self, sensor_id: u8) -> Result<f32> {
        trace!("try to get temperature for cpu {} by EC", sensor_id);
        // 0-4 cpu
        // 5-9 sys
        // 10-14 power
        //15-38: env
        let cmd: u16 = match sensor_id as u16 {
            idx if idx <= 1 => idx + 0x600,
            idx if idx >= 5 && idx <= 7 => idx + 0x5fd,
            idx if idx == 10 => 0x659,
            idx if idx == 0xb => 0x65c,
            idx if idx >= 0xf && idx <= 0x26 => idx + 0x5f7,
            idx => return Err(format!("temperature: invalid sensor id {}", idx).into()),
        };

        self.with_ec(|ec| {
            let res = ec.get_byte(cmd)?;
            Ok(res as f32)
        })
    }

    fn temperature_calibrate(&self, sensor_id: u8, arg1: bool, arg2: u32) -> Result<()> {
        trace!(
            "try to do temperature calibrate for cpu {} by EC: {}, {}",
            sensor_id,
            arg1,
            arg2
        );
        self.with_ec(|ec| {
            //cmd = 0x2e2
            let value = ec.get_byte(0x2e2)?;
            let res = match (arg1, arg2) {
                (false, 0) => value & 0xfe,
                (false, _) => value | 1,
                (true, 0) => value & 0xfd,
                (true, _) => value | 2,
            };
            ec.set_byte(0x2e2, res)?;
            let cmd = match (arg1, arg2) {
                (true, _) => 0x27f,
                (_, _) => 0x27d,
            };
            ec.set_byte(cmd, sensor_id)
        })
    }
}
