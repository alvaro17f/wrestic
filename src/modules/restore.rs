use crate::{
    modules::selector::selector,
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

fn do_restore(
    bucket: &str,
    repository: &str,
    restore_folder: &str,
    restore_snapshot: &str,
) -> Result<()> {
    if run_cmd!(
        restic -r b2:$bucket:$repository --verbose --verbose restore $restore_snapshot --target $restore_folder;
    )
    .is_err()
    {
        cprintln!("<r>Failed to restore snapshot: <c>{restore_snapshot}</c> into: <c>{restore_folder}</c></r>");
    }

    Ok(())
}

pub fn restore(settings: &Vec<Settings>, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<C>RESTORE");
    println!();
    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.clone()).collect();
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Where do you want to restore from?"))
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
    let restore_folder = &settings[selection].restore_folder;
    let restore_snapshot = snapshots_selector(bucket, repository)?;

    if Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!(
            "<y>Do you want to restore the snapshot with ID {restore_snapshot}? (Y/n): "
        ))
        .default(true)
        .interact()?
    {
        do_restore(bucket, repository, restore_folder, &restore_snapshot)?;
        pause()?;
    }
    if !noconfirm {
        selector()?;
    }
    Ok(())
}
