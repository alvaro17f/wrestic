use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn cache(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<G>CACHE");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to clean cache? (Y/n): "))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic cache --cleanup
        )
        .is_err()
        {
            cprintln!("<r>Failed to clean cache");
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
