mod macros;
mod modules;
mod utils;

use anyhow::{Ok, Result};
use clap::{Parser, ValueEnum};
use modules::{backup::backup, selector::selector, snapshots::snapshots};
use utils::{get_env::dotenv, root_checker::is_root};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// List of available commands
    #[arg(value_enum)]
    command: Option<Command>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Command {
    /// Make a backup
    Backup,
    /// List all snapshots
    Snapshots,
}

fn main() -> Result<()> {
    let env = dotenv()?;

    let cli = Cli::parse();
    match cli.command {
        Some(Command::Backup) => {
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
            snapshots(&env.bucket, &env.repository, true);
        }
        None => {
            is_root()?;
            selector()?;
        }
    }

    Ok(())
}
