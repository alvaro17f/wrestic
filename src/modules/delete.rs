use crate::{
    modules::{repair::repair, selector::selector},
    utils::{
        get_config::get_config,
        root_checker::root_checker,
        set_environment_variables::set_environment_variables,
        snapshots_selector::snapshots_selector,
        tools::{clear, confirm, pause, select},
    },
};
use anyhow::Result;
use cmd_lib::run_cmd;
use color_print::cprintln;

fn delete_snapshot(backend: &str, repository: &str, delete_snapshots: &str) -> Result<()> {
    root_checker()?;

    if run_cmd!(
        sudo -E restic -r $backend:$repository forget $delete_snapshots;
    )
    .is_err()
    {
        cprintln!("\n<r>Failed to delete snapshot!\n");

        if confirm("Do you want to repair? (Y/n): ", true) {
            repair(backend, repository, true)?;
            if run_cmd!(
                sudo -E restic -r $backend:$repository forget $delete_snapshots;
            )
            .is_err()
            {
                cprintln!("\n<r>Houston, we have a problem! Failed to delete the snapshot AGAIN\n");
            }
        }
        pause()?;
    }

    Ok(())
}

pub fn delete(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>DELETE");
    println!();

    let settings = get_config()?;

    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_string()).collect();
        select("What snapshot do you want to delete?", selections)
    } else {
        0
    };

    set_environment_variables(&settings[selection])?;

    let backend = &settings[selection].backend;
    let repository = &settings[selection].repository;
    let delete_snapshots = snapshots_selector(backend, repository)?;

    if confirm(
        &format!("Do you want to delete the snapshot with ID {delete_snapshots}? (Y/n): "),
        true,
    ) {
        delete_snapshot(backend, repository, &delete_snapshots)?;
    }

    if !noconfirm {
        selector()?;
    }
    Ok(())
}
