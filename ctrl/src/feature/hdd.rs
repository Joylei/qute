use super::Feature;
use crate::errors::*;
use crate::hal::ec::Controller;
use std::process::Command;
// use hdd::Device;
// use hdd::scsi::{SCSIDevice, SCSICommon};

pub trait DiskDrive: Feature {
    /// check all of disk entry standby mode
    fn get_status() -> Result<()> {
        // let disks = list_disk()?;
        // for disk in disks {
        //     let dev = Device::open(disk)?;
        //     let (sense, data) = dev.scsi_inquiry();
        // }
        Ok(())
    }
}

fn list_disk() -> Result<Vec<String>> {
    let output = Command::new("ls").arg("/sys/block").output()?;
    let res = output
        .split_whitespace()
        .filter(|x| x.starts_with("sda") || x.starts_with("da"))
        .map(|x| format!("/dev/{}", x))
        .collect();
    Ok(res)
}
