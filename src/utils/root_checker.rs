use anyhow::{Ok, Result};

use crate::macros::anyhow::uerror;

pub fn is_root() -> Result<()> {
    if unsafe { libc::geteuid() } != 0 {
        Err(uerror!("Please run as root"))?;
    }
    Ok(())
}
