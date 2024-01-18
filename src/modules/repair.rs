use crate::{
    modules::selector::selector,
    utils::{root_checker::root_checker, tools::pause},
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

fn repair_repository(backend: &str, repository: &str) -> Result<()> {
    root_checker()?;

    if run_cmd!(
        sudo -E restic -r $backend:$repository unlock;
        sudo -E restic -r $backend:$repository rebuild-index;
        sudo -E restic -r $backend:$repository prune;
        sudo -E restic -r $backend:$repository check;
    )
    .is_err()
    {
        cprintln!("\n<r>Failed to repair\n");
    }

    Ok(())
}

pub fn repair(backend: &str, repository: &str, noconfirm: bool) -> Result<()> {
    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!(
                "<y>Do you want to repair your repository? (Y/n): "
            ))
            .default(true)
            .interact()?
    {
        repair_repository(backend, repository)?;

        if !noconfirm {
            pause()?;
            selector()?;
        }
    } else {
        selector()?;
    }
    Ok(())
}
