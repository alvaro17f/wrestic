use crate::{
    modules::selector::selector,
    utils::tools::{clear, pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn repair(bucket: &str, repository: &str, noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<g>REPAIR");
    println!();
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to repair your repository? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r b2:$bucket:$repository unlock;
            restic -r b2:$bucket:$repository rebuild-index;
            restic -r b2:$bucket:$repository prune;
            restic -r b2:$bucket:$repository check;
        )
        .is_err()
        {
            cprintln!("<r>Failed to repair");
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
