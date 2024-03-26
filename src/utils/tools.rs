use anyhow::Result;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};
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

pub fn confirm(prompt: &str, default_value: bool) -> bool {
    Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<y>{}</y>", prompt))
        .default(default_value)
        .interact()
        .unwrap_or(false)
}
