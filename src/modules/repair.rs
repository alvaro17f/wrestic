use crate::{modules::selector::selector, utils::tools::pause};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

pub fn repair(backend: &str, repository: &str, noconfirm: bool) -> Result<()> {
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to repair your repository? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        if run_cmd!(
            restic -r $backend:$repository unlock;
            restic -r $backend:$repository rebuild-index;
            restic -r $backend:$repository prune;
            restic -r $backend:$repository check;
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
