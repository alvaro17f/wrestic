use crate::{
    modules::selector::selector,
    utils::{
        root_checker::root_checker,
        tools::{clear, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Confirm};

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

    if noconfirm
        || Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Do you want to clean cache? (Y/n): "))
            .default(true)
            .interact()?
    {
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
