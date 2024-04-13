use crate::{
    modules::selector::selector,
    utils::{
        get_config::get_config,
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        tools::{clear, pause, select},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::cprintln;
use indicatif::ProgressBar;
use std::time::Duration;

fn get_snapshots(backend: &str, repository: &str) -> Result<()> {
    root_checker()?;

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(120));
    pb.set_message("Loading snapshots...");

    if run_cmd!(
        sudo -E restic -r $backend:$repository --verbose --verbose snapshots;
    )
    .is_err()
    {
        pb.finish_and_clear();
        cprintln!("<r>Failed to list snapshots");
    }

    pb.finish_and_clear();

    Ok(())
}

pub fn snapshots(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>SNAPSHOTS");
    println!();

    let settings = get_config()?;

    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_string()).collect();
        select("Where do you want to list snapshots from?", selections)
    } else {
        0
    };

    let setting = &settings[selection];

    set_environment_variables(setting)?;

    let backend = &setting.backend;
    let repository = &setting.repository;

    get_snapshots(backend, repository)?;
    pause()?;

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
