mod macros;
mod modules;
mod utils;

use crate::utils::tools::clear;
use anyhow::Result;
use clap::{Parser, Subcommand};
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
use modules::{
    backup::backup, cache::cache, check::check, forget::forget, initialize::initialize,
    repair::repair, restore::restore, selector::selector, snapshots::snapshots, update::update,
};
use std::env;
use utils::{get_config::get_config, restic_checker::restic_checker, root_checker::root_checker};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of available commands
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Make a backup of all your repositories
    Backup,
    /// Restore a snapshot
    Restore,
    /// List snapshots
    Snapshots,
    /// Delete a snapshot
    Forget,
    /// Initialize all of your repositories
    Init,
    /// Check repository health
    Check,
    /// Fix any issue
    Repair,
    /// Clean cache
    Cache,
    /// Update Wrestic
    Update,
}

fn main() -> Result<()> {
    restic_checker()?;
    root_checker()?;

    let settings = get_config()?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Command::Backup) => {
            backup(&settings, true)?;
        }
        Some(Command::Restore) => {
            restore(&settings, true)?;
        }
        Some(Command::Snapshots) => {
            snapshots(&settings, true)?;
        }
        Some(Command::Forget) => {
            forget(&settings, true)?;
        }
        Some(Command::Init) => {
            initialize(&settings, true)?;
        }
        Some(Command::Check) => {
            check(&settings, true)?;
        }
        Some(Command::Repair) => {
            clear()?;
            cprintln!("<G>REPAIR");
            println!();
            let selection = if settings.len() > 1 {
                let selections: Vec<String> = settings.iter().map(|x| x.name.clone()).collect();
                Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(cformat!("<y>Where do you want to perform a repair?"))
                    .default(0)
                    .max_length(10)
                    .items(&selections[..])
                    .interact()?
            } else {
                0
            };

            env::set_var("USER", &settings[selection].user);
            env::set_var("B2_ACCOUNT_ID", &settings[selection].account_id);
            env::set_var("RESTIC_PASSWORD", &settings[selection].restic_password);
            env::set_var("B2_ACCOUNT_ID", &settings[selection].account_id);
            env::set_var("B2_ACCOUNT_KEY", &settings[selection].account_key);

            let bucket = &settings[selection].bucket;
            let repository = &settings[selection].repository;
            repair(bucket, repository, true)?;
        }
        Some(Command::Cache) => {
            cache(true)?;
        }
        Some(Command::Update) => {
            update(true)?;
        }
        None => {
            selector()?;
        }
    }

    Ok(())
}
