use crate::errors::*;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LedColor {
    Green,
    Red,
    Auto,
}

impl From<LedColor> for u8 {
    fn from(color: LedColor) -> u8 {
        match color {
            LedColor::Green => 0,
            LedColor::Red => 1,
            LedColor::Auto => 2,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FanMode {
    Auto,
    Manual,
    Custom(Option<u32>, Option<u32>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LedMode {
    Off,
    On,
    Blink,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PowerRecoveryMode {
    /// keep state before power loss
    Last,
    /// start up if power recovery
    On,
    /// shutdown if power recovery
    Off,
}

impl From<PowerRecoveryMode> for u8 {
    fn from(mode: PowerRecoveryMode) -> u8 {
        match mode {
            PowerRecoveryMode::Last => 2,
            PowerRecoveryMode::On => 1,
            PowerRecoveryMode::Off => 0,
        }
    }
}

impl From<u8> for PowerRecoveryMode {
    fn from(v: u8) -> PowerRecoveryMode {
        match v {
            2 => PowerRecoveryMode::Last,
            1 => PowerRecoveryMode::On,
            0 => PowerRecoveryMode::Off,
            _ => panic!("invalid input, must in range 0-2"),
        }
    }
}

impl FromStr for PowerRecoveryMode {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let t = s.trim().to_lowercase();
        let mode = match t.as_str() {
            "last" => PowerRecoveryMode::Last,
            "on" => PowerRecoveryMode::On,
            "off" => PowerRecoveryMode::Off,
            _ => return Err("invalid input, must be one of last|on|off".into()),
        };
        Ok(mode)
    }
}

impl fmt::Display for PowerRecoveryMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PowerRecoveryMode::Last => write!(f, "last"),
            PowerRecoveryMode::On => write!(f, "on"),
            PowerRecoveryMode::Off => write!(f, "off"),
        }
    }
}

impl PowerRecoveryMode {
    pub fn desc(&self) -> String {
        match self {
            PowerRecoveryMode::Last => String::from("restore previous NAS power state"),
            PowerRecoveryMode::On => String::from("turn on the NAS automatically"),
            PowerRecoveryMode::Off => String::from("keep the NAS turned off"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SwitchState {
    Off,
    On,
}

impl SwitchState {
    pub fn is_on(&self) -> bool {
        match self {
            SwitchState::On => true,
            _ => false,
        }
    }
}

impl From<bool> for SwitchState {
    fn from(v: bool) -> SwitchState {
        match v {
            true => SwitchState::On,
            _ => SwitchState::Off,
        }
    }
}

impl fmt::Display for SwitchState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SwitchState::Off => write!(f, "off"),
            _ => write!(f, "on"),
        }
    }
}

impl FromStr for SwitchState {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let t = s.trim().to_lowercase();
        let mode = {
            if t == "on" {
                Self::On
            } else if t == "off" {
                Self::Off
            } else {
                return Err("invalid input, must be one of on|off".into());
            }
        };
        Ok(mode)
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ShutdownMode {
    Shutdown,
    Reboot,
}

impl From<ShutdownMode> for u8 {
    fn from(mode: ShutdownMode) -> u8 {
        match mode {
            ShutdownMode::Shutdown => 0x20,
            ShutdownMode::Reboot => 0x40,
        }
    }
}
