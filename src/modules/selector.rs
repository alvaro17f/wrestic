use crate::{
    modules::{
        backup::backup, cache::cache, check::check, new_repository::new_repository, repair::repair,
        restore::restore, snapshots::snapshots,
    },
    utils::{get_env::dotenv, tools::clear},
};
use anyhow::Result;
use color_print::cformat;
use dialoguer::{theme::ColorfulTheme, Select};
use std::process::exit;

pub fn selector() -> Result<()> {
    clear()?;
    let env = dotenv()?;
    let exit_str = cformat!("<r>Exit");
    let selections = &[
        "Backup",
        "Restore",
        "Snapshots",
        "Check",
        "Repair",
        "Cache",
        "New Repository",
        exit_str.as_str(),
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<g>WRESTIC"))
        .default(0)
        .items(&selections[..])
        .interact()?;

    match selections[selection] {
        "Backup" => {
            backup(
                &env.user,
                &env.bucket,
                &env.repository,
                &env.keep_last,
                &env.backup_folder,
                false,
            )?;
        }
        "Restore" => {
            restore(
                &env.user,
                &env.bucket,
                &env.repository,
                &env.restore_folder,
                false,
            )?;
        }
        "Snapshots" => {
            snapshots(&env.bucket, &env.repository, false)?;
        }
        "Check" => {
            check(&env.bucket, &env.repository, false)?;
        }
        "Repair" => {
            repair(&env.bucket, &env.repository, false)?;
        }
        "Cache" => {
            cache(false)?;
        }
        "New Repository" => {
            new_repository(&env.bucket, false)?;
        }
        "Exit" => {
            exit(0);
        }
        _ => exit(0),
    }

    Ok(())
}
