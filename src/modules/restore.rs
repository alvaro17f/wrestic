use crate::{
    modules::selector::selector,
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

fn do_restore(
    backend: &str,
    repository: &str,
    restore_folder: &str,
    restore_snapshot: &str,
    user: &str,
) -> Result<()> {
    root_checker()?;

    if run_cmd!(
        sudo -E restic -r $backend:$repository --verbose --verbose restore $restore_snapshot --target $restore_folder;
    )
    .is_err()
    {
        cprintln!("<r>Failed to restore snapshot: <c>{restore_snapshot}</c> into: <c>{restore_folder}</c></r>");
    }
    if run_cmd!(sudo -E chown -R $user:$user $restore_folder 2>/dev/null).is_err() {
        cprintln!("\n<r>Failed to change ownership of: <c>{restore_folder}</c></r>\n");
    }

    Ok(())
}

pub fn restore(noconfirm: bool) -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>RESTORE");
    println!();

    let settings = get_config()?;

    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_string()).collect();
        select("Where do you want to restore from?", selections)
    } else {
        0
    };

    let setting = &settings[selection];

    set_environment_variables(setting)?;

    let backend = &setting.backend;
    let repository = &setting.repository;
    let restore_folder = &setting.restore_folder;
    let restore_snapshot = snapshots_selector(backend, repository)?;
    let user = &setting.user;

    if confirm(
        &format!("Do you want to restore the snapshot with ID {restore_snapshot}? (Y/n): "),
        true,
    ) {
        do_restore(backend, repository, restore_folder, &restore_snapshot, user)?;
        pause()?;
    }
    if !noconfirm {
        selector()?;
    }
    Ok(())
}
