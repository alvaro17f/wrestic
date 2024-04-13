use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::get_config,
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        tools::{clear, confirm, pause, select},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::cprintln;

fn do_check(backend: &str, repository: &str) -> Result<()> {
    root_checker()?;

    if run_cmd!(
        sudo -E restic -r $backend:$repository check;
    )
    .is_err()
    {
        cprintln!("\n<r>Failed to check\n");

        if confirm("Do you want to repair? (Y/n): ", true) {
            repair(backend, repository, true)?;
            pause()?;
        }
    }
    Ok(())
}

pub fn check(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>CHECK");
    println!();

    let settings = get_config()?;

    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_string()).collect();
        select("Where do you want to check?", selections)
    } else {
        0
    };

    set_environment_variables(&settings[selection])?;

    let backend = &settings[selection].backend;
    let repository = &settings[selection].repository;

    do_check(backend, repository)?;
    pause()?;

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
