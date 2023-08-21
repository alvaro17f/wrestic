use std::{env, time::Duration};

use crate::{
    modules::selector::selector,
    utils::{
        get_config::Settings,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;
use std::process::Command;

fn get_snapshots(bucket: &str, repository: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Loading snapshots...");

    let out = Command::new("restic")
        .arg("-r")
        .arg(format!("b2:{}:{}", &bucket, &repository))
        .arg("--verbose")
        .arg("--verbose")
        .arg("snapshots")
        .output()?;

    pb.finish_and_clear();

    println!("{}", String::from_utf8(out.stdout)?);
    Ok(())
}

pub fn snapshots(settings: &Vec<Settings>, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>SNAPSHOTS");
    println!();

    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.clone()).collect();
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Where do you want to list snapshots from?"))
            .default(0)
            .max_length(10)
            .items(&selections[..])
            .interact()?
    } else {
        0
    };

    env::set_var("USER", &settings[selection].user);
    env::set_var("B2_ACCOUNT_ID", &settings[selection].account_id);
    env::set_var("RESTIC_PASSWORD", &settings[selection].restic_password);
    env::set_var("B2_ACCOUNT_ID", &settings[selection].account_id);
    env::set_var("B2_ACCOUNT_KEY", &settings[selection].account_key);

    let bucket = &settings[selection].bucket;
    let repository = &settings[selection].repository;

    get_snapshots(bucket, repository)?;
    pause()?;

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
