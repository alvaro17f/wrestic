mod macros;
mod modules;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use modules::{
    backup::backup, cache::cache, check::check, forget::forget, new_repository::new_repository,
    repair::repair, restore::restore, selector::selector, snapshots::snapshots,
};
use utils::{get_env::dotenv, restic_checker::restic_checker, root_checker::root_checker};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of available commands
    #[command(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Make a backup
    Backup,
    /// Restore a snapshot
    Restore,
    /// List all snapshots
    Snapshots,
    /// Check repository health
    Check,
    /// Fix any issue
    Repair,
    /// Clean cache
    Cache,
    /// Delete snapshots
    Forget { delete_snapshots: Vec<String> },
    /// Create a new repository
    New { name: String },
}

fn main() -> Result<()> {
    restic_checker()?;
    root_checker()?;

    let env = dotenv()?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Command::Backup) => {
            backup(
                &env.bucket,
                &env.repository,
                &env.keep_last,
                &env.backup_folder,
                true,
            )?;
        }
        Some(Command::Restore) => {
            restore(&env.bucket, &env.repository, &env.restore_folder, true)?;
        }
        Some(Command::Snapshots) => {
            snapshots(&env.bucket, &env.repository, true)?;
        }
        Some(Command::Check) => {
            check(&env.bucket, &env.repository, true)?;
        }
        Some(Command::Repair) => {
            repair(&env.bucket, &env.repository, true)?;
        }
        Some(Command::Cache) => {
            cache(true)?;
        }
        Some(Command::Forget { delete_snapshots }) => {
            forget(&env.bucket, &env.repository, Some(delete_snapshots), true)?;
        }
        Some(Command::New { name }) => {
            new_repository(&env.bucket, Some(name), true)?;
        }
        None => {
            selector()?;
        }
    }

    Ok(())
}
