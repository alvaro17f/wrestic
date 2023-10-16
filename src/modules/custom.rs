use std::{env, process::Command, time::Duration};

use crate::utils::{get_config::get_config, macros::error, tools::clear};
use anyhow::{Context, Result};
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
use indicatif::ProgressBar;

pub fn custom(args: &Vec<String>) -> Result<()> {
    let settings = get_config()?;
    clear()?;
    cprintln!("<c,u,s>CUSTOM");
    println!();
    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_owned()).collect();
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Where do you want to work?"))
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

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Loading custom command...");

    let out = Command::new("restic")
        .arg("-r")
        .arg(format!("{}:{}", &backend, &repository))
        .args(args)
        .output()
        .context(error!("Failed to run custom command!"))?;

    pb.finish_and_clear();

    println!("{}", String::from_utf8(out.stdout)?);

    Ok(())
}
