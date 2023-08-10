use crate::macros::anyhow::error;
use anyhow::Result;
use which::which;

pub fn restic_checker() -> Result<()> {
    // Check if Restic is in the PATH
    match which("restic") {
        Ok(_) => Ok(()),
        Err(_) => {
            return Err(error!("Restic not found, please install it first"));
        }
    }
}
