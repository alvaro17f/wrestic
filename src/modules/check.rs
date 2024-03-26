use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::get_config,
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        tools::{clear, confirm, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};

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
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Where do you want to check?"))
            .default(0)
            .max_length(10)
            .items(&selections[..])
            .interact()?
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
