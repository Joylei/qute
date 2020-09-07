use super::Feature;
use crate::errors::*;
use crate::hal::ec::Controller;

pub trait DiskDrive: Feature {
    /// check all of disk entry standby mode
    fn get_status() {
        //se_get_disk_power_status
    }
}
