use anyhow::Result;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
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

pub fn select(prompt: &str, selections: Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<y>{}</y>", prompt))
        .default(0)
        .max_length(10)
        .items(&selections[..])
        .interact()
        .unwrap()
}

pub fn select_title(prompt: &str, selections: Vec<String>) -> usize {
    Select::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<g>{}</g>", prompt))
        .default(0)
        .max_length(10)
        .items(&selections[..])
        .interact()
        .unwrap()
}
