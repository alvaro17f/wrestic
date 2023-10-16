use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::get_config,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::env;

fn do_check(backend: &str, repository: &str) -> Result<()> {
    if run_cmd!(
        restic -r $backend:$repository check;
    )
    .is_err()
    {
        cprintln!("<r>Failed to check.");
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to repair? (Y/n):"))
            .default(true)
            .interact()?
        {
            repair(backend, repository, true)?;
            pause()?;
        }
    }
    Ok(())
}

pub fn check(noconfirm: bool) -> Result<()> {
    let settings = get_config()?;
    clear()?;
    cprintln!("<c,u,s>CHECK");
    println!();
    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_owned()).collect();
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Where do you want to check?"))
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

    do_check(backend, repository)?;
    pause()?;

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
