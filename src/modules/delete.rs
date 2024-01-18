use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::get_config,
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        snapshots_selector::snapshots_selector,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

fn delete_snapshot(backend: &str, repository: &str, delete_snapshots: &str) -> Result<()> {
    root_checker()?;

    if run_cmd!(
        sudo -E restic -r $backend:$repository forget $delete_snapshots;
    )
    .is_err()
    {
        cprintln!("\n<r>Failed to delete snapshot!\n");

        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to repair? (Y/n):"))
            .default(true)
            .interact()?
        {
            repair(backend, repository, true)?;
            if run_cmd!(
                sudo -E restic -r $backend:$repository forget $delete_snapshots;
            )
            .is_err()
            {
                cprintln!("\n<r>Houston, we have a problem! Failed to delete the snapshot AGAIN\n");
            }
        }
        pause()?;
    }

    Ok(())
}

pub fn delete(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>DELETE");
    println!();

    let settings = get_config()?;

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

    set_environment_variables(&settings[selection])?;

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
        delete_snapshot(backend, repository, &delete_snapshots)?;
    }

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
