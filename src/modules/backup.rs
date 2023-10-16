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

fn do_backup(backend: &str, repository: &str, backup_folder: &str, keep_last: &str) -> Result<()> {
    if run_cmd!(
        restic -r $backend:$repository --verbose --verbose backup $backup_folder;
    )
    .is_err()
    {
        cprintln!("<r>Failed to backup");
    }

    if run_cmd!(
        restic -r $backend:$repository --verbose --verbose forget --keep-last $keep_last;
    )
    .is_err()
    {
        cprintln!("<r>Failed to delete keeping last {keep_last} snapshots.");
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to repair? (Y/n):"))
            .default(true)
            .interact()?
        {
            repair(backend, repository, true)?;

            if run_cmd!(
                restic -r $backend:$repository --verbose --verbose forget --keep-last $keep_last;
            )
            .is_err()
            {
                cprintln!(
                    "<r>Houston, we have a problem! Failed to delete keeping last {keep_last} snapshots AGAIN."
            )
            }
        }
    }

    Ok(())
}

pub fn backup(noconfirm: bool) -> Result<()> {
    let settings = get_config()?;
    clear()?;
    cprintln!("<c,u,s>BACKUP");
    println!();

    if noconfirm {
        for conf in settings {
            let backend = &conf.backend;
            let repository = &conf.repository;
            let keep_last = &conf.keep_last;
            let backup_folder = &conf.backup_folder;

            env::set_var("USER", &conf.user);
            env::set_var("RESTIC_PASSWORD", &conf.restic_password);
            for env in &conf.env {
                for (key, value) in env {
                    env::set_var(key, value);
                }
            }

            do_backup(backend, repository, backup_folder, keep_last)?;
        }
    } else {
        let selection = if settings.len() > 1 {
            let selections: Vec<String> = settings.iter().map(|x| x.name.to_owned()).collect();
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt(cformat!("<y>Where do you want to perform a backup?"))
                .default(0)
                .max_length(10)
                .items(&selections[..])
                .interact()?
        } else {
            0
        };

        let name = &settings[selection].name;
        let backend = &settings[selection].backend;
        let repository = &settings[selection].repository;
        let keep_last = &settings[selection].keep_last;
        let backup_folder = &settings[selection].backup_folder;

        env::set_var("USER", &settings[selection].user);
        env::set_var("RESTIC_PASSWORD", &settings[selection].restic_password);
        for env in &settings[selection].env {
            for (key, value) in env {
                env::set_var(key, value);
            }
        }
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to perform a backup for {name}? (Y/n): "
            ))
            .default(true)
            .interact()?
        {
            do_backup(backend, repository, backup_folder, keep_last)?;
            pause()?;
        }
        if !noconfirm {
            selector()?;
        }
    }

    Ok(())
}
