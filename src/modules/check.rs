use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};

pub fn check(bucket: &str, repository: &str, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>CHECK");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to check if your repo is working fine? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r b2:$bucket:$repository check;
        )
        .is_err()
        {
            cprintln!("<r>Failed to check");
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
