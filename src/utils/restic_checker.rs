use crate::macros::anyhow::error;
use anyhow::Result;
use which::which;

pub fn restic_checker() -> Result<()> {
    match which("restic") {
        Ok(_) => Ok(()),
        Err(_) => Err(error!("Restic not found, please install it first")),
    }
}
