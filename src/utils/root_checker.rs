use crate::utils::macros::error;
use anyhow::Result;
use nix::unistd::geteuid;

pub fn root_checker() -> Result<()> {
    if geteuid().is_root() {
        Ok(())
    } else {
        Err(error!("Please run as root"))?
    }
}
