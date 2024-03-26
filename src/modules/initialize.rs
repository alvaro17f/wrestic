use crate::{
    modules::selector::selector,
    utils::{
        get_config::get_config,
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        tools::{clear, confirm, pause},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::cprintln;
use indicatif::ProgressBar;
use std::time::Duration;

fn initialize_repository(backend: &str, repository: &str) -> Result<()> {
    root_checker()?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Initializing repositories...");

    if run_cmd!(
        sudo -E restic -r $backend:$repository init 2>/dev/null;
    )
    .is_err()
    {
        pb.finish_and_clear();
        cprintln!("\n<g>Repository: <c>{repository}</c> already exists</g>\n");
    }

    pb.finish_and_clear();

    Ok(())
}

pub fn initialize(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>INITIALIZE REPOSITORIES");
    println!();

    let settings = get_config()?;

    if confirm("Do you want to initialize all repositories? (Y/n): ", true) {
        for conf in settings {
            set_environment_variables(&conf)?;

            let backend = &conf.backend;
            let repository = &conf.repository;

            initialize_repository(backend, repository)?;
        }
        pause()?;
    }

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
