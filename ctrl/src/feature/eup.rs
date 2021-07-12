use super::Feature;
use crate::{hal::ec::Controller, types::SwitchState, Error, Result};

pub trait EupControl: Feature {
    fn get_eup_state(&self) -> Result<SwitchState> {
        trace!("try to get eup state by EC");
        self.with_ec(|ec| {
            //cmd = 0x101
            let v = ec.get_byte(0x101)?;
            if v & 8 == 0 {
                return Err(Error::InvalidValue("eup not supported".to_owned()));
            }
            //cmd = 0x121
            let v = ec.get_byte(0x121)?;
            trace!("raw value of eup state: {}", v);
            Ok(SwitchState::from(v & 8 != 0))
        })
    }

    fn set_eup_state(&self, state: SwitchState) -> Result<()> {
        trace!("try to set eup state to {} by EC", state);
        self.with_ec(|ec| {
            //cmd = 0x101
            let v = ec.get_byte(0x101)?;
            if v & 8 == 0 {
                return Err(Error::InvalidValue("eup not supported".to_owned()));
            }
            //cmd = 0x121
            let cur_val = ec.get_byte(0x121)?;
            trace!("raw value of eup state: {}", cur_val);
            let mask = if state.is_on() { 8 } else { 0 };
            let dst_val = (cur_val & 0xf7) | mask;
            trace!("raw value of eup state to set: {}", dst_val);
            //cmd = 0x121
            ec.set_byte(0x121, dst_val)
        })
    }
}
