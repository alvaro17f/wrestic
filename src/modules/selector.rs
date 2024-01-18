use crate::{
    modules::{
        backup::backup, cache::cache, check::check, delete::delete, initialize::initialize,
        repair::repair, restore::restore, snapshots::snapshots,
    },
    utils::{
        get_config::get_config, set_environment_variables::set_environment_variables, tools::clear,
    },
};
use anyhow::Result;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
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
        let selections: Vec<String> = settings.iter().map(|x| x.name.to_owned()).collect();
        Select::with_theme(&ColorfulTheme::default())
            .with_prompt(cformat!("<y>Where do you want to perform a repair?"))
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

    repair(backend, repository, false)
}

pub fn selector() -> Result<()> {
    clear()?;
    let exit_str = cformat!("<r>Exit");
    let selections = &[
        "Backup",
        "Restore",
        "Snapshots",
        "Delete",
        "Initialize",
        "Check",
        "Repair",
        "Cache",
        exit_str.as_str(),
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(cformat!("<g>WRESTIC"))
        .default(0)
        .max_length(10)
        .items(&selections[..])
        .interact()?;

    handle_selection(selections[selection])
}
