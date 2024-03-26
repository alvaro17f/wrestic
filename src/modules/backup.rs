use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::{get_config, Settings},
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        tools::{clear, confirm, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};

fn do_backup(setting: &Settings) -> Result<()> {
    root_checker()?;

    let backend = &setting.backend;
    let repository = &setting.repository;
    let backup_folder = &setting.backup_folder;
    let keep_last = &setting.keep_last;

    if run_cmd!(
        sudo -E restic -r $backend:$repository --verbose --verbose backup $backup_folder 2>/dev/null;
    )
    .is_err()
    {
        cprintln!("\n<r>Failed to backup\n");
    }

    if run_cmd!(
        sudo -E restic -r $backend:$repository --verbose --verbose forget --keep-last $keep_last 2>/dev/null;
    )
    .is_err()
    {
        cprintln!("\n<r>Failed to delete old snapshots keeping last {keep_last}\n");

        if confirm("Do you want to repair? (Y/n): ", true)
        {
            repair(backend, repository, true)?;

            if run_cmd!(
                sudo -E restic -r $backend:$repository --verbose --verbose forget --keep-last $keep_last 2>/dev/null;
            )
            .is_err()
            {
                cprintln!(
                    "\n<r>Houston, we have a problem! Failed to delete old snapshots keeping last {keep_last} AGAIN\n"
                )
            }
        }
    }

    Ok(())
}

pub fn backup(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>BACKUP");
    println!();

    let settings = get_config()?;

    if noconfirm {
        for setting in settings {
            set_environment_variables(&setting)?;
            do_backup(&setting)?;
        }
    } else {
        let selection = if settings.len() > 1 {
            let selections: Vec<String> = settings.iter().map(|x| x.name.to_string()).collect();
            Select::with_theme(&ColorfulTheme::default())
                .with_prompt(cformat!("<y>Where do you want to perform a backup?"))
                .default(0)
                .max_length(10)
                .items(&selections[..])
                .interact()?
        } else {
            0
        };

        let setting = &settings[selection];

        set_environment_variables(setting)?;

        if confirm(
            &format!(
                "Do you want to perform a backup for {}? (Y/n): ",
                setting.name
            ),
            true,
        ) {
            do_backup(setting)?;
            pause()?;
        }
        if !noconfirm {
            selector()?;
        }
    }

    Ok(())
}
