use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn snapshots(bucket: &str, repository: &str, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>SNAPSHOTS");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to list your snapshots? (Y/n): "))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r b2:$bucket:$repository snapshots
        )
        .is_err()
        {
            cprintln!("<r>Failed to list snapshots");
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
