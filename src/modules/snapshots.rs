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

fn get_snapshots(backend: &str, repository: &str) -> Result<()> {
    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Loading snapshots...");

    let out = Command::new("restic")
        .arg("-r")
        .arg(format!("{}:{}", &backend, &repository))
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
    cprintln!("<c,u,s>SNAPSHOTS");
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
    env::set_var("RESTIC_PASSWORD", &settings[selection].restic_password);
    for env in &settings[selection].env {
        for (key, value) in env {
            env::set_var(key, value);
        }
    }

    let backend = &settings[selection].backend;
    let repository = &settings[selection].repository;

    get_snapshots(backend, repository)?;
    pause()?;

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
