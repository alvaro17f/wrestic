mod macros;
mod modules;
mod utils;

use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use modules::{
    backup::backup, forget::forget, repair::repair, selector::selector, snapshots::snapshots,
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
    /// List all snapshots
    Snapshots,
    /// Fix any issue
    Repair,
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
            );
        }
        Some(Command::Snapshots) => {
            is_root()?;
            snapshots(&env.bucket, &env.repository, true);
        }
        Some(Command::Repair) => {
            is_root()?;
            repair(&env.bucket, &env.repository, true);
        }
        Some(Command::Forget { delete_snapshots }) => {
            is_root()?;
            forget(&env.bucket, &env.repository, delete_snapshots, true);
        }
        None => {
            is_root()?;
            selector()?;
        }
    }

    Ok(())
}
