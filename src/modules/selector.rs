use crate::{
    modules::{
        backup::backup, cache::cache, check::check, delete::delete, initialize::initialize,
        repair::repair, restore::restore, snapshots::snapshots, update::update,
    },
    utils::{get_config::get_config, tools::clear},
};
use anyhow::Result;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
use std::{env, process::exit};

pub fn selector() -> Result<()> {
    clear()?;
    let exit_str = cformat!("<r>Exit");
    let selections = &[
        "Backup",
        "Restore",
        "Snapshots",
        "Delete",
        "Initialize",
        "Check",
        "Repair",
        "Cache",
        "Update",
        exit_str.as_str(),
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<g>WRESTIC"))
        .default(0)
        .max_length(10)
        .items(&selections[..])
        .interact()?;

    match selections[selection] {
        "Backup" => {
            backup(false)?;
        }
        "Restore" => {
            restore(false)?;
        }
        "Snapshots" => {
            snapshots(false)?;
        }
        "Delete" => {
            delete(false)?;
        }
        "Initialize" => {
            initialize(false)?;
        }
        "Check" => {
            check(false)?;
        }
        "Repair" => {
            let settings = get_config()?;
            clear()?;
            cprintln!("<c,u,s>REPAIR");
            println!();
            let selection = if settings.len() > 1 {
                let selections: Vec<String> = settings.iter().map(|x| x.name.to_owned()).collect();
                Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(cformat!("<y>Where do you want to perform a repair?"))
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

            repair(backend, repository, false)?;
        }
        "Cache" => {
            cache(false)?;
        }
        "Update" => {
            update(false)?;
        }
        "Exit" => {
            exit(0);
        }
        _ => exit(0),
    }

    Ok(())
}
