use crate::{
    modules::{repair::repair, selector::selector},
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn backup(
    bucket: &str,
    repository: &str,
    keep_last: &str,
    backup_folder: &str,
    noconfirm: bool,
) -> Result<()> {
    clear()?;
    cprintln!("<g>BACKUP");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to perform a backup? (Y/n): "))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r b2:$bucket:$repository --verbose --verbose backup $backup_folder;
        )
        .is_err()
        {
            cprintln!("<r>Failed to backup");
        }
        if run_cmd!(
            restic -r b2:$bucket:$repository --verbose --verbose forget --keep-last $keep_last;
        )
        .is_err()
        {
            cprintln!(
                "<r>Failed to forget keeping last {keep_last} snapshots, let's try to repair: "
            );
            repair(bucket, repository, true)?;

            if run_cmd!(
                restic -r b2:$bucket:$repository --verbose --verbose forget --keep-last $keep_last;
            )
            .is_err()
            {
                cprintln!(
                "<r>Houston, we have a problem! Failed to forget keeping last {keep_last} snapshots AGAIN."
            );
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
