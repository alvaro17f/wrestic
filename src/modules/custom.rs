use std::{process::Command, time::Duration};

use crate::{
    macros::errors::error,
    utils::{
        get_config::get_config,
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        tools::{clear, select},
    },
};
use anyhow::{Context, Result};
use color_print::cprintln;
use indicatif::ProgressBar;

fn run_custom_command(backend: &str, repository: &str, args: &Vec<String>) -> Result<()> {
    root_checker()?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Loading custom command...");

    let out = Command::new("sudo")
        .arg("-E")
        .arg("restic")
        .arg("-r")
        .arg(format!("{}:{}", &backend, &repository))
        .args(args)
        .output()
        .context(error!("Failed to run custom command!"))?;

    pb.finish_and_clear();

    println!("{}", String::from_utf8(out.stdout)?);

    Ok(())
}

pub fn custom(args: &Vec<String>) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>CUSTOM");
    println!();

    let settings = get_config()?;

    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_string()).collect();
        select("Where do you want to work?", selections)
    } else {
        0
    };

    set_environment_variables(&settings[selection])?;

    let backend = &settings[selection].backend;
    let repository = &settings[selection].repository;

    run_custom_command(backend, repository, args)?;

    Ok(())
}
