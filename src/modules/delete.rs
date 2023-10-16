#![allow(clippy::collapsible_if)]

use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::get_config,
        snapshots_selector::snapshots_selector,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::env;

pub fn delete(noconfirm: bool) -> Result<()> {
    let settings = get_config()?;
    clear()?;
    cprintln!("<c,u,s>DELETE");
    println!();
    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_owned()).collect();
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>What snapshot do you want to delete?"))
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
    let delete_snapshots = snapshots_selector(backend, repository)?;

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!(
            "<y>Do you want to delete the snapshot with ID {delete_snapshots}? (Y/n): "
        ))
        .default(true)
        .interact()?
    {
        if run_cmd!(
            restic -r $backend:$repository forget $delete_snapshots;
        )
        .is_err()
        {
            cprintln!("<r>Failed to delete snapshots!");
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(cformat!("<y>Do you want to repair? (Y/n):"))
                .default(true)
                .interact()?
            {
                repair(backend, repository, true)?;
                if run_cmd!(
                    restic -r $backend:$repository forget $delete_snapshots;
                )
                .is_err()
                {
                    cprintln!(
                        "<r>Houston, we have a problem! Failed to delete the snapshot AGAIN."
                    );
                }
            }
            pause()?;
        }
    }

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
