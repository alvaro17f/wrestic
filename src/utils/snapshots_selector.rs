use anyhow::Result;
use color_print::cformat;
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use regex::Regex;
use std::{process::Command, time::Duration};

use crate::utils::root_checker::root_checker;

fn get_snapshots(backend: &str, repository: &str) -> Result<String> {
    let restic = Command::new("sudo")
        .arg("-E")
        .arg("restic")
        .arg("-r")
        .arg(format!("{}:{}", backend, repository))
        .arg("--verbose")
        .arg("--verbose")
        .arg("snapshots")
        .output()?;

    let restic = String::from_utf8(restic.stdout)?;

    Ok(restic)
}

fn parse_snapshots(restic: &str) -> Result<Vec<String>> {
    let restic_rev = restic
        .lines()
        .rev()
        .collect::<Vec<&str>>()
        .to_owned()
        .join("\n");

    let selections = Regex::new(r"(\w+)\s+(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})")?
        .captures_iter(&restic_rev)
        .map(|cap| format!("[{}] - {}", &cap[1], &cap[2]))
        .collect::<Vec<String>>();

    Ok(selections)
}

pub fn snapshots_selector(backend: &str, repository: &str) -> Result<String> {
    root_checker()?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Loading snapshots...");

    let restic = get_snapshots(backend, repository)?;

    pb.finish_and_clear();

    let selections = parse_snapshots(&restic)?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<g>Snapshots:"))
        .default(0)
        .max_length(10)
        .items(&selections[..])
        .interact()?;

    let selection = Regex::new(r"(\w+)\s+(\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2})")?
        .captures_iter(&restic)
        .map(|cap| (cap[1]).to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .rev()
        .collect::<Vec<String>>()[selection]
        .to_owned();

    Ok(selection)
}
