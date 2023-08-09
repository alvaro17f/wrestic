use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn restore(
    bucket: &str,
    repository: &str,
    restore_folder: &str,
    noconfirm: bool,
) -> Result<()> {
    clear()?;
    cprintln!("<g>RESTORE");
    println!();
    cprintln!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to restore your latest snapshot? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r b2:$bucket:$repository restore latest --target $restore_folder
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
