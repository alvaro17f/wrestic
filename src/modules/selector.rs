use crate::{
    modules::{
        backup::backup, cache::cache, check::check, forget::forget, initialize::initialize,
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
    let settings = get_config()?;
    let exit_str = cformat!("<r>Exit");
    let selections = &[
        "Backup",
        "Restore",
        "Snapshots",
        "Forget",
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
            backup(&settings, false)?;
        }
        "Restore" => {
            restore(&settings, false)?;
        }
        "Snapshots" => {
            snapshots(&settings, false)?;
        }
        "Forget" => {
            forget(&settings, false)?;
        }
        "Initialize" => {
            initialize(&settings, false)?;
        }
        "Check" => {
            check(&settings, false)?;
        }
        "Repair" => {
            clear()?;
            cprintln!("<C>REPAIR");
            println!();
            let selection = if settings.len() > 1 {
                let selections: Vec<String> = settings.iter().map(|x| x.name.clone()).collect();
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
            env::set_var("B2_ACCOUNT_ID", &settings[selection].account_id);
            env::set_var("RESTIC_PASSWORD", &settings[selection].restic_password);
            env::set_var("B2_ACCOUNT_ID", &settings[selection].account_id);
            env::set_var("B2_ACCOUNT_KEY", &settings[selection].account_key);

            let bucket = &settings[selection].bucket;
            let repository = &settings[selection].repository;

            repair(bucket, repository, false)?;
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
