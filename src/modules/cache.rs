use crate::{
    modules::selector::selector,
    utils::{
        root_checker::root_checker,
        tools::{clear, confirm, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::cprintln;

fn clean_cache() -> Result<()> {
    root_checker()?;

    if run_cmd!(
        sudo -E restic cache --cleanup
    )
    .is_err()
    {
        cprintln!("\n<r>Failed to clean cache\n");
    }

    Ok(())
}

pub fn cache(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>CACHE");
    println!();

    if noconfirm || confirm("Do you want to clean cache? (Y/n): ", true) {
        clean_cache()?;

        if !noconfirm {
            pause()?;
            selector()?;
        }
    } else {
        selector()?;
    }

    Ok(())
}
