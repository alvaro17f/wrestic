use crate::{
    modules::selector::selector,
    utils::{
        root_checker::root_checker,
        tools::{confirm, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::cprintln;

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
    if noconfirm || confirm("Do you want to repair your repository? (Y/n): ", true) {
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
