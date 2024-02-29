mod modules;
mod utils;

use crate::utils::{set_environment_variables::set_environment_variables, tools::clear};
use anyhow::Result;
use clap::{CommandFactory, Parser, Subcommand};
use clap_complete::Shell;
use color_print::{cformat, cprintln};
use dialoguer::{theme::ColorfulTheme, Select};
use modules::{
    backup::backup, cache::cache, check::check, custom::custom, delete::delete,
    initialize::initialize, repair::repair, restore::restore, selector::selector,
    snapshots::snapshots, update::update,
};
use utils::{get_config::get_config, restic_checker::restic_checker};

#[derive(Parser, Debug, PartialEq)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of available commands
    #[command(subcommand)]
    commands: Option<Commands>,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Commands {
    /// Make a backup of all your repositories
    #[clap(short_flag = 'b')]
    Backup,
    /// Restore a snapshot
    #[clap(short_flag = 'r')]
    Restore,
    /// List snapshots
    #[clap(short_flag = 's')]
    Snapshots,
    /// Delete a snapshot
    #[clap(short_flag = 'd')]
    Delete,
    /// Initialize all of your repositories
    #[clap(short_flag = 'i')]
    Init,
    /// Check repository health
    Check,
    /// Fix any issue
    Repair,
    /// Clean cache
    Cache,
    /// Update Wrestic
    #[clap(short_flag = 'u')]
    Update,
    /// Custom command
    #[clap(short_flag = 'c', allow_hyphen_values = true)]
    #[command(arg_required_else_help = true)]
    Custom { args: Vec<String> },
    /// Generate tab-completion scripts for your shell
    Completions {
        #[clap(value_enum)]
        shell: Shell,
    },
}

fn handle_commands(cli: &Cli) -> Result<()> {
    match &cli.commands {
        Some(Commands::Backup) => {
            backup(true)?;
        }
        Some(Commands::Restore) => {
            restore(true)?;
        }
        Some(Commands::Snapshots) => {
            snapshots(true)?;
        }
        Some(Commands::Delete) => {
            delete(true)?;
        }
        Some(Commands::Init) => {
            initialize(true)?;
        }
        Some(Commands::Check) => {
            check(true)?;
        }
        Some(Commands::Repair) => {
            handle_repair()?;
        }
        Some(Commands::Cache) => {
            cache(true)?;
        }
        Some(Commands::Update) => {
            update()?;
        }
        Some(Commands::Custom { args }) => {
            custom(args)?;
        }
        Some(Commands::Completions { shell }) => {
            clap_complete::generate(
                *shell,
                &mut Cli::command(),
                "wrestic",
                &mut std::io::stdout().lock(),
            );
        }
        None => {
            selector()?;
        }
    }
    Ok(())
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

    repair(backend, repository, true)?;
    Ok(())
}

fn main() -> Result<()> {
    restic_checker()?;

    let cli = Cli::parse();
    handle_commands(&cli)?;

    Ok(())
}
