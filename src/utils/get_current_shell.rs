use crate::utils::macros::error;
use anyhow::{Context, Result};
use std::env;

pub fn get_current_shell() -> Result<String> {
    let mut shell = env::var("SHELL").context(error!("Failed getting the current shell."))?;
    if shell.is_empty() {
        shell = "/bin/sh".to_string();
    }
    Ok(shell)
}
