#![allow(clippy::collapsible_if)]

use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::Settings,
        snapshots_selector::snapshots_selector,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::env;

pub fn forget(settings: &Vec<Settings>, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<G>DELETE");
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
    let delete_snapshots = snapshots_selector(bucket, repository)?;

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!(
            "<y>Do you want to delete the snapshot with ID {delete_snapshots}? (Y/n): "
        ))
        .default(true)
        .interact()?
    {
        if run_cmd!(

            restic -r b2:$bucket:$repository forget $delete_snapshots;
        )
        .is_err()
        {
            cprintln!("<r>Failed to delete snapshots!");
            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(cformat!("<y>Do you want to repair? (Y/n):"))
                .default(true)
                .interact()?
            {
                repair(bucket, repository, true)?;
                if run_cmd!(
                    restic -r b2:$bucket:$repository forget $delete_snapshots;
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
