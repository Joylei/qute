use crate::errors::*;
use crate::hal::ec::Controller;

pub trait Feature {
    fn with_ec<F, R>(&self, f: F) -> Result<R>
    where
        F: FnOnce(&dyn Controller) -> Result<R>;
}

mod eup;
mod fan;
mod fw;
mod led;
mod power;
mod temp;
mod usb;

//re-export
pub use eup::EupControl;
pub use fan::FanControl;
pub use fw::Firmware;
pub use led::LedControl;
pub use power::Power;
pub use temp::Temperature;
pub use usb::UsbControl;
