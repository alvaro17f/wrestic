use anyhow::Result;
use color_print::cprintln;
use std::{
    io::{self, BufRead},
    process::Command,
};

pub fn clear() -> Result<()> {
    Command::new("clear").status()?;
    Ok(())
}

pub fn pause() -> Result<()> {
    cprintln!("Press <c>'Enter'</c> to continue...");
    io::stdin().lock().lines().next();
    clear()?;
    Ok(())
}
