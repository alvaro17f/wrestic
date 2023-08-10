use crate::{
    modules::selector::selector,
    utils::{
        snapshots_selector::snapshots_selector,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn restore(
    bucket: &str,
    repository: &str,
    restore_folder: &str,
    restore_snapshot: Option<String>,
    noconfirm: bool,
) -> Result<()> {
    clear()?;
    cprintln!("<g>RESTORE");
    println!();
    let restore_snapshot = match restore_snapshot {
        Some(snapshot) => snapshot,
        None => snapshots_selector(bucket, repository)?,
    };
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to restore the snapshot with ID {restore_snapshot}? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r b2:$bucket:$repository restore $restore_snapshot --target $restore_folder
        )
        .is_err()
        {
            cprintln!("<r>Failed to restore");
        }
        if !noconfirm {
            pause()?;
            selector()?;
        }
    } else {
        selector()?;
    }
    Ok(())
}
