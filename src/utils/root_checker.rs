use anyhow::Result;

use crate::macros::anyhow::error;

pub fn root_checker() -> Result<()> {
    if unsafe { libc::geteuid() } != 0 {
        Err(error!("Please run as root"))?;
    }
    Ok(())
}
