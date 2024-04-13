use crate::{
    modules::{
        backup::backup, cache::cache, check::check, delete::delete, initialize::initialize,
        repair::repair, restore::restore, snapshots::snapshots,
    },
    utils::{
        get_config::get_config,
        set_environment_variables::set_environment_variables,
        tools::{clear, select, select_title},
    },
};
use anyhow::Result;
use color_print::{cformat, cprintln};
use std::process::exit;

fn handle_selection(selection: &str) -> Result<()> {
    match selection {
        "Backup" => backup(false),
        "Restore" => restore(false),
        "Snapshots" => snapshots(false),
        "Delete" => delete(false),
        "Initialize" => initialize(false),
        "Check" => check(false),
        "Repair" => handle_repair(),
        "Cache" => cache(false),
        "Exit" => {
            exit(0);
        }
        _ => exit(0),
    }
}

fn handle_repair() -> Result<()> {
    clear()?;
    cprintln!("<c,u,s>REPAIR");
    println!();

    let settings = get_config()?;

    let selection = if settings.len() > 1 {
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_string()).collect();
        select("Where do you want to perform a repair?", selections)
    } else {
        0
    };

    set_environment_variables(&settings[selection])?;

    let backend = &settings[selection].backend;
    let repository = &settings[selection].repository;

    repair(backend, repository, false)
}

pub fn selector() -> Result<()> {
    clear()?;
    let exit_str = cformat!("<r>Exit");
    let selections = vec![
        "Backup".to_string(),
        "Restore".to_string(),
        "Snapshots".to_string(),
        "Delete".to_string(),
        "Initialize".to_string(),
        "Check".to_string(),
        "Repair".to_string(),
        "Cache".to_string(),
        exit_str,
    ];
    let selection = select_title("WRESTIC", selections.clone());

    handle_selection(&selections[selection])
}
