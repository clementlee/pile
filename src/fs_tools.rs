use crate::types::Drive;
use anyhow::Result;
use byte_unit::Byte;
use log::debug;
use std::path::{Path, PathBuf};

/// TODO: make this configurable
const QUOTA: f64 = 0.95;

pub fn get_drive_capacity(drive: &Drive) -> Result<Byte> {
    let expanded = shellexpand::tilde(&drive.mountpoint);
    let drive_path = PathBuf::from(expanded.into_owned());

    let drive_space: Result<Byte> = match Path::exists(&drive_path) {
        true => {
            debug!("loading real size for drive \"{}\"", &drive.name);
            Ok(Byte::from_bytes(fs2::available_space(&drive_path)? as u128))
        }
        false => {
            debug!("using estimated size for drive \"{}\"", &drive.name);
            let location_bytes = Byte::from_str(&drive.size)?;

            Ok(location_bytes)
        }
    };

    let drive_space = apply_quota(drive_space?, QUOTA);

    Ok(drive_space)
}

fn apply_quota(bytes: Byte, quota: f64) -> Byte {
    assert!(0.0 <= quota && quota <= 1.0);

    let bytes = bytes.get_bytes() as f64;
    let bytes = bytes * quota;

    Byte::from_bytes(bytes as u128)
}
