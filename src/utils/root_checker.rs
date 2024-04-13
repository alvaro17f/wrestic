use crate::macros::errors::error;

use anyhow::Result;
use nix::unistd::geteuid;
use std::process::Command;

pub fn root_checker() -> Result<()> {
    if geteuid().is_root() {
        Ok(())
    } else {
        if Command::new("sudo").arg("-v").status().is_err() {
            Err(error!("You need to be root to run this command"))?;
        };
        Ok(())
    }
}
