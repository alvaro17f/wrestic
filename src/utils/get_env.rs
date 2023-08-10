use crate::macros::anyhow::error;
use anyhow::{Context, Ok, Result};
use std::path::PathBuf;
use std::{env, fs};

pub struct Env {
    pub backup_folder: String,
    pub bucket: String,
    pub keep_last: String,
    pub repository: String,
    pub restore_folder: String,
    pub user: String,
}

pub fn dotenv() -> Result<Env> {
    fn find_env_file() -> Option<PathBuf> {
        let home_dir = PathBuf::from("/home/");
        let mut path = PathBuf::new();
        for entry in fs::read_dir(home_dir).ok()? {
            let entry = entry.ok()?;
            let mut env_path = entry.path();
            env_path.push(".config/wrestic/.env");
            if env_path.exists() {
                path = env_path;
                break;
            }
        }
        if path.exists() {
            Some(path)
        } else {
            None
        }
    }

    let env_file = find_env_file().ok_or_else(|| error!("Failed to find .env file"))?;
    let read_dotenv = fs::read_to_string(env_file).context(error!("Failed to read .env file"))?;

    let dotenv: Vec<(&str, &str)> = read_dotenv
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let mut parts = line.splitn(2, '=');
            let key = parts.next().unwrap();
            let value = parts.next().unwrap().trim_end_matches('\n');
            (key, value)
        })
        .collect();

    for (key, value) in &dotenv {
        env::set_var(key, value);
    }

    let user = env::var("USER").context(error!("please, add $USER to .env file"))?;
    if user.contains("root") {
        Err(error!("please, add $USER to .env file"))?;
    }
    let bucket = env::var("BUCKET").context(error!("please, add $BUCKET to .env file"))?;
    let repository =
        env::var("REPOSITORY").context(error!("please, add $REPOSITORY to .env file"))?;
    let keep_last =
        env::var("KEEP_LAST").context(error!("please, add $KEEP_LAST to .env file"))?;
    let backup_folder =
        env::var("BACKUP_FOLDER").context(error!("please, add $BACKUP_FOLDER to .env file"))?;
    let restore_folder =
        env::var("RESTORE_FOLDER").context(error!("please, add $RESTORE_FOLDER to .env file"))?;

    let env = Env {
        backup_folder,
        bucket,
        keep_last,
        repository,
        restore_folder,
        user,
    };

    Ok(env)
}
