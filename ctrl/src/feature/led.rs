use super::Feature;
use crate::{
    hal::ec::Controller,
    types::{LedColor, LedMode},
    Error, Result,
};

pub trait LedControl: Feature {
    /// set led brightness
    fn set_led_by_pwm(&self, val: u8) -> Result<()> {
        self.with_ec(|ec| {
            ec.set_byte(0x243, val)?;
            let t = ec.get_byte(0x245)?;
            let t = t | 0x10;
            ec.set_byte(0x245, t)?;
            ec.set_byte(0x246, val)?;
            let t = ec.get_byte(0x245)?;
            let t = t & 0xef;
            ec.set_byte(0x245, t)
        })
    }

    ///  led_status: green, red, auto
    fn set_fan_led(&self, fan_id: u8, color: LedColor) -> Result<()> {
        if fan_id > 7 {
            return Err(Error::InvalidValue(format!(
                "led control: invalid led index {}",
                fan_id
            )));
        }
        let cmd = 0x16e;
        let value: u8 = color.into();
        let value = if color != LedColor::Auto {
            value | 0x80
        } else {
            value
        };
        self.with_ec(|ec| ec.set_byte(cmd, value))
    }

    fn set_front_usb_led(&self, v: u8) -> Result<()> {
        self.with_ec(|ec| ec.set_byte(0x154, v))
    }

    /// set clear status of LED
    fn set_status_led(&self, color: LedColor, enable: bool) -> Result<()> {
        let v = match (color, enable) {
            (LedColor::Green, true) => 2,
            (LedColor::Red, true) => 1,
            (_, true) => {
                return Err(Error::InvalidValue(
                    "led control: invalid input values".to_owned(),
                ))
            }
            (_, false) => 0,
        };
        self.with_ec(|ec| ec.set_byte(0x155, v))
    }

    /// blink status LED
    fn blink_status_led(&self, color: LedColor, enable: bool) -> Result<()> {
        let v = match (color, enable) {
            (LedColor::Green, true) => 4,
            (LedColor::Red, true) => 3,
            (LedColor::Auto, true) => 5,
            (_, _) => 0,
        };
        self.with_ec(|ec| ec.set_byte(0x155, v))
    }

    fn set_enclosure_ident_led(&self, enable: bool) -> Result<()> {
        let cmd = 0x15e;
        let status = if enable { 1 } else { 2 };
        self.with_ec(|ec| ec.set_byte(cmd, status))
    }

    fn set_disk_active_led(&mut self, port_id: u8, enable: bool) -> Result<()> {
        let cmd = if enable { 0x15f } else { 0x157 };
        self.with_ec(|ec| ec.set_byte(cmd, port_id))
    }

    fn set_disk_ident_led(&mut self, port_id: u8, enable: bool) -> Result<()> {
        let cmd = if enable { 0x158 } else { 0x159 };
        self.with_ec(|ec| ec.set_byte(cmd, port_id))
    }
    fn set_present_led(&self, port_id: u8, enable: bool) -> Result<()> {
        let cmd = if enable { 0x15a } else { 0x15b };
        self.with_ec(|ec| ec.set_byte(cmd, port_id))
    }

    /// set GPIO bbu status  LED
    /// arg1: status = green, red, both
    /// arg2: enable = [01]
    fn set_bbu_led(&self, arg1: u8, enable: u8) -> Result<()> {
        self.with_ec(|ec| {
            let t = if let Ok(v) = ec.get_byte(0x7d) {
                let t = v | 3;
                match (arg1, enable) {
                    (0, y) if y != 0 => t & 0xfe,
                    (1, y) if y != 0 => t & 0xfd,
                    (2, y) if y != 0 => v & 0xfc,
                    (_, _) => t,
                }
            } else {
                0
            };
            ec.set_byte(0x7d, t)
        })
    }

    ///Set hd error led  by the specified port id
    fn set_disk_err_led(&self, port_id: u8, enable: bool) -> Result<()> {
        //ec_sys_set_error_led
        let cmd = if enable { 0x15c } else { 0x15d };
        self.with_ec(|ec| ec.set_byte(cmd, port_id))
    }

    ///Turn off/on the 10G NIC present LED
    #[allow(non_snake_case)]
    fn set_10G_led(&self, enable: bool) -> Result<()> {
        self.with_ec(|ec| ec.set_byte(0x167, enable as u8))
    }
}
