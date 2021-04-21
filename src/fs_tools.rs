use crate::types::Drive;
use anyhow::Result;
use byte_unit::Byte;
use log::debug;
use std::path::PathBuf;

/// TODO: make this configurable
const QUOTA: f64 = 0.95;

pub fn get_drive_capacity(drive: &Drive) -> Result<u64> {
    let expanded = shellexpand::tilde(&drive.mountpoint);
    let drive_path = PathBuf::from(expanded.into_owned());

    let drive_space: Result<u64> = match drive.is_mounted() {
        true => {
            debug!("Loading real size for drive \"{}\"", &drive.name);
            Ok(fs2::available_space(&drive_path)?)
        }
        false => {
            debug!("Using estimated size for drive \"{}\"", &drive.name);
            let location_bytes = Byte::from_str(&drive.size)?;

            Ok(location_bytes.get_bytes() as u64)
        }
    };

    let drive_space = apply_quota(drive_space?, QUOTA);

    Ok(drive_space)
}

fn apply_quota(bytes: u64, quota: f64) -> u64 {
    assert!(0.0 <= quota && quota <= 1.0);

    let bytes = bytes as f64;
    let bytes = bytes * quota;

    bytes as u64
}
