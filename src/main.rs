mod modules;
mod utils;

use crate::utils::{completions::set_completions, tools::clear};
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
use modules::{
    backup::backup, cache::cache, check::check, forget::forget, initialize::initialize,
    repair::repair, restore::restore, selector::selector, snapshots::snapshots, update::update,
};
use std::{env, process::exit};
use utils::{
    completions::print_completions, get_config::get_config, restic_checker::restic_checker,
    root_checker::root_checker,
};

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // If provided, generate completions for given shell
    #[arg(long = "generate", value_enum)]
    generator: Option<Shell>,
    /// List of available commands
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
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
    if let Some(generator) = cli.generator {
        let mut cmd = Cli::command();
        if generator == Shell::Zsh || generator == Shell::Bash {
            set_completions(generator, &mut cmd);
            cprintln!("<c>{}</c> <y>completions are set", generator);
            exit(0)
        } else {
            print_completions(generator, &mut cmd);
            exit(0)
        }
    }

    match &cli.commands {
        Some(Commands::Backup) => {
            backup(&settings, true)?;
        }
        Some(Commands::Restore) => {
            restore(&settings, true)?;
        }
        Some(Commands::Snapshots) => {
            snapshots(&settings, true)?;
        }
        Some(Commands::Forget) => {
            forget(&settings, true)?;
        }
        Some(Commands::Init) => {
            initialize(&settings, true)?;
        }
        Some(Commands::Check) => {
            check(&settings, true)?;
        }
        Some(Commands::Repair) => {
            clear()?;
            cprintln!("<c,u,s>REPAIR");
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
            env::set_var("RESTIC_PASSWORD", &settings[selection].restic_password);
            for env in &settings[selection].env {
                for (key, value) in env {
                    env::set_var(key, value);
                }
            }

            let backend = &settings[selection].backend;
            let bucket = &settings[selection].bucket;
            let repository = &settings[selection].repository;

            repair(backend, bucket, repository, true)?;
        }
        Some(Commands::Cache) => {
            cache(true)?;
        }
        Some(Commands::Update) => {
            update(true)?;
        }
        None => {
            selector()?;
        }
    }

    Ok(())
}
