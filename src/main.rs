mod macros;
mod modules;
mod utils;

use anyhow::Result;
use clap::{Parser, Subcommand};
use modules::{
    backup::backup, cache::cache, check::check, forget::forget, new_repository::new_repository,
    repair::repair, restore::restore, selector::selector, snapshots::snapshots,
};
use utils::{get_env::dotenv, root_checker::is_root};

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
    /// Create a new repository
    New,
    /// Delete snapshots
    Forget { delete_snapshots: Vec<String> },
}

fn main() -> Result<()> {
    let env = dotenv()?;

    let cli = Cli::parse();
    match &cli.command {
        Some(Command::Backup) => {
            is_root()?;
            backup(
                &env.user,
                &env.bucket,
                &env.repository,
                &env.keep_last,
                &env.backup_folder,
                true,
            )?;
        }
        Some(Command::Restore) => {
            is_root()?;
            restore(
                &env.user,
                &env.bucket,
                &env.repository,
                &env.restore_folder,
                true,
            )?;
        }
        Some(Command::Snapshots) => {
            is_root()?;
            snapshots(&env.bucket, &env.repository, true)?;
        }
        Some(Command::Check) => {
            is_root()?;
            check(&env.bucket, &env.repository, true)?;
        }
        Some(Command::Repair) => {
            is_root()?;
            repair(&env.bucket, &env.repository, true)?;
        }
        Some(Command::Cache) => {
            is_root()?;
            cache(true)?;
        }
        Some(Command::Forget { delete_snapshots }) => {
            is_root()?;
            forget(&env.bucket, &env.repository, Some(delete_snapshots), true)?;
        }
        Some(Command::New) => {
            is_root()?;
            new_repository(&env.bucket, true)?;
        }
        None => {
            is_root()?;
            selector()?;
        }
    }

    Ok(())
}
