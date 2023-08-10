use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        snapshots_selector::snapshots_selector,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn forget(
    bucket: &str,
    repository: &str,
    delete_snapshots: Option<&[String]>,
    noconfirm: bool,
) -> Result<()> {
    clear()?;
    cprintln!("<g>DELETE");
    println!();
    let delete_snapshots = match delete_snapshots {
        Some(snapshots) => snapshots.join(" "),
        None => snapshots_selector(bucket, repository)?,
    };

    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to delete the snapshot with ID {delete_snapshots}? (Y/n): "))
            .default(true)
            .interact()?
    {
        if run_cmd!(

            restic -r b2:$bucket:$repository forget $delete_snapshots;
        )
        .is_err()
        {
            cprintln!("<r>Failed to delete snapshots! Let's try to repair:");
            repair(bucket, repository, true)?;
            if run_cmd!(
                restic -r b2:$bucket:$repository forget $delete_snapshots;
            )
            .is_err()
            {
                cprintln!("<r>Houston, we have a problem! Failed to delete the snapshot AGAIN.");
            }
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
